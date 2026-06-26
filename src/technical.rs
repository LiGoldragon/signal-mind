use nota::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_persona::ComponentName;

use crate::{ActorName, QueryLimit, TaskToken, TextBody, TimestampNanos, WirePath};

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
pub struct TechnicalNodeKey(String);

impl TechnicalNodeKey {
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
}

impl TechnicalNodeKind {
    pub const ALL: [Self; 9] = [
        Self::Component,
        Self::Repository,
        Self::Crate,
        Self::Contract,
        Self::WorkItem,
        Self::SourceArtifact,
        Self::Report,
        Self::TechnicalClaim,
        Self::Witness,
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
    pub crate_key: TechnicalNodeKey,
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
    DependsOn,
    Blocks,
    Implements,
    UsesContract,
    UsesStorage,
    Documents,
    DerivedFrom,
    ClaimsAbout,
    ProvenBy,
    Supersedes,
    LocatedAt,
}

impl TechnicalRelationKind {
    pub const ALL: [Self; 14] = [
        Self::OwnsRepository,
        Self::DefinesContract,
        Self::DefinesCrate,
        Self::DependsOn,
        Self::Blocks,
        Self::Implements,
        Self::UsesContract,
        Self::UsesStorage,
        Self::Documents,
        Self::DerivedFrom,
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
            Self::DependsOn => vec![
                TechnicalNodeKind::Component,
                TechnicalNodeKind::Repository,
                TechnicalNodeKind::Crate,
                TechnicalNodeKind::Contract,
                TechnicalNodeKind::WorkItem,
            ],
            Self::Blocks => vec![TechnicalNodeKind::WorkItem],
            Self::Implements => vec![
                TechnicalNodeKind::Component,
                TechnicalNodeKind::Crate,
                TechnicalNodeKind::SourceArtifact,
                TechnicalNodeKind::WorkItem,
            ],
            Self::UsesContract | Self::UsesStorage => {
                vec![TechnicalNodeKind::Component, TechnicalNodeKind::Crate]
            }
            Self::Documents => vec![TechnicalNodeKind::Report, TechnicalNodeKind::SourceArtifact],
            Self::DerivedFrom | Self::Supersedes => TechnicalNodeKind::ALL.to_vec(),
            Self::ClaimsAbout => vec![TechnicalNodeKind::TechnicalClaim],
            Self::ProvenBy => vec![TechnicalNodeKind::TechnicalClaim],
            Self::LocatedAt => TechnicalNodeKind::ALL.to_vec(),
        }
    }

    pub fn expected_target_kinds(self, source: TechnicalNodeKind) -> Vec<TechnicalNodeKind> {
        match self {
            Self::OwnsRepository => vec![TechnicalNodeKind::Repository],
            Self::DefinesContract => vec![TechnicalNodeKind::Contract],
            Self::DefinesCrate => vec![TechnicalNodeKind::Crate],
            Self::DependsOn => vec![
                TechnicalNodeKind::Component,
                TechnicalNodeKind::Repository,
                TechnicalNodeKind::Crate,
                TechnicalNodeKind::Contract,
                TechnicalNodeKind::WorkItem,
            ],
            Self::Blocks => vec![TechnicalNodeKind::WorkItem],
            Self::Implements => vec![
                TechnicalNodeKind::TechnicalClaim,
                TechnicalNodeKind::Contract,
            ],
            Self::UsesContract => vec![TechnicalNodeKind::Contract],
            Self::UsesStorage => vec![TechnicalNodeKind::TechnicalClaim],
            Self::Documents => TechnicalNodeKind::ALL.to_vec(),
            Self::DerivedFrom => vec![TechnicalNodeKind::SourceArtifact, TechnicalNodeKind::Report],
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
