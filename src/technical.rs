use nota::{Block, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_persona::ComponentName;
use std::fmt;
use std::str::FromStr;

use crate::{
    ActorName, Error, MindResult, QueryLimit, TaskToken, TextBody, TimestampNanos, WirePath,
};

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TechnicalNodeIdentifier(String);

impl TechnicalNodeIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TechnicalRelationIdentifier(String);

impl TechnicalRelationIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct TechnicalNodeKey(String);

impl TechnicalNodeKey {
    pub fn try_new(value: String) -> MindResult<Self> {
        Self::from_canonical(value).map_err(|rejection| Error::InvalidTechnicalNodeKey {
            key: rejection.supplied_key.as_str().to_string(),
            reason: rejection.reason.to_string(),
        })
    }

    pub fn from_canonical(
        value: impl Into<String>,
    ) -> std::result::Result<Self, TechnicalNodeKeyRejection> {
        let value = value.into();
        let parts = value.split(':').collect::<Vec<_>>();
        if parts.len() < 2 {
            return Err(TechnicalNodeKeyRejection::new(
                value,
                TechnicalNodeKeyRejectionReason::MissingFamilySeparator,
            ));
        }

        let family = match TechnicalNodeKeyFamily::from_prefix(parts[0]) {
            Some(family) => family,
            None => {
                return Err(TechnicalNodeKeyRejection::new(
                    value,
                    TechnicalNodeKeyRejectionReason::UnknownFamily,
                ));
            }
        };

        if parts.len() != family.segment_count() + 1 {
            return Err(TechnicalNodeKeyRejection::new(
                value,
                TechnicalNodeKeyRejectionReason::WrongSegmentCount,
            ));
        }

        if parts[1..].iter().any(|segment| segment.is_empty()) {
            return Err(TechnicalNodeKeyRejection::new(
                value,
                TechnicalNodeKeyRejectionReason::EmptySegment,
            ));
        }

        if parts[1..]
            .iter()
            .flat_map(|segment| segment.chars())
            .any(|character| {
                !(character.is_ascii_lowercase()
                    || character.is_ascii_digit()
                    || matches!(character, '-' | '_' | '.'))
            })
        {
            return Err(TechnicalNodeKeyRejection::new(
                value,
                TechnicalNodeKeyRejectionReason::InvalidSegmentCharacter,
            ));
        }

        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn family(&self) -> TechnicalNodeKeyFamily {
        let family = self
            .0
            .split(':')
            .next()
            .expect("validated technical node key has a family prefix");
        TechnicalNodeKeyFamily::from_prefix(family)
            .expect("validated technical node key has a known family prefix")
    }

    pub fn expected_node_kind(&self) -> TechnicalNodeKind {
        self.family().node_kind()
    }
}

impl TryFrom<String> for TechnicalNodeKey {
    type Error = Error;

    fn try_from(value: String) -> MindResult<Self> {
        Self::try_new(value)
    }
}

impl TryFrom<&str> for TechnicalNodeKey {
    type Error = Error;

    fn try_from(value: &str) -> MindResult<Self> {
        Self::try_new(value.to_string())
    }
}

impl FromStr for TechnicalNodeKey {
    type Err = Error;

    fn from_str(value: &str) -> MindResult<Self> {
        Self::try_from(value)
    }
}

impl AsRef<str> for TechnicalNodeKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl NotaDecode for TechnicalNodeKey {
    fn from_nota_block(block: &Block) -> std::result::Result<Self, NotaDecodeError> {
        let key = NotaBlock::new(block).parse_string()?;
        Self::from_canonical(key).map_err(|rejection| NotaDecodeError::Parse(rejection.to_string()))
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum TechnicalNodeKeyFamily {
    Component,
    Repository,
    Crate,
    Contract,
    WorkItem,
    SourceArtifact,
    Report,
    TechnicalClaim,
    Witness,
    StorageResource,
    SchemaFamily,
    Table,
}

impl TechnicalNodeKeyFamily {
    pub const ALL: [Self; 12] = [
        Self::Component,
        Self::Repository,
        Self::Crate,
        Self::Contract,
        Self::WorkItem,
        Self::SourceArtifact,
        Self::Report,
        Self::TechnicalClaim,
        Self::Witness,
        Self::StorageResource,
        Self::SchemaFamily,
        Self::Table,
    ];

    pub const fn prefix(self) -> &'static str {
        match self {
            Self::Component => "component",
            Self::Repository => "repo",
            Self::Crate => "crate",
            Self::Contract => "contract",
            Self::WorkItem => "task",
            Self::SourceArtifact => "artifact",
            Self::Report => "report",
            Self::TechnicalClaim => "claim",
            Self::Witness => "witness",
            Self::StorageResource => "storage",
            Self::SchemaFamily => "schema",
            Self::Table => "table",
        }
    }

    pub fn from_prefix(prefix: &str) -> Option<Self> {
        match prefix {
            "component" => Some(Self::Component),
            "repo" => Some(Self::Repository),
            "crate" => Some(Self::Crate),
            "contract" => Some(Self::Contract),
            "task" => Some(Self::WorkItem),
            "artifact" => Some(Self::SourceArtifact),
            "report" => Some(Self::Report),
            "claim" => Some(Self::TechnicalClaim),
            "witness" => Some(Self::Witness),
            "storage" => Some(Self::StorageResource),
            "schema" => Some(Self::SchemaFamily),
            "table" => Some(Self::Table),
            _ => None,
        }
    }

    pub const fn segment_count(self) -> usize {
        match self {
            Self::Contract | Self::StorageResource | Self::SchemaFamily | Self::Table => 2,
            Self::Component
            | Self::Repository
            | Self::Crate
            | Self::WorkItem
            | Self::SourceArtifact
            | Self::Report
            | Self::TechnicalClaim
            | Self::Witness => 1,
        }
    }

    pub const fn node_kind(self) -> TechnicalNodeKind {
        match self {
            Self::Component => TechnicalNodeKind::Component,
            Self::Repository => TechnicalNodeKind::Repository,
            Self::Crate => TechnicalNodeKind::Crate,
            Self::Contract => TechnicalNodeKind::Contract,
            Self::WorkItem => TechnicalNodeKind::WorkItem,
            Self::SourceArtifact => TechnicalNodeKind::SourceArtifact,
            Self::Report => TechnicalNodeKind::Report,
            Self::TechnicalClaim => TechnicalNodeKind::TechnicalClaim,
            Self::Witness => TechnicalNodeKind::Witness,
            Self::StorageResource => TechnicalNodeKind::StorageResource,
            Self::SchemaFamily => TechnicalNodeKind::SchemaFamily,
            Self::Table => TechnicalNodeKind::Table,
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TechnicalNodeKeyRejection {
    pub supplied_key: TextBody,
    pub reason: TechnicalNodeKeyRejectionReason,
}

impl TechnicalNodeKeyRejection {
    pub fn new(supplied_key: impl Into<String>, reason: TechnicalNodeKeyRejectionReason) -> Self {
        Self {
            supplied_key: TextBody::new(supplied_key),
            reason,
        }
    }
}

impl fmt::Display for TechnicalNodeKeyRejection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.supplied_key.as_str(), self.reason)
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum TechnicalNodeKeyRejectionReason {
    MissingFamilySeparator,
    UnknownFamily,
    WrongSegmentCount,
    EmptySegment,
    InvalidSegmentCharacter,
}

impl fmt::Display for TechnicalNodeKeyRejectionReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::MissingFamilySeparator => "missing family separator",
            Self::UnknownFamily => "unknown family",
            Self::WrongSegmentCount => "wrong segment count",
            Self::EmptySegment => "empty segment",
            Self::InvalidSegmentCharacter => "invalid segment character",
        })
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum TechnicalNodeKind {
    Component,
    Repository,
    Crate,
    Contract,
    WorkItem,
    SourceArtifact,
    Report,
    TechnicalClaim,
    Witness,
    StorageResource,
    SchemaFamily,
    Table,
}

impl TechnicalNodeKind {
    pub const ALL: [Self; 12] = [
        Self::Component,
        Self::Repository,
        Self::Crate,
        Self::Contract,
        Self::WorkItem,
        Self::SourceArtifact,
        Self::Report,
        Self::TechnicalClaim,
        Self::Witness,
        Self::StorageResource,
        Self::SchemaFamily,
        Self::Table,
    ];

    pub fn validate_body(
        self,
        body: &TechnicalNodeBody,
    ) -> std::result::Result<(), TechnicalNodeKindMismatch> {
        let got_body_kind = body.kind();
        if self == got_body_kind {
            Ok(())
        } else {
            Err(TechnicalNodeKindMismatch {
                expected_kind: self,
                got_body_kind,
            })
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TechnicalNodeKindMismatch {
    pub expected_kind: TechnicalNodeKind,
    pub got_body_kind: TechnicalNodeKind,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum TechnicalNodeBody {
    Component(ComponentNode),
    Repository(RepositoryNode),
    Crate(CrateNode),
    Contract(ContractNode),
    WorkItem(WorkItemNode),
    SourceArtifact(SourceArtifactNode),
    Report(ReportNode),
    TechnicalClaim(TechnicalClaimNode),
    Witness(WitnessNode),
    StorageResource(StorageResourceNode),
    SchemaFamily(SchemaFamilyNode),
    Table(TableNode),
}

impl TechnicalNodeBody {
    pub const fn kind(&self) -> TechnicalNodeKind {
        match self {
            Self::Component(_) => TechnicalNodeKind::Component,
            Self::Repository(_) => TechnicalNodeKind::Repository,
            Self::Crate(_) => TechnicalNodeKind::Crate,
            Self::Contract(_) => TechnicalNodeKind::Contract,
            Self::WorkItem(_) => TechnicalNodeKind::WorkItem,
            Self::SourceArtifact(_) => TechnicalNodeKind::SourceArtifact,
            Self::Report(_) => TechnicalNodeKind::Report,
            Self::TechnicalClaim(_) => TechnicalNodeKind::TechnicalClaim,
            Self::Witness(_) => TechnicalNodeKind::Witness,
            Self::StorageResource(_) => TechnicalNodeKind::StorageResource,
            Self::SchemaFamily(_) => TechnicalNodeKind::SchemaFamily,
            Self::Table(_) => TechnicalNodeKind::Table,
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentNode {
    pub component: ComponentName,
    pub summary: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RepositoryNode {
    pub path: WirePath,
    pub remote: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CrateNode {
    pub name: TextBody,
    pub repository: TechnicalNodeKey,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ContractNode {
    pub name: TextBody,
    pub surface: ContractSurface,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ContractSurface {
    Ordinary,
    Meta,
    Introspection,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct WorkItemNode {
    pub task: TaskToken,
    pub title: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SourceArtifactNode {
    pub locator: TechnicalSourceLocator,
    pub summary: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ReportNode {
    pub path: WirePath,
    pub summary: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TechnicalClaimNode {
    pub claim: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct WitnessNode {
    pub summary: TextBody,
    pub locator: Option<TechnicalSourceLocator>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct StorageResourceNode {
    pub owner: TechnicalNodeKey,
    pub name: TextBody,
    pub path: Option<WirePath>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SchemaFamilyNode {
    pub owner: TechnicalNodeKey,
    pub name: TextBody,
    pub version: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TableNode {
    pub storage: TechnicalNodeKey,
    pub name: TextBody,
    pub schema_family: Option<TechnicalNodeKey>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum TechnicalSourceLocator {
    Path(WirePath),
    Repository(TechnicalNodeKey),
    Task(TaskToken),
    Url(TextBody),
    Report(WirePath),
    Symbol(TextBody),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TechnicalNode {
    pub identifier: TechnicalNodeIdentifier,
    pub stable_key: TechnicalNodeKey,
    pub kind: TechnicalNodeKind,
    pub body: TechnicalNodeBody,
    pub author: ActorName,
    pub occurred_at: TimestampNanos,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum TechnicalRelationKind {
    OwnsRepository,
    DefinesContract,
    DefinesCrate,
    BuildDependency,
    RuntimeDependency,
    WireDependency,
    StorageDependency,
    TaskDependency,
    ProvenanceDependency,
    Blocks,
    Implements,
    Documents,
    ClaimsAbout,
    ProvenBy,
    Supersedes,
    LocatedAt,
}

impl TechnicalRelationKind {
    pub const ALL: [Self; 16] = [
        Self::OwnsRepository,
        Self::DefinesContract,
        Self::DefinesCrate,
        Self::BuildDependency,
        Self::RuntimeDependency,
        Self::WireDependency,
        Self::StorageDependency,
        Self::TaskDependency,
        Self::ProvenanceDependency,
        Self::Blocks,
        Self::Implements,
        Self::Documents,
        Self::ClaimsAbout,
        Self::ProvenBy,
        Self::Supersedes,
        Self::LocatedAt,
    ];

    pub fn validate_endpoint_kinds(
        self,
        source: TechnicalNodeKind,
        target: TechnicalNodeKind,
    ) -> std::result::Result<(), TechnicalRelationKindMismatch> {
        let expected_source_kinds = self.expected_source_kinds();
        let expected_target_kinds = self.expected_target_kinds(source);
        if expected_source_kinds.contains(&source) && expected_target_kinds.contains(&target) {
            Ok(())
        } else {
            Err(TechnicalRelationKindMismatch {
                relation: self,
                expected_source_kinds,
                expected_target_kinds,
                got_source_kind: source,
                got_target_kind: target,
            })
        }
    }

    pub fn expected_source_kinds(self) -> Vec<TechnicalNodeKind> {
        match self {
            Self::OwnsRepository => vec![TechnicalNodeKind::Component],
            Self::DefinesContract => vec![TechnicalNodeKind::Repository, TechnicalNodeKind::Crate],
            Self::DefinesCrate => vec![TechnicalNodeKind::Repository],
            Self::BuildDependency => vec![
                TechnicalNodeKind::Component,
                TechnicalNodeKind::Repository,
                TechnicalNodeKind::Crate,
                TechnicalNodeKind::Contract,
                TechnicalNodeKind::SchemaFamily,
            ],
            Self::RuntimeDependency | Self::WireDependency => vec![
                TechnicalNodeKind::Component,
                TechnicalNodeKind::Crate,
                TechnicalNodeKind::Contract,
            ],
            Self::StorageDependency => vec![
                TechnicalNodeKind::Component,
                TechnicalNodeKind::Crate,
                TechnicalNodeKind::Contract,
                TechnicalNodeKind::StorageResource,
                TechnicalNodeKind::SchemaFamily,
                TechnicalNodeKind::Table,
            ],
            Self::TaskDependency | Self::Blocks => vec![TechnicalNodeKind::WorkItem],
            Self::ProvenanceDependency => vec![
                TechnicalNodeKind::WorkItem,
                TechnicalNodeKind::SourceArtifact,
                TechnicalNodeKind::Report,
                TechnicalNodeKind::TechnicalClaim,
                TechnicalNodeKind::Witness,
            ],
            Self::Implements => vec![
                TechnicalNodeKind::Component,
                TechnicalNodeKind::Crate,
                TechnicalNodeKind::SourceArtifact,
                TechnicalNodeKind::WorkItem,
            ],
            Self::Documents => vec![TechnicalNodeKind::Report, TechnicalNodeKind::SourceArtifact],
            Self::ClaimsAbout => vec![TechnicalNodeKind::TechnicalClaim],
            Self::ProvenBy => vec![TechnicalNodeKind::TechnicalClaim],
            Self::Supersedes => TechnicalNodeKind::ALL.to_vec(),
            Self::LocatedAt => TechnicalNodeKind::ALL.to_vec(),
        }
    }

    pub fn expected_target_kinds(self, source: TechnicalNodeKind) -> Vec<TechnicalNodeKind> {
        match self {
            Self::OwnsRepository => vec![TechnicalNodeKind::Repository],
            Self::DefinesContract => vec![TechnicalNodeKind::Contract],
            Self::DefinesCrate => vec![TechnicalNodeKind::Crate],
            Self::BuildDependency => vec![
                TechnicalNodeKind::Repository,
                TechnicalNodeKind::Crate,
                TechnicalNodeKind::Contract,
                TechnicalNodeKind::SourceArtifact,
                TechnicalNodeKind::SchemaFamily,
            ],
            Self::RuntimeDependency => vec![
                TechnicalNodeKind::Component,
                TechnicalNodeKind::Crate,
                TechnicalNodeKind::Contract,
                TechnicalNodeKind::StorageResource,
            ],
            Self::WireDependency => vec![TechnicalNodeKind::Contract],
            Self::StorageDependency => vec![
                TechnicalNodeKind::StorageResource,
                TechnicalNodeKind::SchemaFamily,
                TechnicalNodeKind::Table,
            ],
            Self::TaskDependency | Self::Blocks => vec![TechnicalNodeKind::WorkItem],
            Self::ProvenanceDependency => vec![
                TechnicalNodeKind::WorkItem,
                TechnicalNodeKind::SourceArtifact,
                TechnicalNodeKind::Report,
                TechnicalNodeKind::TechnicalClaim,
                TechnicalNodeKind::Witness,
            ],
            Self::Implements => vec![
                TechnicalNodeKind::TechnicalClaim,
                TechnicalNodeKind::Contract,
            ],
            Self::Documents => TechnicalNodeKind::ALL.to_vec(),
            Self::ClaimsAbout => TechnicalNodeKind::ALL.to_vec(),
            Self::ProvenBy => vec![TechnicalNodeKind::Witness],
            Self::Supersedes => vec![source],
            Self::LocatedAt => vec![TechnicalNodeKind::SourceArtifact],
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TechnicalRelationKindMismatch {
    pub relation: TechnicalRelationKind,
    pub expected_source_kinds: Vec<TechnicalNodeKind>,
    pub expected_target_kinds: Vec<TechnicalNodeKind>,
    pub got_source_kind: TechnicalNodeKind,
    pub got_target_kind: TechnicalNodeKind,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TechnicalRelationEndpoint {
    pub identifier: TechnicalNodeIdentifier,
    pub stable_key: TechnicalNodeKey,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TechnicalRelation {
    pub identifier: TechnicalRelationIdentifier,
    pub kind: TechnicalRelationKind,
    pub source: TechnicalRelationEndpoint,
    pub target: TechnicalRelationEndpoint,
    pub author: ActorName,
    pub occurred_at: TimestampNanos,
    pub note: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubmitTechnicalNode {
    pub stable_key: TechnicalNodeKey,
    pub kind: TechnicalNodeKind,
    pub body: TechnicalNodeBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubmitTechnicalRelation {
    pub kind: TechnicalRelationKind,
    pub source: TechnicalNodeKey,
    pub target: TechnicalNodeKey,
    pub note: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct QueryTechnicalNodes {
    pub filter: TechnicalNodeFilter,
    pub limit: QueryLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct QueryTechnicalRelations {
    pub filter: TechnicalRelationFilter,
    pub limit: QueryLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubscribeTechnicalNodes {
    pub filter: TechnicalNodeFilter,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubscribeTechnicalRelations {
    pub filter: TechnicalRelationFilter,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum TechnicalNodeFilter {
    ByKind(ByTechnicalNodeKind),
    ByStableKey(ByTechnicalNodeStableKey),
    BySourceLocator(ByTechnicalSourceLocator),
    Composite(CompositeTechnicalNodeFilter),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ByTechnicalNodeKind {
    pub kinds: Vec<TechnicalNodeKind>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ByTechnicalNodeStableKey {
    pub stable_key: TechnicalNodeKey,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ByTechnicalSourceLocator {
    pub locator: TechnicalSourceLocator,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CompositeTechnicalNodeFilter {
    pub kinds: Vec<TechnicalNodeKind>,
    pub stable_key: Option<TechnicalNodeKey>,
    pub source_locator: Option<TechnicalSourceLocator>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum TechnicalRelationFilter {
    ByKind(ByTechnicalRelationKind),
    BySource(ByTechnicalRelationSource),
    ByTarget(ByTechnicalRelationTarget),
    BetweenEndpoints(ByTechnicalRelationEndpoints),
    Composite(CompositeTechnicalRelationFilter),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ByTechnicalRelationKind {
    pub kinds: Vec<TechnicalRelationKind>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ByTechnicalRelationSource {
    pub source: TechnicalNodeKey,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ByTechnicalRelationTarget {
    pub target: TechnicalNodeKey,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ByTechnicalRelationEndpoints {
    pub source: TechnicalNodeKey,
    pub target: TechnicalNodeKey,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CompositeTechnicalRelationFilter {
    pub kinds: Vec<TechnicalRelationKind>,
    pub source: Option<TechnicalNodeKey>,
    pub target: Option<TechnicalNodeKey>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TechnicalNodeCommitted {
    pub node: TechnicalNode,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TechnicalRelationCommitted {
    pub relation: TechnicalRelation,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TechnicalNodeList {
    pub nodes: Vec<TechnicalNode>,
    pub has_more: bool,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TechnicalRelationList {
    pub relations: Vec<TechnicalRelation>,
    pub has_more: bool,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TechnicalNodeRejected {
    pub reason: TechnicalNodeRejectionReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum TechnicalNodeRejectionReason {
    InvalidStableNodeKey(TechnicalNodeKeyRejection),
    KindBodyMismatch(TechnicalNodeKindMismatch),
    DuplicateStableNodeKey(TechnicalNodeKey),
    PersistenceRejected,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TechnicalRelationRejected {
    pub reason: TechnicalRelationRejectionReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum TechnicalRelationRejectionReason {
    DuplicateRelation,
    MissingEndpoint(TechnicalNodeKey),
    DomainRangeViolation(TechnicalRelationKindMismatch),
    PersistenceRejected,
}
