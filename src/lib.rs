//! Signal contract — `mind` CLI ↔ `mind`.
//!
//! Read this file as the public interface of the central Persona
//! mind channel. The channel carries:
//!
//! - **Memory/work graph** — append typed item, note, edge,
//!   alias, and status events, then query the derived view.
//! - **Typed mind graph substrate** — submit/query/subscribe to
//!   closed Thought and Relation records (`Observation`, `Memory`,
//!   `Belief`, `Goal`, `Claim`, `Decision`, `Reference`) per
//!   designer/152.
//!
//! The channel is **request/reply** (every operation has a
//! typed reply). Long-lived subscription delivery uses the stream grammar
//! declared in this contract.
//!
//! See `ARCHITECTURE.md` for the channel's role and
//! boundaries; `~/primary/skills/contract-repo.md` for the
//! contract-repo discipline this crate follows.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent, NotaTryTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
use signal_persona_origin::{ChannelIdentifier, ComponentName, ConnectionClass, MessageOrigin};
pub use signal_sema::Magnitude;
use std::fmt;
use std::str::FromStr;

mod graph;
pub use graph::*;

// ─── Error ────────────────────────────────────────────────

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("wire path must be absolute and normalized: {path}")]
    InvalidWirePath { path: String },
    #[error("task token must be non-empty, unbracketed, and contain no whitespace: {token}")]
    InvalidTaskToken { token: String },
    #[error("unknown workspace role token: {role}")]
    UnknownRoleName { role: String },
}

// ─── Identity ─────────────────────────────────────────────

/// The closed set of workspace roles. Adding a role is a
/// coordinated schema change — every consumer of this
/// contract recompiles together.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    NotaEnum,
)]
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

impl RoleName {
    pub const ALL: [Self; 11] = [
        Self::Operator,
        Self::OperatorAssistant,
        Self::SecondOperatorAssistant,
        Self::Designer,
        Self::DesignerAssistant,
        Self::SecondDesignerAssistant,
        Self::SystemSpecialist,
        Self::SystemAssistant,
        Self::SecondSystemAssistant,
        Self::Poet,
        Self::PoetAssistant,
    ];

    pub const fn as_wire_token(self) -> &'static str {
        match self {
            Self::Operator => "operator",
            Self::OperatorAssistant => "operator-assistant",
            Self::SecondOperatorAssistant => "second-operator-assistant",
            Self::Designer => "designer",
            Self::DesignerAssistant => "designer-assistant",
            Self::SecondDesignerAssistant => "second-designer-assistant",
            Self::SystemSpecialist => "system-specialist",
            Self::SystemAssistant => "system-assistant",
            Self::SecondSystemAssistant => "second-system-assistant",
            Self::Poet => "poet",
            Self::PoetAssistant => "poet-assistant",
        }
    }

    pub fn from_wire_token(role: impl Into<String>) -> Result<Self> {
        let role = role.into();
        match role.as_str() {
            "operator" => Ok(Self::Operator),
            "operator-assistant" => Ok(Self::OperatorAssistant),
            "second-operator-assistant" => Ok(Self::SecondOperatorAssistant),
            "designer" => Ok(Self::Designer),
            "designer-assistant" => Ok(Self::DesignerAssistant),
            "second-designer-assistant" => Ok(Self::SecondDesignerAssistant),
            "system-specialist" => Ok(Self::SystemSpecialist),
            "system-assistant" => Ok(Self::SystemAssistant),
            "second-system-assistant" => Ok(Self::SecondSystemAssistant),
            "poet" => Ok(Self::Poet),
            "poet-assistant" => Ok(Self::PoetAssistant),
            _ => Err(Error::UnknownRoleName { role }),
        }
    }
}

impl fmt::Display for RoleName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_wire_token())
    }
}

impl FromStr for RoleName {
    type Err = Error;

    fn from_str(role: &str) -> Result<Self> {
        Self::from_wire_token(role)
    }
}

impl TryFrom<String> for RoleName {
    type Error = Error;

    fn try_from(role: String) -> Result<Self> {
        Self::from_wire_token(role)
    }
}

impl TryFrom<&str> for RoleName {
    type Error = Error;

    fn try_from(role: &str) -> Result<Self> {
        Self::from_wire_token(role)
    }
}

// ─── Scope reference ──────────────────────────────────────

/// What's being claimed / observed / acted on.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScopeReference {
    /// An absolute file or directory path.
    Path(WirePath),
    /// A bracketed task token like `[primary-f99]` (stored
    /// without brackets here).
    Task(TaskToken),
}

/// Absolute path, newtyped for cross-platform stability on
/// the wire (per `~/primary/skills/rust-discipline.md`
/// §"Newtype the wire form" — `PathBuf` archives
/// non-deterministically).
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTryTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct WirePath(String);

impl WirePath {
    pub fn try_new(path: String) -> Result<Self> {
        Self::from_absolute_path(path)
    }

