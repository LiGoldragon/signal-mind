use nota::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_persona::ComponentName;

use crate::{ActorName, ContractSurface, QueryLimit, TextBody, TimestampNanos};

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
pub struct KnowledgeName(String);

impl KnowledgeName {
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
pub enum KnowledgeSubject {
    Component,
    Contract,
    Repository,
    Architecture,
    Interface,
    Storage,
    Source,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeIdentity {
    Component(ComponentName),
    Repository(KnowledgeName),
    Crate(KnowledgeName),
    Contract(KnowledgeName, ContractSurface),
    Statement(KnowledgeName),
    Source(KnowledgeName),
    Domain(KnowledgeSubject),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeIdentitySlot {
    Unkeyed,
    Keyed(KnowledgeIdentity),
}

impl KnowledgeIdentitySlot {
    pub fn as_identity(&self) -> Option<&KnowledgeIdentity> {
        match self {
            Self::Unkeyed => None,
            Self::Keyed(identity) => Some(identity),
        }
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
    pub identity: KnowledgeIdentitySlot,
    pub accepted_by: ActorName,
    pub accepted_at: TimestampNanos,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeEntity {
    pub header: KnowledgeRecordHeader,
    pub name: TextBody,
    pub description: Vec<TextBody>,
    pub domains: Vec<KnowledgeSubject>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeStatement {
    pub header: KnowledgeRecordHeader,
    pub body: TextBody,
    pub about: Vec<KnowledgeIdentifier>,
    pub domains: Vec<KnowledgeSubject>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeDomain {
    pub header: KnowledgeRecordHeader,
    pub subject: KnowledgeSubject,
    pub name: TextBody,
    pub description: Vec<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeSource {
    pub header: KnowledgeRecordHeader,
    pub locator: TextBody,
    pub description: Vec<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeRelationEndpoint {
    pub identifier: KnowledgeIdentifier,
    pub identity: KnowledgeIdentitySlot,
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
    pub note: Vec<TextBody>,
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
    pub summaries: Vec<TextBody>,
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
pub enum KnowledgeEntityCandidate {
    Keyed(
        KnowledgeIdentity,
        TextBody,
        Vec<TextBody>,
        Vec<KnowledgeSubject>,
    ),
    Unkeyed(TextBody, Vec<TextBody>, Vec<KnowledgeSubject>),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeStatementCandidate {
    Keyed(
        KnowledgeIdentity,
        TextBody,
        Vec<KnowledgeIdentifier>,
        Vec<KnowledgeSubject>,
    ),
    Unkeyed(TextBody, Vec<KnowledgeIdentifier>, Vec<KnowledgeSubject>),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeRelationCandidate {
    pub kind: KnowledgeRelationKind,
    pub source: KnowledgeEndpointSelector,
    pub target: KnowledgeEndpointSelector,
    pub note: Vec<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeDomainCandidate {
    pub subject: KnowledgeSubject,
    pub name: TextBody,
    pub description: Vec<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeSourceCandidate {
    Keyed(KnowledgeIdentity, TextBody, Vec<TextBody>),
    Unkeyed(TextBody, Vec<TextBody>),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeEndpointSelector {
    Identifier(KnowledgeIdentifier),
    Identity(KnowledgeIdentity),
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
    Accept,
    Reject(KnowledgeRejection),
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
    WrongDomain(KnowledgeSubject),
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
    DuplicateIdentity(KnowledgeIdentity),
    PersistenceRejected,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeQuery {
    GetByIdentifier(KnowledgeIdentifier),
    GetByIdentity(KnowledgeIdentity),
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
    Direct(KnowledgeSubject),
    WithDescendants(KnowledgeSubject),
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
