use nota_next::{Block, Delimiter, NotaBlock, NotaDecode, NotaDecodeError, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_persona_origin::{
    ChannelIdentifier, ComponentName, EngineIdentifier, HostName, UnixUserIdentifier,
};

use crate::{
    ActorName, DisplayIdentifier, RoleName, ScopeReference, TextBody, TimestampNanos, WirePath,
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
pub struct RecordIdentifier(String);

impl RecordIdentifier {
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
pub struct RelationIdentifier(String);

impl RelationIdentifier {
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
pub struct SubscriptionIdentifier(String);

impl SubscriptionIdentifier {
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
pub struct AlternativeIdentifier(String);

impl AlternativeIdentifier {
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
pub struct SymbolName(String);

impl SymbolName {
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
pub struct NormalizedUrl(String);

impl NormalizedUrl {
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
pub struct DocumentReference(String);

impl DocumentReference {
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
pub struct HarnessKind(String);

impl HarnessKind {
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
pub struct HarnessIdentifier(String);

impl HarnessIdentifier {
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
pub enum ThoughtKind {
    Observation,
    Memory,
    Belief,
    Goal,
    Claim,
    Decision,
    Reference,
}

impl ThoughtKind {
    pub const ALL: [Self; 7] = [
        Self::Observation,
        Self::Memory,
        Self::Belief,
        Self::Goal,
        Self::Claim,
        Self::Decision,
        Self::Reference,
    ];
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
pub enum RelationKind {
    Implements,
    Realizes,
    Requires,
    Supports,
    Refutes,
    Supersedes,
    Authored,
    References,
    Decides,
    Considered,
    Belongs,
}

impl RelationKind {
    pub const ALL: [Self; 11] = [
        Self::Implements,
        Self::Realizes,
        Self::Requires,
        Self::Supports,
        Self::Refutes,
        Self::Supersedes,
        Self::Authored,
        Self::References,
        Self::Decides,
        Self::Considered,
        Self::Belongs,
    ];

    pub fn validate_endpoint_kinds(
        self,
        source: ThoughtKind,
        target: ThoughtKind,
    ) -> std::result::Result<(), RelationKindMismatch> {
        let expected_source_kinds = self.expected_source_kinds();
        let expected_target_kinds = self.expected_target_kinds(source);
        if expected_source_kinds.contains(&source) && expected_target_kinds.contains(&target) {
            Ok(())
        } else {
            Err(RelationKindMismatch {
                relation: self,
                reason: RelationKindMismatchReason::DomainRange,
                expected_source_kinds,
                expected_target_kinds,
                got_source_kind: source,
                got_target_kind: target,
            })
        }
    }

    pub fn validate_endpoints(
        self,
        source: &Thought,
        target: &Thought,
    ) -> std::result::Result<(), RelationKindMismatch> {
        self.validate_endpoint_kinds(source.kind, target.kind)?;
        if self == Self::Authored && !source.is_identity_reference() {
            Err(RelationKindMismatch {
                relation: self,
                reason: RelationKindMismatchReason::AuthoredSourceNotIdentity,
                expected_source_kinds: vec![ThoughtKind::Reference],
                expected_target_kinds: self.expected_target_kinds(source.kind),
                got_source_kind: source.kind,
                got_target_kind: target.kind,
            })
        } else {
            Ok(())
        }
    }

    pub fn expected_source_kinds(self) -> Vec<ThoughtKind> {
        match self {
            Self::Implements => vec![ThoughtKind::Claim],
            Self::Realizes => vec![ThoughtKind::Observation],
            Self::Requires => vec![ThoughtKind::Goal, ThoughtKind::Claim],
            Self::Supports | Self::Refutes => vec![ThoughtKind::Observation, ThoughtKind::Belief],
            Self::Supersedes | Self::References | Self::Belongs => ThoughtKind::ALL.to_vec(),
            Self::Authored => vec![ThoughtKind::Reference],
            Self::Decides | Self::Considered => vec![ThoughtKind::Decision],
        }
    }

    pub fn expected_target_kinds(self, source: ThoughtKind) -> Vec<ThoughtKind> {
        match self {
            Self::Implements => vec![ThoughtKind::Goal],
            Self::Realizes => vec![ThoughtKind::Claim],
            Self::Requires => match source {
                ThoughtKind::Goal => vec![ThoughtKind::Goal],
                ThoughtKind::Claim => vec![ThoughtKind::Claim],
                _ => vec![ThoughtKind::Goal, ThoughtKind::Claim],
            },
            Self::Supports | Self::Refutes => vec![ThoughtKind::Belief],
            Self::Supersedes => vec![source],
            Self::Authored => ThoughtKind::ALL.to_vec(),
            Self::References => vec![ThoughtKind::Reference],
            Self::Decides => vec![ThoughtKind::Goal],
            Self::Considered => vec![ThoughtKind::Goal, ThoughtKind::Belief],
            Self::Belongs => vec![ThoughtKind::Memory, ThoughtKind::Goal],
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
pub enum RelationKindMismatchReason {
    DomainRange,
    AuthoredSourceNotIdentity,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RelationKindMismatch {
    pub relation: RelationKind,
    pub reason: RelationKindMismatchReason,
    pub expected_source_kinds: Vec<ThoughtKind>,
    pub expected_target_kinds: Vec<ThoughtKind>,
    pub got_source_kind: ThoughtKind,
    pub got_target_kind: ThoughtKind,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Thought {
    pub id: RecordIdentifier,
    pub kind: ThoughtKind,
    pub body: ThoughtBody,
    pub author: ActorName,
    pub occurred_at: TimestampNanos,
}

impl Thought {
    pub fn is_identity_reference(&self) -> bool {
        self.body.is_identity_reference()
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Relation {
    pub id: RelationIdentifier,
    pub kind: RelationKind,
    pub source: RecordIdentifier,
    pub target: RecordIdentifier,
    pub author: ActorName,
    pub occurred_at: TimestampNanos,
    pub note: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum ThoughtBody {
    Observation(ObservationBody),
    Memory(MemoryBody),
    Belief(BeliefBody),
    Goal(GoalBody),
    Claim(ClaimBody),
    Decision(DecisionBody),
    Reference(ReferenceBody),
}

impl ThoughtBody {
    pub const fn kind(&self) -> ThoughtKind {
        match self {
            Self::Observation(_) => ThoughtKind::Observation,
            Self::Memory(_) => ThoughtKind::Memory,
            Self::Belief(_) => ThoughtKind::Belief,
            Self::Goal(_) => ThoughtKind::Goal,
            Self::Claim(_) => ThoughtKind::Claim,
            Self::Decision(_) => ThoughtKind::Decision,
            Self::Reference(_) => ThoughtKind::Reference,
        }
    }

    pub fn is_identity_reference(&self) -> bool {
        match self {
            Self::Reference(reference) => reference.is_identity(),
            _ => false,
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ObservationBody {
    pub summary: ObservationSummary,
    pub detail: Option<TextBody>,
    pub location: Option<RecordIdentifier>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum ObservationSummary {
    ComponentSpawned(ComponentSpawned),
    ComponentReady(ComponentReady),
    ComponentExited(ComponentExited),
    MessageReceived(MessageReceived),
    MessageDelivered(MessageDelivered),
    ChannelGranted(ChannelGranted),
    ChannelRetracted(ChannelRetracted),
    ClaimStarted(ClaimStarted),
    ClaimReleased(ClaimReleased),
    SessionEnded(SessionEnded),
    NoteToSelf(NoteToSelf),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentSpawned {
    pub component: ComponentName,
    pub engine: EngineIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentReady {
    pub component: ComponentName,
    pub engine: EngineIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentExited {
    pub component: ComponentName,
    pub engine: EngineIdentifier,
    pub exit_code: Option<ProcessExitCode>,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct ProcessExitCode(u32);

impl ProcessExitCode {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }
}

impl NotaDecode for ProcessExitCode {
    fn from_nota_block(block: &Block) -> std::result::Result<Self, NotaDecodeError> {
        let value = NotaBlock::new(block).parse_integer()?;
        let value = u32::try_from(value).map_err(|_| NotaDecodeError::InvalidInteger {
            value: value.to_string(),
        })?;
        Ok(Self(value))
    }
}

impl NotaEncode for ProcessExitCode {
    fn to_nota(&self) -> String {
        self.0.to_string()
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct MessageReceived {
    pub channel: ChannelIdentifier,
    pub origin: ActorName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct MessageDelivered {
    pub channel: ChannelIdentifier,
    pub recipient: ActorName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ChannelGranted {
    pub channel: ChannelIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ChannelRetracted {
    pub channel: ChannelIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ClaimStarted {
    pub claim: RecordIdentifier,
    pub role: RoleName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ClaimReleased {
    pub claim: RecordIdentifier,
    pub role: RoleName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionEnded {
    pub session: RecordIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct NoteToSelf {
    pub body: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct MemoryBody {
    pub kind: MemoryKind,
    pub title: TextBody,
    pub summary: TextBody,
    pub boundary: Option<TimeRange>,
    pub role: Option<RecordIdentifier>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum MemoryKind {
    Session(SessionMemory),
    Thread(ThreadMemory),
    IncidentRecord(IncidentMemory),
    Report(ReportMemory),
    Other(OtherMemory),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionMemory {
    pub harness: HarnessIdentifier,
    pub engine: EngineIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ThreadMemory {
    pub topic: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct IncidentMemory {
    pub name: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ReportMemory {
    pub role: RoleName,
    pub number: ReportNumber,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub struct ReportNumber(u32);

impl ReportNumber {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }
}

impl NotaDecode for ReportNumber {
    fn from_nota_block(block: &Block) -> std::result::Result<Self, NotaDecodeError> {
        let value = NotaBlock::new(block).parse_integer()?;
        let value = u32::try_from(value).map_err(|_| NotaDecodeError::InvalidInteger {
            value: value.to_string(),
        })?;
        Ok(Self(value))
    }
}

impl NotaEncode for ReportNumber {
    fn to_nota(&self) -> String {
        self.0.to_string()
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OtherMemory {
    pub kind: TextBody,
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
)]
pub struct TimeRange {
    pub start: TimestampNanos,
    pub end: Option<TimestampNanos>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct BeliefBody {
    pub claim: TextBody,
    pub confidence: Confidence,
    pub status: BeliefStatus,
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
pub enum Confidence {
    Asserted,
    Cited,
    Tested,
    Disputed,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum BeliefStatus {
    Current(CurrentBelief),
    Superseded(SupersededBelief),
    Retracted(RetractedBelief),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentBelief;

impl NotaDecode for CurrentBelief {
    fn from_nota_block(block: &Block) -> std::result::Result<Self, NotaDecodeError> {
        let children =
            NotaBlock::new(block).expect_children(Delimiter::Parenthesis, "CurrentBelief", 1)?;
        let head = children[0]
            .demote_to_string()
            .ok_or(NotaDecodeError::ExpectedAtom {
                type_name: "CurrentBelief",
            })?;
        if head != "CurrentBelief" {
            return Err(NotaDecodeError::UnknownVariant {
                enum_name: "CurrentBelief",
                variant: head.to_owned(),
            });
        }
        Ok(Self)
    }
}

impl NotaEncode for CurrentBelief {
    fn to_nota(&self) -> String {
        Delimiter::Parenthesis.wrap(["CurrentBelief".to_owned()])
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SupersededBelief {
    pub replacement: Option<RecordIdentifier>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RetractedBelief {
    pub reason: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct GoalBody {
    pub description: TextBody,
    pub scope: GoalScope,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum GoalScope {
    Workspace(WorkspaceGoal),
    Project(ProjectGoal),
    Repo(RepoGoal),
    Personal(PersonalGoal),
    Crosscutting(CrosscuttingGoal),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct WorkspaceGoal {
    pub workspace: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ProjectGoal {
    pub project: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RepoGoal {
    pub repo: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PersonalGoal {
    pub actor: ActorName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CrosscuttingGoal {
    pub description: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ClaimBody {
    pub claimed_by: ActorName,
    pub scope: ClaimScope,
    pub role: RoleName,
    pub activity: ClaimActivity,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum ClaimScope {
    Paths(PathClaimScope),
    Tasks(TaskClaimScope),
    Combined(CombinedClaimScope),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PathClaimScope {
    pub paths: Vec<WirePath>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TaskClaimScope {
    pub tasks: Vec<ScopeReference>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CombinedClaimScope {
    pub paths: Vec<WirePath>,
    pub tasks: Vec<ScopeReference>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum ClaimActivity {
    Active(ActiveClaim),
    Paused(PausedClaim),
    Releasing(ReleasingClaim),
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
)]
pub struct ActiveClaim {
    pub started_at: TimestampNanos,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct PausedClaim {
    pub paused_at: TimestampNanos,
    pub reason: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ReleasingClaim {
    pub releasing_at: TimestampNanos,
    pub completion: Option<RecordIdentifier>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DecisionBody {
    pub question: TextBody,
    pub alternatives: Vec<Alternative>,
    pub chosen: AlternativeIdentifier,
    pub criteria: Vec<TextBody>,
    pub rationale: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct Alternative {
    pub id: AlternativeIdentifier,
    pub description: TextBody,
    pub pros: Vec<TextBody>,
    pub cons: Vec<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ReferenceBody {
    pub target: ReferenceTarget,
    pub sense: Option<TextBody>,
}

impl ReferenceBody {
    pub fn is_identity(&self) -> bool {
        matches!(&self.target, ReferenceTarget::Identity(_))
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum ReferenceTarget {
    File(FileReference),
    CodeSymbol(CodeSymbolReference),
    Url(UrlReference),
    Identity(IdentityReference),
    Document(DocumentReferenceTarget),
    BeadsTask(BeadsReference),
    Other(OtherReference),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct FileReference {
    pub path: WirePath,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CodeSymbolReference {
    pub file: WirePath,
    pub symbol: SymbolName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct UrlReference {
    pub url: NormalizedUrl,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum IdentityReference {
    User(UserIdentity),
    Role(RoleIdentity),
    Component(ComponentIdentity),
    Harness(HarnessIdentity),
    Engine(EngineIdentity),
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
)]
pub struct UserIdentity {
    pub uid: UnixUserIdentifier,
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
)]
pub struct RoleIdentity {
    pub role: RoleName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ComponentIdentity {
    pub engine: EngineIdentifier,
    pub component: ComponentName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct HarnessIdentity {
    pub kind: HarnessKind,
    pub id: HarnessIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct EngineIdentity {
    pub engine: EngineIdentifier,
    pub host: HostName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct DocumentReferenceTarget {
    pub document: DocumentReference,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct BeadsReference {
    pub token: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct OtherReference {
    pub kind: TextBody,
    pub body: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubmitThought {
    pub kind: ThoughtKind,
    pub body: ThoughtBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubmitRelation {
    pub kind: RelationKind,
    pub source: RecordIdentifier,
    pub target: RecordIdentifier,
    pub note: Option<TextBody>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct QueryThoughts {
    pub filter: ThoughtFilter,
    pub limit: crate::QueryLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct QueryRelations {
    pub filter: RelationFilter,
    pub limit: crate::QueryLimit,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubscribeThoughts {
    pub filter: ThoughtFilter,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubscribeRelations {
    pub filter: RelationFilter,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum ThoughtFilter {
    ByKind(ByThoughtKind),
    ByAuthor(ByThoughtAuthor),
    ByTimeRange(ByThoughtTimeRange),
    InGoal(InGoal),
    InMemory(InMemory),
    Composite(CompositeThoughtFilter),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ByThoughtKind {
    pub kinds: Vec<ThoughtKind>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ByThoughtAuthor {
    pub author: ActorName,
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
)]
pub struct ByThoughtTimeRange {
    pub start: TimestampNanos,
    pub end: Option<TimestampNanos>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct InGoal {
    pub goal: RecordIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct InMemory {
    pub memory: RecordIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CompositeThoughtFilter {
    pub kinds: Vec<ThoughtKind>,
    pub author: Option<ActorName>,
    pub time_range: Option<ByThoughtTimeRange>,
    pub goal: Option<RecordIdentifier>,
    pub memory: Option<RecordIdentifier>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum RelationFilter {
    ByKind(ByRelationKind),
    BySource(ByRelationSource),
    ByTarget(ByRelationTarget),
    Composite(CompositeRelationFilter),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ByRelationKind {
    pub kinds: Vec<RelationKind>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ByRelationSource {
    pub source: RecordIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ByRelationTarget {
    pub target: RecordIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CompositeRelationFilter {
    pub kinds: Vec<RelationKind>,
    pub source: Option<RecordIdentifier>,
    pub target: Option<RecordIdentifier>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ThoughtCommitted {
    pub record: RecordIdentifier,
    pub display: DisplayIdentifier,
    pub occurred_at: TimestampNanos,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RelationCommitted {
    pub relation: RelationIdentifier,
    pub occurred_at: TimestampNanos,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct ThoughtList {
    pub thoughts: Vec<Thought>,
    pub has_more: bool,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RelationList {
    pub relations: Vec<Relation>,
    pub has_more: bool,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum MindSnapshot {
    Thought(Thought),
    Relation(Relation),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubscriptionAccepted {
    pub subscription: SubscriptionIdentifier,
    pub initial_snapshot: Vec<MindSnapshot>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum MindDelta {
    ThoughtCommitted(Thought),
    RelationCommitted(Relation),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubscriptionEvent {
    pub subscription: SubscriptionIdentifier,
    pub delta: MindDelta,
}

/// Typed acknowledgement that a mind-graph subscription has been retracted.
///
/// Returned in reply to `MindRequest::SubscriptionRetraction(SubscriptionIdentifier)`.
/// Carries the retracted subscription so callers can match the ack to the
/// request they sent. Matches the Path A pattern used by `signal-criome` and
/// `signal-persona-terminal`.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SubscriptionRetracted {
    pub subscription: SubscriptionIdentifier,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct MindRequestUnimplemented {
    pub reason: MindUnimplementedReason,
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
pub enum DependencyKind {
    Router,
    Harness,
    Terminal,
    DurableStore,
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
pub enum ResourceKind {
    SocketPath,
    StateDirectory,
    Database,
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
pub enum MindUnimplementedReason {
    NotInPrototypeScope,
    ChoreographyPolicyMissing,
    DependencyMissing(DependencyKind),
    ResourceUnavailable(ResourceKind),
}