    pub fn from_absolute_path(path: impl Into<String>) -> Result<Self> {
        let path = path.into();

        if !path.starts_with('/') || path.split('/').any(|component| component == "..") {
            return Err(Error::InvalidWirePath { path });
        }

        let components = path
            .split('/')
            .filter(|component| !component.is_empty() && *component != ".")
            .collect::<Vec<_>>();
        let normalized = if components.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", components.join("/"))
        };

        Ok(Self(normalized))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for WirePath {
    type Error = Error;

    fn try_from(path: String) -> Result<Self> {
        Self::from_absolute_path(path)
    }
}

impl TryFrom<&str> for WirePath {
    type Error = Error;

    fn try_from(path: &str) -> Result<Self> {
        Self::from_absolute_path(path)
    }
}

impl AsRef<str> for WirePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// A bracketed task identifier (stored without brackets).
/// Bracketed form like `[primary-f99]` is the human surface;
/// the wire carries the raw token.
#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTryTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct TaskToken(String);

impl TaskToken {
    pub fn try_new(token: String) -> Result<Self> {
        Self::from_wire_token(token)
    }

    pub fn from_wire_token(token: impl Into<String>) -> Result<Self> {
        let token = token.into();
        if token.is_empty()
            || token.contains('[')
            || token.contains(']')
            || token.chars().any(char::is_whitespace)
        {
            Err(Error::InvalidTaskToken { token })
        } else {
            Ok(Self(token))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for TaskToken {
    type Error = Error;

    fn try_from(token: String) -> Result<Self> {
        Self::from_wire_token(token)
    }
}

impl TryFrom<&str> for TaskToken {
    type Error = Error;

    fn try_from(token: &str) -> Result<Self> {
        Self::from_wire_token(token)
    }
}

impl AsRef<str> for TaskToken {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// ─── Time ─────────────────────────────────────────────────

/// Nanoseconds since the UNIX epoch. Store-supplied at
/// commit time; never agent-supplied.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    NotaTransparent,
)]
pub struct TimestampNanos(u64);

impl TimestampNanos {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u64 {
        self.0
    }
}

// ─── Mind Memory Identity ─────────────────────────────────

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct StableItemIdentifier(String);

impl StableItemIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct DisplayIdentifier(String);

impl DisplayIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct ExternalAlias(String);

impl ExternalAlias {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct BeadsToken(String);

impl BeadsToken {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct OperationIdentifier(String);

impl OperationIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct ActorName(String);

impl ActorName {
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
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct EventSeq(u64);

impl EventSeq {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaTransparent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub struct QueryLimit(u16);

impl QueryLimit {
    pub fn new(value: u16) -> Self {
        Self(value)
    }

    pub fn into_u16(self) -> u16 {
        self.0
    }
}

// ─── Mind Memory Text ─────────────────────────────────────

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq)]
pub struct Title(String);

impl Title {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq)]
pub struct TextBody(String);

impl TextBody {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct ReportPath(String);

impl ReportPath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct ReferencePath(String);

impl ReferencePath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct CommitHash(String);

impl CommitHash {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ─── Mind Memory Domain ───────────────────────────────────

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum ItemKind {
    Task,
    Defect,
    Question,
    Decision,
    Note,
    Handoff,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum ItemStatus {
    Open,
    InProgress,
    Blocked,
    Closed,
    Deferred,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum EdgeKind {
    DependsOn,
    ParentOf,
    RelatesTo,
    Duplicates,
    Supersedes,
    Answers,
    References,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemReference {
    Stable(StableItemIdentifier),
    Display(DisplayIdentifier),
    Alias(ExternalAlias),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExternalReference {
    Report(ReportPath),
    GitCommit(CommitHash),
    BeadsTask(BeadsToken),
    File(ReferencePath),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LinkTarget {
    Item(ItemReference),
    External(ExternalReference),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq, Hash)]
pub enum EdgeTarget {
    Item(StableItemIdentifier),
    External(ExternalReference),
}

// ─── Mind Memory Requests ─────────────────────────────────

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Opening {
    pub kind: ItemKind,
    pub priority: Magnitude,
    pub title: Title,
    pub body: TextBody,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct NoteSubmission {
    pub item: ItemReference,
    pub body: TextBody,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub source: ItemReference,
    pub kind: EdgeKind,
    pub target: LinkTarget,
    pub body: Option<TextBody>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct StatusChange {
    pub item: ItemReference,
    pub status: ItemStatus,
    pub body: Option<TextBody>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct AliasAssignment {
    pub item: ItemReference,
    pub alias: ExternalAlias,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Query {
    pub kind: QueryKind,
    pub limit: QueryLimit,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
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

// ─── Mind Memory Projections ──────────────────────────────

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub id: StableItemIdentifier,
    pub display_identifier: DisplayIdentifier,
    pub aliases: Vec<ExternalAlias>,
    pub kind: ItemKind,
    pub status: ItemStatus,
    pub priority: Magnitude,
    pub title: Title,
    pub body: TextBody,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Note {
    pub event: EventSeq,
    pub item: StableItemIdentifier,
    pub author: ActorName,
    pub body: TextBody,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    pub event: EventSeq,
    pub source: StableItemIdentifier,
    pub kind: EdgeKind,
    pub target: EdgeTarget,
    pub body: Option<TextBody>,
}

// ─── Mind Memory Events ───────────────────────────────────

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EventHeader {
    pub event: EventSeq,
    pub operation: OperationIdentifier,
    pub actor: ActorName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ItemOpenedEvent {
    pub header: EventHeader,
    pub item: Item,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct NoteAddedEvent {
    pub header: EventHeader,
    pub note: Note,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EdgeAddedEvent {
    pub header: EventHeader,
    pub edge: Edge,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct StatusChangedEvent {
    pub header: EventHeader,
    pub item: StableItemIdentifier,
    pub status: ItemStatus,
    pub body: Option<TextBody>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct AliasAddedEvent {
    pub header: EventHeader,
    pub item: StableItemIdentifier,
    pub alias: ExternalAlias,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum Event {
    ItemOpened(ItemOpenedEvent),
    NoteAdded(NoteAddedEvent),
    EdgeAdded(EdgeAddedEvent),
    StatusChanged(StatusChangedEvent),
    AliasAdded(AliasAddedEvent),
}

// ─── Mind Memory Replies ──────────────────────────────────

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct OpeningReceipt {
    pub event: ItemOpenedEvent,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct NoteReceipt {
    pub event: NoteAddedEvent,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct LinkReceipt {
    pub event: EdgeAddedEvent,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct StatusReceipt {
    pub event: StatusChangedEvent,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct AliasReceipt {
    pub event: AliasAddedEvent,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct View {
    pub items: Vec<Item>,
    pub edges: Vec<Edge>,
    pub notes: Vec<Note>,
    pub events: Vec<Event>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct Rejection {
    pub reason: RejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum RejectionReason {
    UnknownItem,
    DuplicateAlias,
    InvalidEdge,
    PersistenceRejected,
    UnsupportedQuery,
    CollisionUnresolved,
}

// ─── Channel Choreography ─────────────────────────────────

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
)]
pub struct AdjudicationRequestIdentifier(String);

impl AdjudicationRequestIdentifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum ChannelEndpoint {
    Internal(ComponentName),
    External(ConnectionClass),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
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

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum ChannelDuration {
    OneShot,
    Permanent,
    TimeBound(TimestampNanos),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct AdjudicationRequest {
    pub request: AdjudicationRequestIdentifier,
    pub origin: MessageOrigin,
    pub destination: ChannelEndpoint,
    pub kind: ChannelMessageKind,
    pub body_summary: TextBody,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct AdjudicationReceipt {
    pub request: AdjudicationRequestIdentifier,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ChannelList {
    pub filters: Vec<ChannelFilter>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, PartialEq, Eq)]
pub enum ChannelFilter {
    Source(ChannelEndpoint),
    Destination(ChannelEndpoint),
    Kind(ChannelMessageKind),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ChannelView {
    pub channel: ChannelIdentifier,
    pub source: ChannelEndpoint,
    pub destination: ChannelEndpoint,
    pub kinds: Vec<ChannelMessageKind>,
    pub duration: ChannelDuration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct ChannelListView {
    pub channels: Vec<ChannelView>,
}

// ─── Channel declaration ──────────────────────────────────

signal_channel! {
    channel Mind {
        operation SubmitThought(SubmitThought),
        operation SubmitRelation(SubmitRelation),
        operation QueryThoughts(QueryThoughts),
        operation QueryRelations(QueryRelations),
        operation SubscribeThoughts(SubscribeThoughts) opens MindEventStream,
        operation SubscribeRelations(SubscribeRelations) opens MindEventStream,
        operation SubscriptionRetraction(SubscriptionIdentifier),
        operation Opening(Opening),
        operation NoteSubmission(NoteSubmission),
        operation Link(Link),
        operation StatusChange(StatusChange),
        operation AliasAssignment(AliasAssignment),
        operation Query(Query),
        operation AdjudicationRequest(AdjudicationRequest),
        operation ChannelList(ChannelList),
    }
    reply MindReply {
        ThoughtCommitted(ThoughtCommitted),
        RelationCommitted(RelationCommitted),
        ThoughtList(ThoughtList),
        RelationList(RelationList),
        SubscriptionAccepted(SubscriptionAccepted),
        SubscriptionRetracted(SubscriptionRetracted),
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
    }
    event MindEvent {
        SubscriptionDelta(SubscriptionEvent) belongs MindEventStream,
    }
    stream MindEventStream {
        token SubscriptionIdentifier;
        opened SubscriptionAccepted;
        event SubscriptionDelta;
        close SubscriptionRetraction;
    }
}

pub type MindRequest = Operation;
pub type MindFrame = Frame;
pub type MindFrameBody = FrameBody;
pub type MindReplyEnvelope = ReplyEnvelope;
pub type MindRequestBuilder = RequestBuilder;
pub type MindOperationKind = OperationKind;
pub type MindStreamKind = StreamKind;

impl MindRequest {
    pub fn operation_kind(&self) -> MindOperationKind {
        self.kind()
    }
}
