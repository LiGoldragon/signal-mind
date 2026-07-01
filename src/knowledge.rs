use nota::{Block, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use std::fmt;
use std::str::FromStr;

use crate::{ActorName, Error, MindResult, QueryLimit, TextBody, TimestampNanos};

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
pub struct KnowledgeIdentifier(String);

impl KnowledgeIdentifier {
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
pub struct KnowledgeStableKey(String);

impl KnowledgeStableKey {
    pub fn try_new(value: String) -> MindResult<Self> {
        Self::from_canonical(value).map_err(|rejection| Error::InvalidKnowledgeStableKey {
            key: rejection.supplied_key.as_str().to_string(),
            reason: rejection.reason.to_string(),
        })
    }

    pub fn from_canonical(
        value: impl Into<String>,
    ) -> std::result::Result<Self, KnowledgeKeyRejection> {
        let value = value.into();
        KnowledgeKeyShape::Generic.validate(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for KnowledgeStableKey {
    type Error = Error;

    fn try_from(value: String) -> MindResult<Self> {
        Self::try_new(value)
    }
}

impl TryFrom<&str> for KnowledgeStableKey {
    type Error = Error;

    fn try_from(value: &str) -> MindResult<Self> {
        Self::try_new(value.to_string())
    }
}

impl FromStr for KnowledgeStableKey {
    type Err = Error;

    fn from_str(value: &str) -> MindResult<Self> {
        Self::try_from(value)
    }
}

impl AsRef<str> for KnowledgeStableKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl NotaDecode for KnowledgeStableKey {
    fn from_nota_block(block: &Block) -> std::result::Result<Self, NotaDecodeError> {
        let key = NotaBlock::new(block).parse_string()?;
        Self::from_canonical(key).map_err(|rejection| NotaDecodeError::Parse(rejection.to_string()))
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct KnowledgeDomainKey(String);

impl KnowledgeDomainKey {
    pub fn try_new(value: String) -> MindResult<Self> {
        Self::from_canonical(value).map_err(|rejection| Error::InvalidKnowledgeDomainKey {
            key: rejection.supplied_key.as_str().to_string(),
            reason: rejection.reason.to_string(),
        })
    }

    pub fn from_canonical(
        value: impl Into<String>,
    ) -> std::result::Result<Self, KnowledgeKeyRejection> {
        let value = value.into();
        KnowledgeKeyShape::Domain.validate(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for KnowledgeDomainKey {
    type Error = Error;

    fn try_from(value: String) -> MindResult<Self> {
        Self::try_new(value)
    }
}

impl TryFrom<&str> for KnowledgeDomainKey {
    type Error = Error;

    fn try_from(value: &str) -> MindResult<Self> {
        Self::try_new(value.to_string())
    }
}

impl FromStr for KnowledgeDomainKey {
    type Err = Error;

    fn from_str(value: &str) -> MindResult<Self> {
        Self::try_from(value)
    }
}

impl AsRef<str> for KnowledgeDomainKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl NotaDecode for KnowledgeDomainKey {
    fn from_nota_block(block: &Block) -> std::result::Result<Self, NotaDecodeError> {
        let key = NotaBlock::new(block).parse_string()?;
        Self::from_canonical(key).map_err(|rejection| NotaDecodeError::Parse(rejection.to_string()))
    }
}

enum KnowledgeKeyShape {
    Generic,
    Domain,
}

impl KnowledgeKeyShape {
    fn validate(&self, value: &str) -> std::result::Result<(), KnowledgeKeyRejection> {
        let parts = value.split(':').collect::<Vec<_>>();
        if parts.len() < 2 {
            return Err(KnowledgeKeyRejection::new(
                value,
                KnowledgeKeyRejectionReason::MissingFamilySeparator,
            ));
        }

        if matches!(self, Self::Domain) && parts[0] != "domain" {
            return Err(KnowledgeKeyRejection::new(
                value,
                KnowledgeKeyRejectionReason::WrongFamily,
            ));
        }

        if parts.iter().any(|segment| segment.is_empty()) {
            return Err(KnowledgeKeyRejection::new(
                value,
                KnowledgeKeyRejectionReason::EmptySegment,
            ));
        }

        if parts
            .iter()
            .flat_map(|segment| segment.chars())
            .any(|character| {
                !(character.is_ascii_lowercase()
                    || character.is_ascii_digit()
                    || matches!(character, '-' | '_' | '.' | ':'))
            })
        {
            return Err(KnowledgeKeyRejection::new(
                value,
                KnowledgeKeyRejectionReason::InvalidSegmentCharacter,
            ));
        }

        Ok(())
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeKeyRejection {
    pub supplied_key: TextBody,
    pub reason: KnowledgeKeyRejectionReason,
}

impl KnowledgeKeyRejection {
    pub fn new(supplied_key: impl Into<String>, reason: KnowledgeKeyRejectionReason) -> Self {
        Self {
            supplied_key: TextBody::new(supplied_key),
            reason,
        }
    }
}

impl fmt::Display for KnowledgeKeyRejection {
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
pub enum KnowledgeKeyRejectionReason {
    MissingFamilySeparator,
    WrongFamily,
    EmptySegment,
    InvalidSegmentCharacter,
}

impl fmt::Display for KnowledgeKeyRejectionReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::MissingFamilySeparator => "missing family separator",
            Self::WrongFamily => "wrong family",
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
pub enum KnowledgeRecordKind {
    Entity,
    Statement,
    Relation,
    Domain,
    Source,
}

impl KnowledgeRecordKind {
    pub const ALL: [Self; 5] = [
        Self::Entity,
        Self::Statement,
        Self::Relation,
        Self::Domain,
        Self::Source,
    ];
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum AcceptedKnowledge {
    Entity(KnowledgeEntity),
    Statement(KnowledgeStatement),
    Relation(KnowledgeRelation),
    Domain(KnowledgeDomain),
    Source(KnowledgeSource),
}

impl AcceptedKnowledge {
    pub const fn kind(&self) -> KnowledgeRecordKind {
        match self {
            Self::Entity(_) => KnowledgeRecordKind::Entity,
            Self::Statement(_) => KnowledgeRecordKind::Statement,
            Self::Relation(_) => KnowledgeRecordKind::Relation,
            Self::Domain(_) => KnowledgeRecordKind::Domain,
            Self::Source(_) => KnowledgeRecordKind::Source,
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeRecordHeader {
    pub identifier: KnowledgeIdentifier,
    pub stable_key: Option<KnowledgeStableKey>,
    pub accepted_by: ActorName,
    pub accepted_at: TimestampNanos,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeEntity {
    pub header: KnowledgeRecordHeader,
    pub name: TextBody,
    pub description: Option<TextBody>,
    pub domains: Vec<KnowledgeDomainKey>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeStatement {
    pub header: KnowledgeRecordHeader,
    pub body: TextBody,
    pub about: Vec<KnowledgeIdentifier>,
    pub domains: Vec<KnowledgeDomainKey>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeDomain {
    pub header: KnowledgeRecordHeader,
    pub domain_key: KnowledgeDomainKey,
    pub name: TextBody,
    pub description: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeSource {
    pub header: KnowledgeRecordHeader,
    pub locator: TextBody,
    pub description: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeRelationEndpoint {
    pub identifier: KnowledgeIdentifier,
    pub stable_key: Option<KnowledgeStableKey>,
    pub kind: KnowledgeRecordKind,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeRelation {
    pub header: KnowledgeRecordHeader,
    pub kind: KnowledgeRelationKind,
    pub source: KnowledgeRelationEndpoint,
    pub target: KnowledgeRelationEndpoint,
    pub note: Option<TextBody>,
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
pub enum KnowledgeRelationKind {
    ClassifiedAs,
    BroaderThan,
    NarrowerThan,
    RelatedTo,
    References,
    SupportedBy,
    Contradicts,
    Supersedes,
    Defines,
    Implements,
    DependsOn,
}

impl KnowledgeRelationKind {
    pub const ALL: [Self; 11] = [
        Self::ClassifiedAs,
        Self::BroaderThan,
        Self::NarrowerThan,
        Self::RelatedTo,
        Self::References,
        Self::SupportedBy,
        Self::Contradicts,
        Self::Supersedes,
        Self::Defines,
        Self::Implements,
        Self::DependsOn,
    ];

    pub fn validate_endpoint_kinds(
        self,
        source: KnowledgeRecordKind,
        target: KnowledgeRecordKind,
    ) -> std::result::Result<(), KnowledgeRelationKindMismatch> {
        let expected_source_kinds = self.expected_source_kinds();
        let expected_target_kinds = self.expected_target_kinds();
        if expected_source_kinds.contains(&source) && expected_target_kinds.contains(&target) {
            Ok(())
        } else {
            Err(KnowledgeRelationKindMismatch {
                relation: self,
                expected_source_kinds,
                expected_target_kinds,
                got_source_kind: source,
                got_target_kind: target,
            })
        }
    }

    pub fn validate_endpoints(
        self,
        source: &KnowledgeRelationEndpoint,
        target: &KnowledgeRelationEndpoint,
    ) -> std::result::Result<(), KnowledgeRelationKindMismatch> {
        self.validate_endpoint_kinds(source.kind, target.kind)
    }

    pub fn expected_source_kinds(self) -> Vec<KnowledgeRecordKind> {
        match self {
            Self::ClassifiedAs | Self::Contradicts | Self::Supersedes | Self::DependsOn => {
                KnowledgeRecordKind::ALL.to_vec()
            }
            Self::BroaderThan | Self::NarrowerThan => vec![KnowledgeRecordKind::Domain],
            Self::RelatedTo => vec![KnowledgeRecordKind::Entity, KnowledgeRecordKind::Domain],
            Self::References => KnowledgeRecordKind::ALL.to_vec(),
            Self::SupportedBy => vec![
                KnowledgeRecordKind::Entity,
                KnowledgeRecordKind::Statement,
                KnowledgeRecordKind::Relation,
                KnowledgeRecordKind::Domain,
            ],
            Self::Defines => vec![KnowledgeRecordKind::Entity, KnowledgeRecordKind::Source],
            Self::Implements => vec![KnowledgeRecordKind::Entity],
        }
    }

    pub fn expected_target_kinds(self) -> Vec<KnowledgeRecordKind> {
        match self {
            Self::ClassifiedAs => vec![KnowledgeRecordKind::Domain],
            Self::BroaderThan | Self::NarrowerThan => vec![KnowledgeRecordKind::Domain],
            Self::RelatedTo => vec![KnowledgeRecordKind::Entity, KnowledgeRecordKind::Domain],
            Self::References => vec![KnowledgeRecordKind::Entity, KnowledgeRecordKind::Source],
            Self::SupportedBy => vec![KnowledgeRecordKind::Statement, KnowledgeRecordKind::Source],
            Self::Contradicts | Self::Supersedes | Self::DependsOn => {
                KnowledgeRecordKind::ALL.to_vec()
            }
            Self::Defines => vec![KnowledgeRecordKind::Entity, KnowledgeRecordKind::Domain],
            Self::Implements => vec![
                KnowledgeRecordKind::Entity,
                KnowledgeRecordKind::Statement,
                KnowledgeRecordKind::Domain,
            ],
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeRelationKindMismatch {
    pub relation: KnowledgeRelationKind,
    pub expected_source_kinds: Vec<KnowledgeRecordKind>,
    pub expected_target_kinds: Vec<KnowledgeRecordKind>,
    pub got_source_kind: KnowledgeRecordKind,
    pub got_target_kind: KnowledgeRecordKind,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeSubmission {
    pub candidate: KnowledgeCandidate,
    pub fixture_policy: KnowledgeFixturePolicy,
    pub requester_context: KnowledgeRequesterContext,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeFixturePolicy {
    FixtureOnly,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeRequesterContext {
    pub request_summary: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeCandidate {
    Entity(KnowledgeEntityCandidate),
    Statement(KnowledgeStatementCandidate),
    Relation(KnowledgeRelationCandidate),
    Domain(KnowledgeDomainCandidate),
    Source(KnowledgeSourceCandidate),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeEntityCandidate {
    pub stable_key: Option<KnowledgeStableKey>,
    pub name: TextBody,
    pub description: Option<TextBody>,
    pub domains: Vec<KnowledgeDomainKey>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeStatementCandidate {
    pub stable_key: Option<KnowledgeStableKey>,
    pub body: TextBody,
    pub about: Vec<KnowledgeIdentifier>,
    pub domains: Vec<KnowledgeDomainKey>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeRelationCandidate {
    pub kind: KnowledgeRelationKind,
    pub source: KnowledgeEndpointSelector,
    pub target: KnowledgeEndpointSelector,
    pub note: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeDomainCandidate {
    pub domain_key: KnowledgeDomainKey,
    pub name: TextBody,
    pub description: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeSourceCandidate {
    pub stable_key: Option<KnowledgeStableKey>,
    pub locator: TextBody,
    pub description: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeEndpointSelector {
    Identifier(KnowledgeIdentifier),
    StableKey(KnowledgeStableKey),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeJudgePacket {
    pub candidate: KnowledgeCandidate,
    pub relevant_neighbors: Vec<AcceptedKnowledge>,
    pub allowed_relations: Vec<KnowledgeRelationRule>,
    pub fixture_policy: KnowledgeFixturePolicy,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeRelationRule {
    pub kind: KnowledgeRelationKind,
    pub source_kinds: Vec<KnowledgeRecordKind>,
    pub target_kinds: Vec<KnowledgeRecordKind>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeJudgeVerdict {
    Accept(AcceptedKnowledgeDraft),
    Reject(KnowledgeRejection),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AcceptedKnowledgeDraft {
    pub records: Vec<KnowledgeRecordDraft>,
    pub relations: Vec<KnowledgeRelationDraft>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeRecordDraft {
    Entity(KnowledgeEntityCandidate),
    Statement(KnowledgeStatementCandidate),
    Domain(KnowledgeDomainCandidate),
    Source(KnowledgeSourceCandidate),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeRelationDraft {
    pub kind: KnowledgeRelationKind,
    pub source: KnowledgeEndpointSelector,
    pub target: KnowledgeEndpointSelector,
    pub note: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeAccepted {
    pub accepted: AcceptedKnowledgeView,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AcceptedKnowledgeView {
    pub records: Vec<AcceptedKnowledge>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeList {
    pub records: Vec<AcceptedKnowledge>,
    pub has_more: bool,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeRejection {
    pub reason: KnowledgeRejectionReason,
    pub candidate_summary: CandidateSummary,
    pub retry_hint: Option<RetryHint>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CandidateSummary {
    pub summary: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RetryHint {
    pub hint: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeRejectionReason {
    NotKnowledge,
    PrivateOrUnauthorized,
    MeaningUnclear,
    FalseOrUnsupported,
    SemanticDuplicate(KnowledgeIdentifier),
    ConflictsAcceptedKnowledge(Vec<KnowledgeIdentifier>),
    WrongDomain(KnowledgeDomainKey),
    NeedsMoreSpecificShape(Vec<ExpectedKnowledgeShape>),
    SourceRequiredByCandidate,
    StructuralPreflightFailed(StructuralRejection),
    PersistenceRejected,
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
pub enum ExpectedKnowledgeShape {
    Entity,
    Statement,
    Relation,
    Domain,
    Source,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct StructuralRejection {
    pub reason: StructuralRejectionReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum StructuralRejectionReason {
    MissingEndpoint(KnowledgeEndpointSelector),
    RelationDomainRangeViolation(KnowledgeRelationKindMismatch),
    InvalidStableKey(KnowledgeKeyRejection),
    InvalidDomainKey(KnowledgeKeyRejection),
    EmptyAcceptedDraft,
    PersistenceRejected,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeQuery {
    GetByIdentifier(KnowledgeIdentifier),
    GetByStableKey(KnowledgeStableKey),
    ListByKind(KnowledgeRecordKind, CurrentView),
    ListByDomain(KnowledgeDomainSelector, CurrentView),
    ListRelations(RelationSelector, CurrentView),
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
pub enum CurrentView {
    CurrentOnly,
    IncludeSuperseded,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeDomainSelector {
    Any,
    Direct(KnowledgeDomainKey),
    WithDescendants(KnowledgeDomainKey),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RelationSelector {
    pub kind: Option<KnowledgeRelationKind>,
    pub source: Option<KnowledgeIdentifier>,
    pub target: Option<KnowledgeIdentifier>,
    pub limit: QueryLimit,
}
