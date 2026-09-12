#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type WirePath = String;
#[rustfmt::skip]
pub type TaskToken = String;
#[rustfmt::skip]
pub type TimestampNanos = i64;
#[rustfmt::skip]
pub type StableItemIdentifier = String;
#[rustfmt::skip]
pub type DisplayIdentifier = String;
#[rustfmt::skip]
pub type ExternalAlias = String;
#[rustfmt::skip]
pub type BeadsToken = String;
#[rustfmt::skip]
pub type OperationIdentifier = String;
#[rustfmt::skip]
pub type ActorName = String;
#[rustfmt::skip]
pub type EventSeq = i64;
#[rustfmt::skip]
pub type QueryLimit = i64;
#[rustfmt::skip]
pub type Title = String;
#[rustfmt::skip]
pub type TextBody = String;
#[rustfmt::skip]
pub type ReportPath = String;
#[rustfmt::skip]
pub type ReferencePath = String;
#[rustfmt::skip]
pub type CommitHash = String;
#[rustfmt::skip]
pub type HasMore = bool;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Magnitude {
    Zero,
    Minimum,
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
    Maximum,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RoleName {
    Operator,
    OperatorAssistant,
    SecondOperatorAssistant,
    Designer,
    DesignerAssistant,
    SecondDesignerAssistant,
    SystemSpecialist,
    SystemAssistant,
    SecondSystemAssistant,
    Poet,
    PoetAssistant,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ScopeReference {
    Path(WirePath),
    Task(TaskToken),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ItemKind {
    Task,
    Defect,
    Question,
    Decision,
    Note,
    Handoff,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ItemStatus {
    Open,
    InProgress,
    Blocked,
    Closed,
    Deferred,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum EdgeKind {
    DependsOn,
    ParentOf,
    RelatesTo,
    Duplicates,
    Supersedes,
    Answers,
    References,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ItemReference {
    Stable(StableItemIdentifier),
    Display(DisplayIdentifier),
    Alias(ExternalAlias),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ExternalReference {
    Report(ReportPath),
    GitCommit(CommitHash),
    BeadsTask(BeadsToken),
    File(ReferencePath),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum LinkTarget {
    Item(ItemReference),
    External(ExternalReference),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum EdgeTarget {
    Item(StableItemIdentifier),
    External(ExternalReference),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Opening {
    pub item_kind: ItemKind,
    pub magnitude: Magnitude,
    pub title: Title,
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NoteSubmission {
    pub item_reference: ItemReference,
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Link {
    pub item_reference: ItemReference,
    pub edge_kind: EdgeKind,
    pub link_target: LinkTarget,
    pub text_body_option: Option<TextBody>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StatusChange {
    pub item_reference: ItemReference,
    pub item_status: ItemStatus,
    pub text_body_option: Option<TextBody>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AliasAssignment {
    pub item_reference: ItemReference,
    pub external_alias: ExternalAlias,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MemoryQuery {
    pub query_kind: QueryKind,
    pub query_limit: QueryLimit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum QueryKind {
    Ready,
    Blocked,
    Open,
    RecentEvents,
    ByItem(ItemReference),
    ByKind(ItemKind),
    ByStatus(ItemStatus),
    ByAlias(ExternalAlias),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Item {
    pub stable_item_identifier: StableItemIdentifier,
    pub display_identifier: DisplayIdentifier,
    pub external_alias_vector: std::vec::Vec<ExternalAlias>,
    pub item_kind: ItemKind,
    pub item_status: ItemStatus,
    pub magnitude: Magnitude,
    pub title: Title,
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ItemNote {
    pub event_seq: EventSeq,
    pub stable_item_identifier: StableItemIdentifier,
    pub actor_name: ActorName,
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Edge {
    pub event_seq: EventSeq,
    pub stable_item_identifier: StableItemIdentifier,
    pub edge_kind: EdgeKind,
    pub edge_target: EdgeTarget,
    pub text_body_option: Option<TextBody>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EventHeader {
    pub event_seq: EventSeq,
    pub operation_identifier: OperationIdentifier,
    pub actor_name: ActorName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ItemOpenedEvent {
    pub event_header: EventHeader,
    pub item: Item,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NoteAddedEvent {
    pub event_header: EventHeader,
    pub item_note: ItemNote,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EdgeAddedEvent {
    pub event_header: EventHeader,
    pub edge: Edge,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StatusChangedEvent {
    pub event_header: EventHeader,
    pub stable_item_identifier: StableItemIdentifier,
    pub item_status: ItemStatus,
    pub text_body_option: Option<TextBody>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AliasAddedEvent {
    pub event_header: EventHeader,
    pub stable_item_identifier: StableItemIdentifier,
    pub external_alias: ExternalAlias,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Event {
    ItemOpened(ItemOpenedEvent),
    NoteAdded(NoteAddedEvent),
    EdgeAdded(EdgeAddedEvent),
    StatusChanged(StatusChangedEvent),
    AliasAdded(AliasAddedEvent),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OpeningReceipt {
    pub item_opened_event: ItemOpenedEvent,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NoteReceipt {
    pub note_added_event: NoteAddedEvent,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LinkReceipt {
    pub edge_added_event: EdgeAddedEvent,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StatusReceipt {
    pub status_changed_event: StatusChangedEvent,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AliasReceipt {
    pub alias_added_event: AliasAddedEvent,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct View {
    pub item_vector: std::vec::Vec<Item>,
    pub edge_vector: std::vec::Vec<Edge>,
    pub item_note_vector: std::vec::Vec<ItemNote>,
    pub event_vector: std::vec::Vec<Event>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Rejection {
    pub rejection_reason: RejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RejectionReason {
    UnknownItem,
    DuplicateAlias,
    InvalidEdge,
    PersistenceRejected,
    UnsupportedQuery,
    CollisionUnresolved,
}
#[rustfmt::skip]
pub type AdjudicationRequestIdentifier = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ChannelEndpoint {
    Internal(signal_persona::ComponentName),
    External(signal_persona::ConnectionClass),
}
#[rustfmt::skip]
pub type ChannelSource = ChannelEndpoint;
#[rustfmt::skip]
pub type ChannelDestination = ChannelEndpoint;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ChannelMessageKind {
    MessageIngressSubmission,
    MessageSubmission,
    InboxQuery,
    FocusObservation,
    PromptBufferObservation,
    MessageDelivery,
    TerminalInput,
    TerminalCapture,
    TerminalResize,
    TranscriptEvent,
    AdjudicationRequest,
    DeliveryNotification,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ChannelDuration {
    OneShot,
    Permanent,
    TimeBound(TimestampNanos),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AdjudicationSubmission {
    pub adjudication_request_identifier: AdjudicationRequestIdentifier,
    pub message_origin: signal_persona::MessageOrigin,
    pub channel_endpoint: ChannelEndpoint,
    pub channel_message_kind: ChannelMessageKind,
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AdjudicationReceipt {
    pub adjudication_request_identifier: AdjudicationRequestIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ChannelList {
    pub channel_filter_vector: std::vec::Vec<ChannelFilter>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ChannelFilter {
    Source(ChannelEndpoint),
    Destination(ChannelEndpoint),
    Kind(ChannelMessageKind),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ChannelView {
    pub channel_identifier: signal_persona::ChannelIdentifier,
    pub channel_source: ChannelSource,
    pub channel_destination: ChannelDestination,
    pub channel_message_kind_vector: std::vec::Vec<ChannelMessageKind>,
    pub channel_duration: ChannelDuration,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ChannelListView {
    pub channel_view_vector: std::vec::Vec<ChannelView>,
}
#[rustfmt::skip]
pub type RecordIdentifier = String;
#[rustfmt::skip]
pub type SourceRecordIdentifier = RecordIdentifier;
#[rustfmt::skip]
pub type TargetRecordIdentifier = RecordIdentifier;
#[rustfmt::skip]
pub type GoalRecordIdentifier = RecordIdentifier;
#[rustfmt::skip]
pub type MemoryRecordIdentifier = RecordIdentifier;
#[rustfmt::skip]
pub type RelationIdentifier = String;
#[rustfmt::skip]
pub type SubscriptionIdentifier = String;
#[rustfmt::skip]
pub type SubscriptionCursor = i64;
#[rustfmt::skip]
pub type SubscriptionDemandCredit = i64;
#[rustfmt::skip]
pub type SubscriptionBufferBound = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SubscriptionStreamKind {
    Thoughts,
    Relations,
    TechnicalNodes,
    TechnicalRelations,
}
#[rustfmt::skip]
pub type AlternativeIdentifier = String;
#[rustfmt::skip]
pub type SymbolName = String;
#[rustfmt::skip]
pub type NormalizedUrl = String;
#[rustfmt::skip]
pub type DocumentReference = String;
#[rustfmt::skip]
pub type HarnessKind = String;
#[rustfmt::skip]
pub type HarnessIdentifier = String;
#[rustfmt::skip]
pub type ProcessExitCode = i64;
#[rustfmt::skip]
pub type ReportNumber = i64;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ThoughtKind {
    Observation,
    Memory,
    Belief,
    Goal,
    Claim,
    Decision,
    Reference,
}
#[rustfmt::skip]
pub type SourceThoughtKind = ThoughtKind;
#[rustfmt::skip]
pub type TargetThoughtKind = ThoughtKind;
#[rustfmt::skip]
pub type ExpectedSourceThoughtKinds = std::vec::Vec<ThoughtKind>;
#[rustfmt::skip]
pub type ExpectedTargetThoughtKinds = std::vec::Vec<ThoughtKind>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
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
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RelationKindMismatchReason {
    DomainRange,
    AuthoredSourceNotIdentity,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RelationKindMismatch {
    pub relation_kind: RelationKind,
    pub relation_kind_mismatch_reason: RelationKindMismatchReason,
    pub expected_source_thought_kinds: ExpectedSourceThoughtKinds,
    pub expected_target_thought_kinds: ExpectedTargetThoughtKinds,
    pub source_thought_kind: SourceThoughtKind,
    pub target_thought_kind: TargetThoughtKind,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Thought {
    pub record_identifier: RecordIdentifier,
    pub thought_kind: ThoughtKind,
    pub thought_body: ThoughtBody,
    pub actor_name: ActorName,
    pub timestamp_nanos: TimestampNanos,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Relation {
    pub relation_identifier: RelationIdentifier,
    pub relation_kind: RelationKind,
    pub source_record_identifier: SourceRecordIdentifier,
    pub target_record_identifier: TargetRecordIdentifier,
    pub actor_name: ActorName,
    pub timestamp_nanos: TimestampNanos,
    pub text_body_option: Option<TextBody>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ThoughtBody {
    Observation(ObservationBody),
    Memory(MemoryBody),
    Belief(BeliefBody),
    Goal(GoalBody),
    Claim(ClaimBody),
    Decision(DecisionBody),
    Reference(ReferenceBody),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ObservationBody {
    pub observation_summary: ObservationSummary,
    pub text_body_option: Option<TextBody>,
    pub record_identifier_option: Option<RecordIdentifier>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
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
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComponentSpawned {
    pub component_name: signal_persona::ComponentName,
    pub engine_identifier: signal_persona::EngineIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComponentReady {
    pub component_name: signal_persona::ComponentName,
    pub engine_identifier: signal_persona::EngineIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComponentExited {
    pub component_name: signal_persona::ComponentName,
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub process_exit_code_option: Option<ProcessExitCode>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MessageReceived {
    pub channel_identifier: signal_persona::ChannelIdentifier,
    pub actor_name: ActorName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MessageDelivered {
    pub channel_identifier: signal_persona::ChannelIdentifier,
    pub actor_name: ActorName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ChannelGranted {
    pub channel_identifier: signal_persona::ChannelIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ChannelRetracted {
    pub channel_identifier: signal_persona::ChannelIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ClaimStarted {
    pub record_identifier: RecordIdentifier,
    pub role_name: RoleName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ClaimReleased {
    pub record_identifier: RecordIdentifier,
    pub role_name: RoleName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SessionEnded {
    pub record_identifier: RecordIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct NoteToSelf {
    pub text_body: TextBody,
}
#[rustfmt::skip]
pub type MemoryTitle = TextBody;
#[rustfmt::skip]
pub type MemorySummary = TextBody;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MemoryBody {
    pub memory_kind: MemoryKind,
    pub memory_title: MemoryTitle,
    pub memory_summary: MemorySummary,
    pub time_range_option: Option<TimeRange>,
    pub record_identifier_option: Option<RecordIdentifier>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MemoryKind {
    Session(SessionMemory),
    Thread(ThreadMemory),
    IncidentRecord(IncidentMemory),
    Report(ReportMemory),
    Other(OtherMemory),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SessionMemory {
    pub harness_identifier: HarnessIdentifier,
    pub engine_identifier: signal_persona::EngineIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ThreadMemory {
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct IncidentMemory {
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReportMemory {
    pub role_name: RoleName,
    pub report_number: ReportNumber,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OtherMemory {
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TimeRange {
    pub timestamp_nanos: TimestampNanos,
    pub timestamp_nanos_option: Option<TimestampNanos>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BeliefBody {
    pub text_body: TextBody,
    pub confidence: Confidence,
    pub belief_status: BeliefStatus,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Confidence {
    Asserted,
    Cited,
    Tested,
    Disputed,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum BeliefStatus {
    Current,
    Superseded(SupersededBelief),
    Retracted(RetractedBelief),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SupersededBelief {
    pub record_identifier_option: Option<RecordIdentifier>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RetractedBelief {
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct GoalBody {
    pub text_body: TextBody,
    pub goal_scope: GoalScope,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum GoalScope {
    Workspace(WorkspaceGoal),
    Project(ProjectGoal),
    Repo(RepoGoal),
    Personal(PersonalGoal),
    Crosscutting(CrosscuttingGoal),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct WorkspaceGoal {
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ProjectGoal {
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RepoGoal {
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PersonalGoal {
    pub actor_name: ActorName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CrosscuttingGoal {
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ClaimBody {
    pub actor_name: ActorName,
    pub claim_scope: ClaimScope,
    pub role_name: RoleName,
    pub claim_activity: ClaimActivity,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ClaimScope {
    Paths(PathClaimScope),
    Tasks(TaskClaimScope),
    Combined(CombinedClaimScope),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PathClaimScope {
    pub wire_path_vector: std::vec::Vec<WirePath>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TaskClaimScope {
    pub scope_reference_vector: std::vec::Vec<ScopeReference>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CombinedClaimScope {
    pub wire_path_vector: std::vec::Vec<WirePath>,
    pub scope_reference_vector: std::vec::Vec<ScopeReference>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ClaimActivity {
    Active(ActiveClaim),
    Paused(PausedClaim),
    Releasing(ReleasingClaim),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ActiveClaim {
    pub timestamp_nanos: TimestampNanos,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PausedClaim {
    pub timestamp_nanos: TimestampNanos,
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReleasingClaim {
    pub timestamp_nanos: TimestampNanos,
    pub record_identifier_option: Option<RecordIdentifier>,
}
#[rustfmt::skip]
pub type DecisionQuestion = TextBody;
#[rustfmt::skip]
pub type DecisionRationale = TextBody;
#[rustfmt::skip]
pub type DecisionCriteria = std::vec::Vec<TextBody>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DecisionBody {
    pub decision_question: DecisionQuestion,
    pub alternative_vector: std::vec::Vec<Alternative>,
    pub alternative_identifier: AlternativeIdentifier,
    pub decision_criteria: DecisionCriteria,
    pub decision_rationale: DecisionRationale,
}
#[rustfmt::skip]
pub type AlternativeDescription = TextBody;
#[rustfmt::skip]
pub type AlternativePros = std::vec::Vec<TextBody>;
#[rustfmt::skip]
pub type AlternativeCons = std::vec::Vec<TextBody>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Alternative {
    pub alternative_identifier: AlternativeIdentifier,
    pub alternative_description: AlternativeDescription,
    pub alternative_pros: AlternativePros,
    pub alternative_cons: AlternativeCons,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReferenceBody {
    pub reference_target: ReferenceTarget,
    pub text_body_option: Option<TextBody>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ReferenceTarget {
    File(FileReference),
    CodeSymbol(CodeSymbolReference),
    Url(UrlReference),
    Identity(IdentityReference),
    Document(DocumentReferenceTarget),
    BeadsTask(BeadsReference),
    Other(OtherReference),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct FileReference {
    pub wire_path: WirePath,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CodeSymbolReference {
    pub wire_path: WirePath,
    pub symbol_name: SymbolName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UrlReference {
    pub normalized_url: NormalizedUrl,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum IdentityReference {
    User(UserIdentity),
    Role(RoleIdentity),
    Component(ComponentIdentity),
    Harness(HarnessIdentity),
    Engine(EngineIdentity),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UserIdentity {
    pub unix_user_identifier: signal_persona::UnixUserIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RoleIdentity {
    pub role_name: RoleName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComponentIdentity {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub component_name: signal_persona::ComponentName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct HarnessIdentity {
    pub harness_kind: HarnessKind,
    pub harness_identifier: HarnessIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EngineIdentity {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub host_name: signal_persona::HostName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct DocumentReferenceTarget {
    pub document_reference: DocumentReference,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct BeadsReference {
    pub text_body: TextBody,
}
#[rustfmt::skip]
pub type OtherReferenceKind = TextBody;
#[rustfmt::skip]
pub type OtherReferenceBody = TextBody;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct OtherReference {
    pub other_reference_kind: OtherReferenceKind,
    pub other_reference_body: OtherReferenceBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubmitThought {
    pub thought_kind: ThoughtKind,
    pub thought_body: ThoughtBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubmitRelation {
    pub relation_kind: RelationKind,
    pub source_record_identifier: SourceRecordIdentifier,
    pub target_record_identifier: TargetRecordIdentifier,
    pub text_body_option: Option<TextBody>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QueryThoughts {
    pub thought_filter: ThoughtFilter,
    pub query_limit: QueryLimit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QueryRelations {
    pub relation_filter: RelationFilter,
    pub query_limit: QueryLimit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubscribeThoughts {
    pub thought_filter: ThoughtFilter,
    pub subscription_cursor_option: Option<SubscriptionCursor>,
    pub subscription_demand_credit: SubscriptionDemandCredit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubscribeRelations {
    pub relation_filter: RelationFilter,
    pub subscription_cursor_option: Option<SubscriptionCursor>,
    pub subscription_demand_credit: SubscriptionDemandCredit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ThoughtFilter {
    ByKind(ByThoughtKind),
    ByAuthor(ByThoughtAuthor),
    ByTimeRange(ByThoughtTimeRange),
    InGoal(InGoal),
    InMemory(InMemory),
    Composite(CompositeThoughtFilter),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ByThoughtKind {
    pub thought_kind_vector: std::vec::Vec<ThoughtKind>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ByThoughtAuthor {
    pub actor_name: ActorName,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ByThoughtTimeRange {
    pub timestamp_nanos: TimestampNanos,
    pub timestamp_nanos_option: Option<TimestampNanos>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InGoal {
    pub record_identifier: RecordIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InMemory {
    pub record_identifier: RecordIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CompositeThoughtFilter {
    pub thought_kind_vector: std::vec::Vec<ThoughtKind>,
    pub actor_name_option: Option<ActorName>,
    pub by_thought_time_range_option: Option<ByThoughtTimeRange>,
    pub goal_record_identifier_option: Option<GoalRecordIdentifier>,
    pub memory_record_identifier_option: Option<MemoryRecordIdentifier>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RelationFilter {
    ByKind(ByRelationKind),
    BySource(ByRelationSource),
    ByTarget(ByRelationTarget),
    Composite(CompositeRelationFilter),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ByRelationKind {
    pub relation_kind_vector: std::vec::Vec<RelationKind>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ByRelationSource {
    pub record_identifier: RecordIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ByRelationTarget {
    pub record_identifier: RecordIdentifier,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CompositeRelationFilter {
    pub relation_kind_vector: std::vec::Vec<RelationKind>,
    pub source_record_identifier_option: Option<SourceRecordIdentifier>,
    pub target_record_identifier_option: Option<TargetRecordIdentifier>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ThoughtCommitted {
    pub record_identifier: RecordIdentifier,
    pub display_identifier: DisplayIdentifier,
    pub timestamp_nanos: TimestampNanos,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RelationCommitted {
    pub relation_identifier: RelationIdentifier,
    pub timestamp_nanos: TimestampNanos,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ThoughtList {
    pub thought_vector: std::vec::Vec<Thought>,
    pub has_more: HasMore,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RelationList {
    pub relation_vector: std::vec::Vec<Relation>,
    pub has_more: HasMore,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ThoughtStreamAccepted {
    pub subscription_cursor: SubscriptionCursor,
    pub subscription_buffer_bound: SubscriptionBufferBound,
    pub thought_vector: std::vec::Vec<Thought>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RelationStreamAccepted {
    pub subscription_cursor: SubscriptionCursor,
    pub subscription_buffer_bound: SubscriptionBufferBound,
    pub relation_vector: std::vec::Vec<Relation>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalNodeStreamAccepted {
    pub subscription_cursor: SubscriptionCursor,
    pub subscription_buffer_bound: SubscriptionBufferBound,
    pub technical_node_vector: std::vec::Vec<TechnicalNode>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalRelationStreamAccepted {
    pub subscription_cursor: SubscriptionCursor,
    pub subscription_buffer_bound: SubscriptionBufferBound,
    pub technical_relation_vector: std::vec::Vec<TechnicalRelation>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum AcceptedSubscriptionStream {
    Thoughts(ThoughtStreamAccepted),
    Relations(RelationStreamAccepted),
    TechnicalNodes(TechnicalNodeStreamAccepted),
    TechnicalRelations(TechnicalRelationStreamAccepted),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubscriptionAccepted {
    pub subscription_identifier: SubscriptionIdentifier,
    pub accepted_subscription_stream: AcceptedSubscriptionStream,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ThoughtSubscriptionEvent {
    pub subscription_cursor: SubscriptionCursor,
    pub thought: Thought,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RelationSubscriptionEvent {
    pub subscription_cursor: SubscriptionCursor,
    pub relation: Relation,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalNodeSubscriptionEvent {
    pub subscription_cursor: SubscriptionCursor,
    pub technical_node: TechnicalNode,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalRelationSubscriptionEvent {
    pub subscription_cursor: SubscriptionCursor,
    pub technical_relation: TechnicalRelation,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum SubscriptionStreamEvent {
    ThoughtCommitted(ThoughtSubscriptionEvent),
    RelationCommitted(RelationSubscriptionEvent),
    TechnicalNodeCommitted(TechnicalNodeSubscriptionEvent),
    TechnicalRelationCommitted(TechnicalRelationSubscriptionEvent),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubscriptionEvent {
    pub subscription_identifier: SubscriptionIdentifier,
    pub subscription_stream_event: SubscriptionStreamEvent,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubscriptionDemand {
    pub subscription_identifier: SubscriptionIdentifier,
    pub subscription_demand_credit: SubscriptionDemandCredit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubscriptionDemandAccepted {
    pub subscription_identifier: SubscriptionIdentifier,
    pub subscription_demand_credit: SubscriptionDemandCredit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubscriptionRetracted {
    pub subscription_identifier: SubscriptionIdentifier,
    pub subscription_stream_kind: SubscriptionStreamKind,
    pub subscription_cursor: SubscriptionCursor,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MindRequestUnimplemented {
    pub mind_unimplemented_reason: MindUnimplementedReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum DependencyKind {
    Router,
    Harness,
    Terminal,
    DurableStore,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ResourceKind {
    SocketPath,
    StateDirectory,
    Database,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MindUnimplementedReason {
    NotInPrototypeScope,
    ChoreographyPolicyMissing,
    DependencyMissing(DependencyKind),
    ResourceUnavailable(ResourceKind),
}
#[rustfmt::skip]
pub type KnowledgeIdentity = String;
#[rustfmt::skip]
pub type ConflictingKnowledgeIdentities = std::vec::Vec<KnowledgeIdentity>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct KnowledgeSubmission {
    pub domain: signal_domain::Domain,
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AcceptedKnowledge {
    pub knowledge_identity: KnowledgeIdentity,
    pub domain: signal_domain::Domain,
    pub text_body: TextBody,
    pub actor_name: ActorName,
    pub timestamp_nanos: TimestampNanos,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct KnowledgeRecord {
    pub knowledge_identity: KnowledgeIdentity,
    pub domain: signal_domain::Domain,
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct KnowledgeJudgePacket {
    pub domain: signal_domain::Domain,
    pub text_body: TextBody,
    pub accepted_knowledge_vector: std::vec::Vec<AcceptedKnowledge>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct KnowledgeJudgeResponse {
    pub knowledge_judge_verdict: KnowledgeJudgeVerdict,
    pub text_body_option: Option<TextBody>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum KnowledgeJudgeVerdict {
    Accept,
    Reject(KnowledgeRejectionReason),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum KnowledgeRejectionReason {
    NotKnowledge,
    PrivateOrUnauthorized,
    MeaningUnclear,
    SemanticDuplicate(KnowledgeIdentity),
    ConflictsAcceptedKnowledge(ConflictingKnowledgeIdentities),
    WrongDomain(signal_domain::Domain),
    NeedsMoreSpecificShape,
    PersistenceRejected,
}
#[rustfmt::skip]
pub type TechnicalNodeIdentifier = String;
#[rustfmt::skip]
pub type TechnicalRelationIdentifier = String;
#[rustfmt::skip]
pub type TechnicalNodeKey = String;
#[rustfmt::skip]
pub type SourceTechnicalNodeKey = TechnicalNodeKey;
#[rustfmt::skip]
pub type TargetTechnicalNodeKey = TechnicalNodeKey;
#[rustfmt::skip]
pub type StorageTechnicalNodeKey = TechnicalNodeKey;
#[rustfmt::skip]
pub type SchemaFamilyTechnicalNodeKey = TechnicalNodeKey;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
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
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalNodeKeyRejection {
    pub text_body: TextBody,
    pub technical_node_key_rejection_reason: TechnicalNodeKeyRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TechnicalNodeKeyRejectionReason {
    MissingFamilySeparator,
    UnknownFamily,
    WrongSegmentCount,
    EmptySegment,
    InvalidSegmentCharacter,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
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
#[rustfmt::skip]
pub type ExpectedTechnicalNodeKind = TechnicalNodeKind;
#[rustfmt::skip]
pub type GotTechnicalNodeKind = TechnicalNodeKind;
#[rustfmt::skip]
pub type SourceTechnicalNodeKind = TechnicalNodeKind;
#[rustfmt::skip]
pub type TargetTechnicalNodeKind = TechnicalNodeKind;
#[rustfmt::skip]
pub type ExpectedSourceTechnicalNodeKinds = std::vec::Vec<TechnicalNodeKind>;
#[rustfmt::skip]
pub type ExpectedTargetTechnicalNodeKinds = std::vec::Vec<TechnicalNodeKind>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalNodeKindMismatch {
    pub expected_technical_node_kind: ExpectedTechnicalNodeKind,
    pub got_technical_node_kind: GotTechnicalNodeKind,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
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
#[rustfmt::skip]
pub type CrateName = TextBody;
#[rustfmt::skip]
pub type ContractName = TextBody;
#[rustfmt::skip]
pub type StorageResourceName = TextBody;
#[rustfmt::skip]
pub type SchemaFamilyName = TextBody;
#[rustfmt::skip]
pub type SchemaFamilyVersion = TextBody;
#[rustfmt::skip]
pub type TableName = TextBody;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ComponentNode {
    pub component_name: signal_persona::ComponentName,
    pub text_body_option: Option<TextBody>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RepositoryNode {
    pub wire_path: WirePath,
    pub text_body_option: Option<TextBody>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CrateNode {
    pub crate_name: CrateName,
    pub technical_node_key: TechnicalNodeKey,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ContractNode {
    pub contract_name: ContractName,
    pub contract_surface: ContractSurface,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ContractSurface {
    Ordinary,
    Meta,
    Introspection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct WorkItemNode {
    pub task_token: TaskToken,
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SourceArtifactNode {
    pub technical_source_locator: TechnicalSourceLocator,
    pub text_body_option: Option<TextBody>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ReportNode {
    pub wire_path: WirePath,
    pub text_body_option: Option<TextBody>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalClaimNode {
    pub text_body: TextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct WitnessNode {
    pub text_body: TextBody,
    pub technical_source_locator_option: Option<TechnicalSourceLocator>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct StorageResourceNode {
    pub technical_node_key: TechnicalNodeKey,
    pub storage_resource_name: StorageResourceName,
    pub wire_path_option: Option<WirePath>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SchemaFamilyNode {
    pub technical_node_key: TechnicalNodeKey,
    pub schema_family_name: SchemaFamilyName,
    pub schema_family_version_option: Option<SchemaFamilyVersion>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TableNode {
    pub storage_technical_node_key: StorageTechnicalNodeKey,
    pub table_name: TableName,
    pub schema_family_technical_node_key_option: Option<SchemaFamilyTechnicalNodeKey>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TechnicalSourceLocator {
    Path(WirePath),
    Repository(TechnicalNodeKey),
    Task(TaskToken),
    Url(TextBody),
    Report(WirePath),
    Symbol(TextBody),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalNode {
    pub technical_node_identifier: TechnicalNodeIdentifier,
    pub technical_node_key: TechnicalNodeKey,
    pub technical_node_kind: TechnicalNodeKind,
    pub technical_node_body: TechnicalNodeBody,
    pub actor_name: ActorName,
    pub timestamp_nanos: TimestampNanos,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
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
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalRelationKindMismatch {
    pub technical_relation_kind: TechnicalRelationKind,
    pub expected_source_technical_node_kinds: ExpectedSourceTechnicalNodeKinds,
    pub expected_target_technical_node_kinds: ExpectedTargetTechnicalNodeKinds,
    pub source_technical_node_kind: SourceTechnicalNodeKind,
    pub target_technical_node_kind: TargetTechnicalNodeKind,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalRelationEndpoint {
    pub technical_node_identifier: TechnicalNodeIdentifier,
    pub technical_node_key: TechnicalNodeKey,
}
#[rustfmt::skip]
pub type SourceTechnicalRelationEndpoint = TechnicalRelationEndpoint;
#[rustfmt::skip]
pub type TargetTechnicalRelationEndpoint = TechnicalRelationEndpoint;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalRelation {
    pub technical_relation_identifier: TechnicalRelationIdentifier,
    pub technical_relation_kind: TechnicalRelationKind,
    pub source_technical_relation_endpoint: SourceTechnicalRelationEndpoint,
    pub target_technical_relation_endpoint: TargetTechnicalRelationEndpoint,
    pub actor_name: ActorName,
    pub timestamp_nanos: TimestampNanos,
    pub text_body_option: Option<TextBody>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubmitTechnicalNode {
    pub technical_node_key: TechnicalNodeKey,
    pub technical_node_kind: TechnicalNodeKind,
    pub technical_node_body: TechnicalNodeBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubmitTechnicalRelation {
    pub technical_relation_kind: TechnicalRelationKind,
    pub source_technical_node_key: SourceTechnicalNodeKey,
    pub target_technical_node_key: TargetTechnicalNodeKey,
    pub text_body_option: Option<TextBody>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QueryTechnicalNodes {
    pub technical_node_query: TechnicalNodeQuery,
    pub query_limit: QueryLimit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QueryTechnicalRelations {
    pub technical_relation_filter: TechnicalRelationFilter,
    pub query_limit: QueryLimit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubscribeTechnicalNodes {
    pub technical_node_filter: TechnicalNodeFilter,
    pub subscription_cursor_option: Option<SubscriptionCursor>,
    pub subscription_demand_credit: SubscriptionDemandCredit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SubscribeTechnicalRelations {
    pub technical_relation_filter: TechnicalRelationFilter,
    pub subscription_cursor_option: Option<SubscriptionCursor>,
    pub subscription_demand_credit: SubscriptionDemandCredit,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TechnicalNodeQuery {
    Filter(TechnicalNodeFilter),
    About(AboutTechnicalNode),
    RelationNeighborhood(TechnicalRelationNeighborhoodQuery),
    DependencyClosure(TechnicalDependencyClosureQuery),
    ProvenanceChain(TechnicalProvenanceChainQuery),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AboutTechnicalNode {
    pub technical_node_key: TechnicalNodeKey,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TechnicalRelationNeighborhoodDirection {
    Incoming,
    Outgoing,
    Both,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalRelationNeighborhoodQuery {
    pub technical_node_key: TechnicalNodeKey,
    pub technical_relation_neighborhood_direction: TechnicalRelationNeighborhoodDirection,
    pub technical_relation_kind_vector: std::vec::Vec<TechnicalRelationKind>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalDependencyClosureQuery {
    pub technical_node_key: TechnicalNodeKey,
    pub technical_relation_kind_vector: std::vec::Vec<TechnicalRelationKind>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalProvenanceChainQuery {
    pub technical_node_key: TechnicalNodeKey,
    pub technical_relation_kind_vector: std::vec::Vec<TechnicalRelationKind>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TechnicalNodeFilter {
    ByKind(ByTechnicalNodeKind),
    ByStableKey(ByTechnicalNodeStableKey),
    BySourceLocator(ByTechnicalSourceLocator),
    Composite(CompositeTechnicalNodeFilter),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ByTechnicalNodeKind {
    pub technical_node_kind_vector: std::vec::Vec<TechnicalNodeKind>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ByTechnicalNodeStableKey {
    pub technical_node_key: TechnicalNodeKey,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ByTechnicalSourceLocator {
    pub technical_source_locator: TechnicalSourceLocator,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CompositeTechnicalNodeFilter {
    pub technical_node_kind_vector: std::vec::Vec<TechnicalNodeKind>,
    pub technical_node_key_option: Option<TechnicalNodeKey>,
    pub technical_source_locator_option: Option<TechnicalSourceLocator>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TechnicalRelationFilter {
    ByKind(ByTechnicalRelationKind),
    BySource(ByTechnicalRelationSource),
    ByTarget(ByTechnicalRelationTarget),
    BetweenEndpoints(ByTechnicalRelationEndpoints),
    Composite(CompositeTechnicalRelationFilter),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ByTechnicalRelationKind {
    pub technical_relation_kind_vector: std::vec::Vec<TechnicalRelationKind>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ByTechnicalRelationSource {
    pub source_technical_node_key: SourceTechnicalNodeKey,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ByTechnicalRelationTarget {
    pub target_technical_node_key: TargetTechnicalNodeKey,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ByTechnicalRelationEndpoints {
    pub source_technical_node_key: SourceTechnicalNodeKey,
    pub target_technical_node_key: TargetTechnicalNodeKey,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CompositeTechnicalRelationFilter {
    pub technical_relation_kind_vector: std::vec::Vec<TechnicalRelationKind>,
    pub source_technical_node_key_option: Option<SourceTechnicalNodeKey>,
    pub target_technical_node_key_option: Option<TargetTechnicalNodeKey>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalNodeCommitted {
    pub technical_node: TechnicalNode,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalRelationCommitted {
    pub technical_relation: TechnicalRelation,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalNodeList {
    pub technical_node_vector: std::vec::Vec<TechnicalNode>,
    pub has_more: HasMore,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalRelationList {
    pub technical_relation_vector: std::vec::Vec<TechnicalRelation>,
    pub has_more: HasMore,
}
#[rustfmt::skip]
pub type IncomingTechnicalRelations = std::vec::Vec<TechnicalRelation>;
#[rustfmt::skip]
pub type OutgoingTechnicalRelations = std::vec::Vec<TechnicalRelation>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalNodeNeighborhood {
    pub technical_node_option: Option<TechnicalNode>,
    pub incoming_technical_relations: IncomingTechnicalRelations,
    pub outgoing_technical_relations: OutgoingTechnicalRelations,
    pub has_more: HasMore,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalDependencyClosure {
    pub technical_node_option: Option<TechnicalNode>,
    pub technical_node_vector: std::vec::Vec<TechnicalNode>,
    pub technical_relation_vector: std::vec::Vec<TechnicalRelation>,
    pub has_more: HasMore,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalProvenanceChain {
    pub technical_node_option: Option<TechnicalNode>,
    pub technical_node_vector: std::vec::Vec<TechnicalNode>,
    pub technical_relation_vector: std::vec::Vec<TechnicalRelation>,
    pub has_more: HasMore,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalNodeRejected {
    pub technical_node_rejection_reason: TechnicalNodeRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TechnicalNodeRejectionReason {
    InvalidStableNodeKey(TechnicalNodeKeyRejection),
    KindBodyMismatch(TechnicalNodeKindMismatch),
    DuplicateStableNodeKey(TechnicalNodeKey),
    PersistenceRejected,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TechnicalRelationRejected {
    pub technical_relation_rejection_reason: TechnicalRelationRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum TechnicalRelationRejectionReason {
    DuplicateRelation,
    MissingEndpoint(TechnicalNodeKey),
    DomainRangeViolation(TechnicalRelationKindMismatch),
    PersistenceRejected,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    SubmitThought(SubmitThought),
    SubmitRelation(SubmitRelation),
    QueryThoughts(QueryThoughts),
    QueryRelations(QueryRelations),
    SubscribeThoughts(SubscribeThoughts),
    SubscribeRelations(SubscribeRelations),
    SubscriptionRetraction(SubscriptionIdentifier),
    SubscriptionDemand(SubscriptionDemand),
    Opening(Opening),
    NoteSubmission(NoteSubmission),
    Link(Link),
    StatusChange(StatusChange),
    AliasAssignment(AliasAssignment),
    Query(MemoryQuery),
    AdjudicationRequest(AdjudicationSubmission),
    ChannelList(ChannelList),
    SubmitTechnicalNode(SubmitTechnicalNode),
    SubmitTechnicalRelation(SubmitTechnicalRelation),
    QueryTechnicalNodes(QueryTechnicalNodes),
    QueryTechnicalRelations(QueryTechnicalRelations),
    SubscribeTechnicalNodes(SubscribeTechnicalNodes),
    SubscribeTechnicalRelations(SubscribeTechnicalRelations),
    Submit(KnowledgeSubmission),
    Get(KnowledgeIdentity),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    ThoughtCommitted(ThoughtCommitted),
    RelationCommitted(RelationCommitted),
    ThoughtList(ThoughtList),
    RelationList(RelationList),
    SubscriptionAccepted(SubscriptionAccepted),
    SubscriptionRetracted(SubscriptionRetracted),
    SubscriptionDemandAccepted(SubscriptionDemandAccepted),
    SubscriptionDelta(SubscriptionEvent),
    OpeningReceipt(OpeningReceipt),
    NoteReceipt(NoteReceipt),
    LinkReceipt(LinkReceipt),
    StatusReceipt(StatusReceipt),
    AliasReceipt(AliasReceipt),
    View(View),
    Rejection(Rejection),
    AdjudicationReceipt(AdjudicationReceipt),
    ChannelListView(ChannelListView),
    MindRequestUnimplemented(MindRequestUnimplemented),
    TechnicalNodeCommitted(TechnicalNodeCommitted),
    TechnicalRelationCommitted(TechnicalRelationCommitted),
    TechnicalNodeList(TechnicalNodeList),
    TechnicalRelationList(TechnicalRelationList),
    TechnicalNodeNeighborhood(TechnicalNodeNeighborhood),
    TechnicalDependencyClosure(TechnicalDependencyClosure),
    TechnicalProvenanceChain(TechnicalProvenanceChain),
    TechnicalNodeRejected(TechnicalNodeRejected),
    TechnicalRelationRejected(TechnicalRelationRejected),
    Accepted(KnowledgeIdentity),
    Rejected(KnowledgeRejectionReason),
    Found(KnowledgeRecord),
    NotFound,
}
