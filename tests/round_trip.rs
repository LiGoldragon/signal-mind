//! Architectural-truth round-trip tests for the
//! `signal-mind` channel.
//!
//! Per `~/primary/skills/architectural-truth-tests.md`,
//! each variant of both enums has a witness test that
//! proves the macro-emitted type round-trips through a
//! length-prefixed Frame.

use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SignalOperationHeads, StreamEventIdentifier, SubReply, SubscriptionTokenInner,
};
use signal_mind::*;
use signal_persona::{
    ChannelIdentifier, ComponentName, ConnectionClass, EngineIdentifier, MessageOrigin,
};

// ─── Helpers ──────────────────────────────────────────────

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn stream_event() -> StreamEventIdentifier {
    StreamEventIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Acceptor,
        LaneSequence::first(),
    )
}

fn round_trip_request(request: MindRequest) -> MindRequest {
    let frame = MindFrame::new(MindFrameBody::Request {
        exchange: exchange(),
        request: request.into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = MindFrame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        MindFrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_reply(reply: MindReply) -> MindReply {
    let frame = MindFrame::new(MindFrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = MindFrame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        MindFrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply operation, got {other:?}"),
    }
}

fn round_trip_event(event: MindEvent) -> MindEvent {
    let frame = MindFrame::new(MindFrameBody::SubscriptionEvent {
        event_identifier: stream_event(),
        token: SubscriptionTokenInner::new(1),
        event,
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = MindFrame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        MindFrameBody::SubscriptionEvent { event, .. } => event,
        other => panic!("expected subscription event, got {other:?}"),
    }
}

fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_nota();
    assert_eq!(encoded, expected);

    let recovered = NotaSource::new(&encoded)
        .parse::<T>()
        .expect("decode nota text");
    assert_eq!(recovered, value);
}

fn sample_path() -> WirePath {
    WirePath::from_absolute_path("/git/github.com/LiGoldragon/signal-mind/src/lib.rs")
        .expect("absolute path")
}

fn sample_task() -> TaskToken {
    TaskToken::from_wire_token("primary-f99").expect("wire task token")
}

fn sample_adjudication_request() -> AdjudicationRequestIdentifier {
    AdjudicationRequestIdentifier::new("aab")
}

fn sample_channel() -> ChannelIdentifier {
    ChannelIdentifier::new("channel-aab")
}

fn sample_record() -> RecordIdentifier {
    RecordIdentifier::new("rec-aab")
}

fn sample_relation() -> RelationIdentifier {
    RelationIdentifier::new("rel-aab")
}

fn sample_engine() -> EngineIdentifier {
    EngineIdentifier::new("engine-aab")
}

fn sample_actor() -> ActorName {
    ActorName::new("operator")
}

fn sample_subscription_demand() -> SubscriptionDemandCredit {
    SubscriptionDemandCredit::new(8)
}

fn sample_subscription_bound() -> SubscriptionBufferBound {
    SubscriptionBufferBound::new(32)
}

fn sample_technical_key(value: &str) -> TechnicalNodeKey {
    TechnicalNodeKey::from_canonical(value).expect("canonical technical node key")
}

fn sample_knowledge_name(value: &str) -> KnowledgeName {
    KnowledgeName::new(value)
}

fn component_identity(value: &str) -> KnowledgeIdentity {
    KnowledgeIdentity::Component(ComponentName::new(value))
}

fn repository_identity(value: &str) -> KnowledgeIdentity {
    KnowledgeIdentity::Repository(sample_knowledge_name(value))
}

fn contract_identity(value: &str, surface: ContractSurface) -> KnowledgeIdentity {
    KnowledgeIdentity::Contract(sample_knowledge_name(value), surface)
}

fn source_identity(value: &str) -> KnowledgeIdentity {
    KnowledgeIdentity::Source(sample_knowledge_name(value))
}

fn sample_mind_component() -> ComponentName {
    ComponentName::new("mind")
}

fn sample_router_component() -> ComponentName {
    ComponentName::new("router")
}

fn sample_internal_endpoint(component: ComponentName) -> ChannelEndpoint {
    ChannelEndpoint::Internal(component)
}

struct MemoryFixture {
    item_id: StableItemIdentifier,
    display_identifier: DisplayIdentifier,
    actor: ActorName,
    operation: OperationIdentifier,
}

impl MemoryFixture {
    fn new() -> Self {
        Self {
            item_id: StableItemIdentifier::new("aab"),
            display_identifier: DisplayIdentifier::new("aab"),
            actor: ActorName::new("operator"),
            operation: OperationIdentifier::new("aab"),
        }
    }

    fn header(&self, event: u64) -> EventHeader {
        EventHeader {
            event: EventSeq::new(event),
            operation: self.operation.clone(),
            actor: self.actor.clone(),
        }
    }

    fn item(&self) -> Item {
        Item {
            id: self.item_id.clone(),
            display_identifier: self.display_identifier.clone(),
            aliases: vec![ExternalAlias::new("primary-aab")],
            kind: ItemKind::Task,
            status: ItemStatus::Open,
            priority: Magnitude::High,
            title: Title::new("Implement native mind memory graph"),
            body: TextBody::new("Replace BEADS with typed Persona mind records."),
        }
    }

    fn opened_event(&self) -> ItemOpenedEvent {
        ItemOpenedEvent {
            header: self.header(1),
            item: self.item(),
        }
    }

    fn note(&self) -> Note {
        Note {
            event: EventSeq::new(2),
            item: self.item_id.clone(),
            author: self.actor.clone(),
            body: TextBody::new("First implementation slice is the contract repo."),
        }
    }

    fn note_event(&self) -> NoteAddedEvent {
        NoteAddedEvent {
            header: self.header(2),
            note: self.note(),
        }
    }

    fn edge(&self) -> Edge {
        Edge {
            event: EventSeq::new(3),
            source: StableItemIdentifier::new("aac"),
            kind: EdgeKind::DependsOn,
            target: EdgeTarget::Item(self.item_id.clone()),
            body: Some(TextBody::new("Implementation waits on the contract.")),
        }
    }

    fn edge_event(&self) -> EdgeAddedEvent {
        EdgeAddedEvent {
            header: self.header(3),
            edge: self.edge(),
        }
    }

    fn status_event(&self) -> StatusChangedEvent {
        StatusChangedEvent {
            header: self.header(4),
            item: self.item_id.clone(),
            status: ItemStatus::Closed,
            body: Some(TextBody::new("Contract shipped.")),
        }
    }

    fn alias_event(&self) -> AliasAddedEvent {
        AliasAddedEvent {
            header: self.header(5),
            item: self.item_id.clone(),
            alias: ExternalAlias::new("primary-aab"),
        }
    }

    fn view(&self) -> View {
        View {
            items: vec![self.item()],
            edges: vec![self.edge()],
            notes: vec![self.note()],
            events: vec![
                Event::ItemOpened(self.opened_event()),
                Event::NoteAdded(self.note_event()),
                Event::EdgeAdded(self.edge_event()),
                Event::StatusChanged(self.status_event()),
                Event::AliasAdded(self.alias_event()),
            ],
        }
    }

    fn assert_request_round_trips(&self, request: MindRequest) {
        let decoded = round_trip_request(request.clone());
        assert_eq!(decoded, request);
    }
}

struct MindGraphFixture {
    record: RecordIdentifier,
    relation: RelationIdentifier,
    actor: ActorName,
    occurred_at: TimestampNanos,
}

impl MindGraphFixture {
    fn new() -> Self {
        Self {
            record: sample_record(),
            relation: sample_relation(),
            actor: sample_actor(),
            occurred_at: TimestampNanos::new(1_790_000_000_000_000_000),
        }
    }

    fn observation_body(&self) -> ThoughtBody {
        ThoughtBody::Observation(ObservationBody {
            summary: ObservationSummary::ComponentReady(ComponentReady {
                component: sample_mind_component(),
                engine: sample_engine(),
            }),
            detail: Some(TextBody::new("mind graph contract ready")),
            location: None,
        })
    }

    fn thought(&self) -> Thought {
        Thought {
            id: self.record.clone(),
            kind: ThoughtKind::Observation,
            body: self.observation_body(),
            author: self.actor.clone(),
            occurred_at: self.occurred_at,
        }
    }

    fn identity_reference_thought(&self) -> Thought {
        Thought {
            id: RecordIdentifier::new("identity-aab"),
            kind: ThoughtKind::Reference,
            body: self.reference_body(),
            author: self.actor.clone(),
            occurred_at: self.occurred_at,
        }
    }

    fn file_reference_thought(&self) -> Thought {
        Thought {
            id: RecordIdentifier::new("file-aab"),
            kind: ThoughtKind::Reference,
            body: ThoughtBody::Reference(ReferenceBody {
                target: ReferenceTarget::File(FileReference {
                    path: sample_path(),
                }),
                sense: Some(TextBody::new("a source file is not an identity")),
            }),
            author: self.actor.clone(),
            occurred_at: self.occurred_at,
        }
    }

    fn relation(&self) -> Relation {
        Relation {
            id: self.relation.clone(),
            kind: RelationKind::Authored,
            source: RecordIdentifier::new("identity-aab"),
            target: self.record.clone(),
            author: self.actor.clone(),
            occurred_at: self.occurred_at,
            note: Some(TextBody::new("operator authored the thought")),
        }
    }

    fn decision_body(&self) -> ThoughtBody {
        ThoughtBody::Decision(DecisionBody {
            question: TextBody::new("Should the mind graph land in the contract first?"),
            alternatives: vec![
                Alternative {
                    id: AlternativeIdentifier::new("contract-first"),
                    description: TextBody::new("Land signal-mind first."),
                    pros: vec![TextBody::new("consumers compile against one vocabulary")],
                    cons: vec![TextBody::new("mind waits for the pin")],
                },
                Alternative {
                    id: AlternativeIdentifier::new("consumer-first"),
                    description: TextBody::new("Prototype in mind first."),
                    pros: vec![TextBody::new("fast local reducer feedback")],
                    cons: vec![TextBody::new("risks a parallel vocabulary")],
                },
            ],
            chosen: AlternativeIdentifier::new("contract-first"),
            criteria: vec![TextBody::new("contracts choreograph parallel work")],
            rationale: TextBody::new("The typed vocabulary must be the shared boundary."),
        })
    }

    fn reference_body(&self) -> ThoughtBody {
        ThoughtBody::Reference(ReferenceBody {
            target: ReferenceTarget::Identity(IdentityReference::Component(ComponentIdentity {
                engine: sample_engine(),
                component: sample_mind_component(),
            })),
            sense: Some(TextBody::new("the component whose graph owns this record")),
        })
    }
}

struct TechnicalFixture {
    actor: ActorName,
    occurred_at: TimestampNanos,
}

impl TechnicalFixture {
    fn new() -> Self {
        Self {
            actor: sample_actor(),
            occurred_at: TimestampNanos::new(1_790_000_000_000_000_000),
        }
    }

    fn component_key(&self) -> TechnicalNodeKey {
        sample_technical_key("component:mind")
    }

    fn repository_key(&self) -> TechnicalNodeKey {
        sample_technical_key("repo:signal-mind")
    }

    fn crate_key(&self) -> TechnicalNodeKey {
        sample_technical_key("crate:signal-mind")
    }

    fn contract_key(&self) -> TechnicalNodeKey {
        sample_technical_key("contract:signal-mind:ordinary")
    }

    fn claim_key(&self) -> TechnicalNodeKey {
        sample_technical_key("claim:technical-memory")
    }

    fn witness_key(&self) -> TechnicalNodeKey {
        sample_technical_key("witness:round-trip")
    }

    fn storage_key(&self) -> TechnicalNodeKey {
        sample_technical_key("storage:mind:sema")
    }

    fn schema_key(&self) -> TechnicalNodeKey {
        sample_technical_key("schema:mind:technical")
    }

    fn component_body(&self) -> TechnicalNodeBody {
        TechnicalNodeBody::Component(ComponentNode {
            component: sample_mind_component(),
            summary: Some(TextBody::new("central mind daemon")),
        })
    }

    fn repository_body(&self) -> TechnicalNodeBody {
        TechnicalNodeBody::Repository(RepositoryNode {
            path: WirePath::from_absolute_path("/git/github.com/LiGoldragon/signal-mind")
                .expect("absolute repository path"),
            remote: Some(TextBody::new("github:LiGoldragon/signal-mind")),
        })
    }

    fn crate_body(&self) -> TechnicalNodeBody {
        TechnicalNodeBody::Crate(CrateNode {
            name: TextBody::new("signal-mind"),
            repository: self.repository_key(),
        })
    }

    fn contract_body(&self) -> TechnicalNodeBody {
        TechnicalNodeBody::Contract(ContractNode {
            name: TextBody::new("mind technical memory contract"),
            surface: ContractSurface::Ordinary,
        })
    }

    fn work_item_body(&self) -> TechnicalNodeBody {
        TechnicalNodeBody::WorkItem(WorkItemNode {
            task: sample_task(),
            title: TextBody::new("Add signal-mind technical contract types"),
        })
    }

    fn source_artifact_body(&self) -> TechnicalNodeBody {
        TechnicalNodeBody::SourceArtifact(SourceArtifactNode {
            locator: TechnicalSourceLocator::Path(sample_path()),
            summary: Some(TextBody::new("technical contract module")),
        })
    }

    fn report_body(&self) -> TechnicalNodeBody {
        TechnicalNodeBody::Report(ReportNode {
            path: WirePath::from_absolute_path(
                "/home/li/primary/reports/operator/technical-memory.md",
            )
            .expect("absolute report path"),
            summary: Some(TextBody::new("technical memory design note")),
        })
    }

    fn claim_body(&self) -> TechnicalNodeBody {
        TechnicalNodeBody::TechnicalClaim(TechnicalClaimNode {
            claim: TextBody::new("technical nodes are separate from ThoughtBody"),
        })
    }

    fn witness_body(&self) -> TechnicalNodeBody {
        TechnicalNodeBody::Witness(WitnessNode {
            summary: TextBody::new("round-trip tests cover technical memory records"),
            locator: Some(TechnicalSourceLocator::Path(sample_path())),
        })
    }

    fn storage_body(&self) -> TechnicalNodeBody {
        TechnicalNodeBody::StorageResource(StorageResourceNode {
            owner: self.component_key(),
            name: TextBody::new("mind sema store"),
            path: Some(
                WirePath::from_absolute_path("/home/li/.local/state/mind/mind.sema")
                    .expect("absolute storage path"),
            ),
        })
    }

    fn schema_body(&self) -> TechnicalNodeBody {
        TechnicalNodeBody::SchemaFamily(SchemaFamilyNode {
            owner: self.component_key(),
            name: TextBody::new("technical dependency memory"),
            version: Some(TextBody::new("2")),
        })
    }

    fn table_body(&self) -> TechnicalNodeBody {
        TechnicalNodeBody::Table(TableNode {
            storage: self.storage_key(),
            name: TextBody::new("technical_nodes"),
            schema_family: Some(self.schema_key()),
        })
    }

    fn node(&self) -> TechnicalNode {
        TechnicalNode {
            identifier: TechnicalNodeIdentifier::new("tn-aab"),
            stable_key: self.component_key(),
            kind: TechnicalNodeKind::Component,
            body: self.component_body(),
            author: self.actor.clone(),
            occurred_at: self.occurred_at,
        }
    }

    fn relation(&self) -> TechnicalRelation {
        TechnicalRelation {
            identifier: TechnicalRelationIdentifier::new("tr-aab"),
            kind: TechnicalRelationKind::DefinesContract,
            source: TechnicalRelationEndpoint {
                identifier: TechnicalNodeIdentifier::new("tn-crate"),
                stable_key: self.crate_key(),
            },
            target: TechnicalRelationEndpoint {
                identifier: TechnicalNodeIdentifier::new("tn-contract"),
                stable_key: self.contract_key(),
            },
            author: self.actor.clone(),
            occurred_at: self.occurred_at,
            note: Some(TextBody::new("crate defines the public contract")),
        }
    }
}

struct KnowledgeFixture {
    actor: ActorName,
    accepted_at: TimestampNanos,
}

impl KnowledgeFixture {
    fn new() -> Self {
        Self {
            actor: sample_actor(),
            accepted_at: TimestampNanos::new(1_790_000_000_000_000_100),
        }
    }

    fn header(&self, identifier: &str, identity: KnowledgeIdentitySlot) -> KnowledgeRecordHeader {
        KnowledgeRecordHeader {
            identifier: KnowledgeIdentifier::new(identifier),
            identity,
            accepted_by: self.actor.clone(),
            accepted_at: self.accepted_at,
        }
    }

    fn component_subject(&self) -> KnowledgeSubject {
        KnowledgeSubject::Component
    }

    fn contract_subject(&self) -> KnowledgeSubject {
        KnowledgeSubject::Contract
    }

    fn component_domain(&self) -> KnowledgeDomain {
        KnowledgeDomain {
            header: self.header(
                "kn-domain-component",
                KnowledgeIdentitySlot::Keyed(KnowledgeIdentity::Domain(
                    KnowledgeSubject::Component,
                )),
            ),
            subject: self.component_subject(),
            name: TextBody::new("component"),
            description: vec![TextBody::new("software component classification")],
        }
    }

    fn mind_entity(&self) -> KnowledgeEntity {
        KnowledgeEntity {
            header: self.header(
                "kn-component-mind",
                KnowledgeIdentitySlot::Keyed(component_identity("mind")),
            ),
            name: TextBody::new("Mind"),
            description: vec![TextBody::new("central accepted knowledge substrate")],
            domains: vec![self.component_subject()],
        }
    }

    fn contract_entity(&self) -> KnowledgeEntity {
        KnowledgeEntity {
            header: self.header(
                "kn-contract-signal-mind",
                KnowledgeIdentitySlot::Keyed(contract_identity(
                    "signal-mind",
                    ContractSurface::Ordinary,
                )),
            ),
            name: TextBody::new("signal-mind ordinary contract"),
            description: vec![TextBody::new("typed Mind public vocabulary")],
            domains: vec![self.contract_subject()],
        }
    }

    fn source(&self) -> KnowledgeSource {
        KnowledgeSource {
            header: self.header(
                "kn-source-architecture",
                KnowledgeIdentitySlot::Keyed(source_identity("signal-mind-architecture")),
            ),
            locator: TextBody::new("/git/github.com/LiGoldragon/signal-mind/ARCHITECTURE.md"),
            description: vec![TextBody::new("signal-mind architecture document")],
        }
    }

    fn statement(&self) -> KnowledgeStatement {
        KnowledgeStatement {
            header: self.header("kn-statement-source-backed", KnowledgeIdentitySlot::Unkeyed),
            body: TextBody::new("Mind accepted knowledge is fixture-driven in v1."),
            about: vec![KnowledgeIdentifier::new("kn-component-mind")],
            domains: vec![self.component_subject()],
        }
    }

    fn relation(&self) -> KnowledgeRelation {
        KnowledgeRelation {
            header: self.header("kn-relation-classified-as", KnowledgeIdentitySlot::Unkeyed),
            kind: KnowledgeRelationKind::ClassifiedAs,
            source: KnowledgeRelationEndpoint {
                identifier: KnowledgeIdentifier::new("kn-component-mind"),
                identity: KnowledgeIdentitySlot::Keyed(component_identity("mind")),
                kind: KnowledgeRecordKind::Entity,
            },
            target: KnowledgeRelationEndpoint {
                identifier: KnowledgeIdentifier::new("kn-domain-component"),
                identity: KnowledgeIdentitySlot::Keyed(KnowledgeIdentity::Domain(
                    KnowledgeSubject::Component,
                )),
                kind: KnowledgeRecordKind::Domain,
            },
            note: Vec::new(),
        }
    }

    fn entity_submission(&self) -> KnowledgeSubmission {
        KnowledgeSubmission {
            candidate: KnowledgeCandidate::Entity(KnowledgeEntityCandidate::Keyed(
                component_identity("mind"),
                TextBody::new("Mind"),
                vec![TextBody::new("accepted knowledge substrate")],
                vec![self.component_subject()],
            )),
            fixture_policy: KnowledgeFixturePolicy::FixtureOnly,
            requester_context: KnowledgeRequesterContext {
                summaries: vec![TextBody::new("fixture accepted entity")],
            },
        }
    }
}

// ─── Mind graph contract variants ─────────────────────────

#[test]
fn every_thought_kind_round_trips_through_nota_text() {
    let cases = [
        (ThoughtKind::Observation, "Observation"),
        (ThoughtKind::Memory, "Memory"),
        (ThoughtKind::Belief, "Belief"),
        (ThoughtKind::Goal, "Goal"),
        (ThoughtKind::Claim, "Claim"),
        (ThoughtKind::Decision, "Decision"),
        (ThoughtKind::Reference, "Reference"),
    ];

    for (kind, expected) in cases {
        round_trip_nota(kind, expected);
    }
}

#[test]
fn every_relation_kind_round_trips_through_nota_text() {
    let cases = [
        (RelationKind::Implements, "Implements"),
        (RelationKind::Realizes, "Realizes"),
        (RelationKind::Requires, "Requires"),
        (RelationKind::Supports, "Supports"),
        (RelationKind::Refutes, "Refutes"),
        (RelationKind::Supersedes, "Supersedes"),
        (RelationKind::Authored, "Authored"),
        (RelationKind::References, "References"),
        (RelationKind::Decides, "Decides"),
        (RelationKind::Considered, "Considered"),
        (RelationKind::Belongs, "Belongs"),
    ];

    for (kind, expected) in cases {
        round_trip_nota(kind, expected);
    }
}

#[test]
fn relation_kind_domain_table_covers_every_relation_kind() {
    let valid_cases = [
        (
            RelationKind::Implements,
            ThoughtKind::Claim,
            ThoughtKind::Goal,
        ),
        (
            RelationKind::Realizes,
            ThoughtKind::Observation,
            ThoughtKind::Claim,
        ),
        (RelationKind::Requires, ThoughtKind::Goal, ThoughtKind::Goal),
        (
            RelationKind::Requires,
            ThoughtKind::Claim,
            ThoughtKind::Claim,
        ),
        (
            RelationKind::Supports,
            ThoughtKind::Observation,
            ThoughtKind::Belief,
        ),
        (
            RelationKind::Supports,
            ThoughtKind::Belief,
            ThoughtKind::Belief,
        ),
        (
            RelationKind::Refutes,
            ThoughtKind::Observation,
            ThoughtKind::Belief,
        ),
        (
            RelationKind::Refutes,
            ThoughtKind::Belief,
            ThoughtKind::Belief,
        ),
        (
            RelationKind::Supersedes,
            ThoughtKind::Decision,
            ThoughtKind::Decision,
        ),
        (
            RelationKind::Authored,
            ThoughtKind::Reference,
            ThoughtKind::Observation,
        ),
        (
            RelationKind::References,
            ThoughtKind::Belief,
            ThoughtKind::Reference,
        ),
        (
            RelationKind::Decides,
            ThoughtKind::Decision,
            ThoughtKind::Goal,
        ),
        (
            RelationKind::Considered,
            ThoughtKind::Decision,
            ThoughtKind::Belief,
        ),
        (
            RelationKind::Belongs,
            ThoughtKind::Observation,
            ThoughtKind::Memory,
        ),
        (RelationKind::Belongs, ThoughtKind::Claim, ThoughtKind::Goal),
    ];

    for relation in RelationKind::ALL {
        assert!(
            valid_cases
                .iter()
                .any(|(candidate, _, _)| *candidate == relation),
            "{relation:?} must have at least one valid witness case",
        );
    }

    for (relation, source, target) in valid_cases {
        relation
            .validate_endpoint_kinds(source, target)
            .unwrap_or_else(|mismatch| panic!("unexpected mismatch: {mismatch:?}"));
    }
}

#[test]
fn relation_kind_rejects_wrong_domain() {
    let mismatch = RelationKind::Implements
        .validate_endpoint_kinds(ThoughtKind::Goal, ThoughtKind::Claim)
        .expect_err("Goal -> Claim cannot implement");

    assert_eq!(mismatch.relation, RelationKind::Implements);
    assert_eq!(mismatch.reason, RelationKindMismatchReason::DomainRange);
    assert_eq!(mismatch.expected_source_kinds, vec![ThoughtKind::Claim]);
    assert_eq!(mismatch.expected_target_kinds, vec![ThoughtKind::Goal]);
    assert_eq!(mismatch.got_source_kind, ThoughtKind::Goal);
    assert_eq!(mismatch.got_target_kind, ThoughtKind::Claim);
}

#[test]
fn authored_relation_rejects_non_identity_reference_source() {
    let fixture = MindGraphFixture::new();
    let source = fixture.file_reference_thought();
    let target = fixture.thought();
    let mismatch = RelationKind::Authored
        .validate_endpoints(&source, &target)
        .expect_err("Authored source must be an identity reference");

    assert_eq!(mismatch.relation, RelationKind::Authored);
    assert_eq!(
        mismatch.reason,
        RelationKindMismatchReason::AuthoredSourceNotIdentity
    );
    assert_eq!(mismatch.expected_source_kinds, vec![ThoughtKind::Reference]);
    assert_eq!(mismatch.got_source_kind, ThoughtKind::Reference);
    assert_eq!(mismatch.got_target_kind, ThoughtKind::Observation);
}

#[test]
fn authored_relation_accepts_identity_reference_source() {
    let fixture = MindGraphFixture::new();
    RelationKind::Authored
        .validate_endpoints(&fixture.identity_reference_thought(), &fixture.thought())
        .expect("identity reference can author any thought");
}

#[test]
fn submit_thought_request_round_trips() {
    let fixture = MindGraphFixture::new();
    let request = MindRequest::SubmitThought(SubmitThought {
        kind: ThoughtKind::Observation,
        body: fixture.observation_body(),
    });

    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn submit_relation_request_round_trips() {
    let request = MindRequest::SubmitRelation(SubmitRelation {
        kind: RelationKind::Implements,
        source: RecordIdentifier::new("claim-aab"),
        target: RecordIdentifier::new("goal-aab"),
        note: Some(TextBody::new("claim commits work toward the goal")),
    });

    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn query_thoughts_request_round_trips_with_composite_filter() {
    let request = MindRequest::QueryThoughts(QueryThoughts {
        filter: ThoughtFilter::Composite(CompositeThoughtFilter {
            kinds: vec![ThoughtKind::Goal, ThoughtKind::Claim],
            author: Some(sample_actor()),
            time_range: None,
            goal: None,
            memory: None,
        }),
        limit: QueryLimit::new(32),
    });

    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn query_relations_request_round_trips_with_source_filter() {
    let request = MindRequest::QueryRelations(QueryRelations {
        filter: RelationFilter::BySource(ByRelationSource {
            source: RecordIdentifier::new("goal-aab"),
        }),
        limit: QueryLimit::new(16),
    });

    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn subscribe_requests_round_trip() {
    let thoughts = MindRequest::SubscribeThoughts(SubscribeThoughts {
        filter: ThoughtFilter::InMemory(InMemory {
            memory: RecordIdentifier::new("memory-aab"),
        }),
        resume_after: Some(SubscriptionCursor::new(41)),
        initial_demand: sample_subscription_demand(),
    });
    let relations = MindRequest::SubscribeRelations(SubscribeRelations {
        filter: RelationFilter::ByTarget(ByRelationTarget {
            target: RecordIdentifier::new("goal-aab"),
        }),
        resume_after: None,
        initial_demand: sample_subscription_demand(),
    });

    assert_eq!(round_trip_request(thoughts.clone()), thoughts);
    assert_eq!(round_trip_request(relations.clone()), relations);
}

#[test]
fn thought_and_relation_replies_round_trip() {
    let fixture = MindGraphFixture::new();
    let replies = vec![
        MindReply::ThoughtCommitted(ThoughtCommitted {
            record: fixture.record.clone(),
            display: DisplayIdentifier::new("aab"),
            occurred_at: fixture.occurred_at,
        }),
        MindReply::RelationCommitted(RelationCommitted {
            relation: fixture.relation.clone(),
            occurred_at: fixture.occurred_at,
        }),
        MindReply::ThoughtList(ThoughtList {
            thoughts: vec![fixture.thought()],
            has_more: false,
        }),
        MindReply::RelationList(RelationList {
            relations: vec![fixture.relation()],
            has_more: false,
        }),
    ];

    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }
}

#[test]
fn subscription_replies_round_trip() {
    let fixture = MindGraphFixture::new();
    let accepted = MindReply::SubscriptionAccepted(SubscriptionAccepted {
        subscription: SubscriptionIdentifier::new("sub-aab"),
        stream: AcceptedSubscriptionStream::Thoughts(ThoughtStreamAccepted {
            cursor: SubscriptionCursor::new(7),
            buffer_bound: sample_subscription_bound(),
            snapshot: vec![fixture.thought()],
        }),
    });
    let retracted = MindReply::SubscriptionRetracted(SubscriptionRetracted {
        subscription: SubscriptionIdentifier::new("sub-aab"),
        stream: SubscriptionStreamKind::Thoughts,
        last_cursor: SubscriptionCursor::new(8),
    });
    let demand = MindRequest::SubscriptionDemand(SubscriptionDemand {
        subscription: SubscriptionIdentifier::new("sub-aab"),
        credit: SubscriptionDemandCredit::new(3),
    });
    let demand_accepted = MindReply::SubscriptionDemandAccepted(SubscriptionDemandAccepted {
        subscription: SubscriptionIdentifier::new("sub-aab"),
        accepted: SubscriptionDemandCredit::new(3),
    });
    let event = MindEvent::SubscriptionDelta(SubscriptionEvent {
        subscription: SubscriptionIdentifier::new("sub-aab"),
        event: SubscriptionStreamEvent::ThoughtCommitted(ThoughtSubscriptionEvent {
            cursor: SubscriptionCursor::new(8),
            thought: Thought {
                body: fixture.decision_body(),
                kind: ThoughtKind::Decision,
                ..fixture.thought()
            },
        }),
    });

    assert_eq!(round_trip_reply(accepted.clone()), accepted);
    assert_eq!(round_trip_reply(retracted.clone()), retracted);
    assert_eq!(round_trip_request(demand.clone()), demand);
    assert_eq!(round_trip_reply(demand_accepted.clone()), demand_accepted);
    assert_eq!(round_trip_event(event.clone()), event);
    assert_eq!(event.stream_kind(), MindStreamKind::MindEventStream);
}

#[test]
fn subscription_stream_payloads_are_typed_by_family() {
    let fixture = MindGraphFixture::new();
    let technical = TechnicalFixture::new();
    let accepted = [
        AcceptedSubscriptionStream::Thoughts(ThoughtStreamAccepted {
            cursor: SubscriptionCursor::new(1),
            buffer_bound: sample_subscription_bound(),
            snapshot: vec![fixture.thought()],
        }),
        AcceptedSubscriptionStream::Relations(RelationStreamAccepted {
            cursor: SubscriptionCursor::new(2),
            buffer_bound: sample_subscription_bound(),
            snapshot: vec![fixture.relation()],
        }),
        AcceptedSubscriptionStream::TechnicalNodes(TechnicalNodeStreamAccepted {
            cursor: SubscriptionCursor::new(3),
            buffer_bound: sample_subscription_bound(),
            snapshot: vec![technical.node()],
        }),
        AcceptedSubscriptionStream::TechnicalRelations(TechnicalRelationStreamAccepted {
            cursor: SubscriptionCursor::new(4),
            buffer_bound: sample_subscription_bound(),
            snapshot: vec![technical.relation()],
        }),
    ];
    let events = [
        SubscriptionStreamEvent::ThoughtCommitted(ThoughtSubscriptionEvent {
            cursor: SubscriptionCursor::new(5),
            thought: fixture.thought(),
        }),
        SubscriptionStreamEvent::RelationCommitted(RelationSubscriptionEvent {
            cursor: SubscriptionCursor::new(6),
            relation: fixture.relation(),
        }),
        SubscriptionStreamEvent::TechnicalNodeCommitted(TechnicalNodeSubscriptionEvent {
            cursor: SubscriptionCursor::new(7),
            node: technical.node(),
        }),
        SubscriptionStreamEvent::TechnicalRelationCommitted(TechnicalRelationSubscriptionEvent {
            cursor: SubscriptionCursor::new(8),
            relation: technical.relation(),
        }),
    ];

    for (index, stream) in accepted.into_iter().enumerate() {
        let reply = MindReply::SubscriptionAccepted(SubscriptionAccepted {
            subscription: SubscriptionIdentifier::new(format!("sub-accepted-{index}")),
            stream,
        });
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }

    for (index, event) in events.into_iter().enumerate() {
        let event = MindEvent::SubscriptionDelta(SubscriptionEvent {
            subscription: SubscriptionIdentifier::new(format!("sub-event-{index}")),
            event,
        });
        assert_eq!(round_trip_event(event.clone()), event);
    }
}

/// The streaming subscription contract pairs `Subscribe*` (opens) with
/// `SubscriptionRetraction` (close request) and `SubscriptionRetracted`
/// (final ack reply). The `signal_channel!` macro emits the
/// `opened_stream()` and `closed_stream()` discriminants from that
/// pairing; this test pins both halves so a future refactor that drops
/// the request-side retract verb in favor of a producer-only close
/// breaks compilation and review.
#[test]
fn subscribe_opens_and_subscription_retraction_closes_the_mind_event_stream() {
    let subscribe_thoughts = MindRequest::SubscribeThoughts(SubscribeThoughts {
        filter: ThoughtFilter::InMemory(InMemory {
            memory: RecordIdentifier::new("memory-aab"),
        }),
        resume_after: Some(SubscriptionCursor::new(9)),
        initial_demand: sample_subscription_demand(),
    });
    let subscribe_relations = MindRequest::SubscribeRelations(SubscribeRelations {
        filter: RelationFilter::ByTarget(ByRelationTarget {
            target: RecordIdentifier::new("goal-aab"),
        }),
        resume_after: None,
        initial_demand: sample_subscription_demand(),
    });
    let subscribe_technical_nodes = MindRequest::SubscribeTechnicalNodes(SubscribeTechnicalNodes {
        filter: TechnicalNodeFilter::ByKind(ByTechnicalNodeKind {
            kinds: vec![TechnicalNodeKind::Component],
        }),
        resume_after: Some(SubscriptionCursor::new(11)),
        initial_demand: sample_subscription_demand(),
    });
    let subscribe_technical_relations =
        MindRequest::SubscribeTechnicalRelations(SubscribeTechnicalRelations {
            filter: TechnicalRelationFilter::ByKind(ByTechnicalRelationKind {
                kinds: vec![TechnicalRelationKind::BuildDependency],
            }),
            resume_after: None,
            initial_demand: sample_subscription_demand(),
        });
    let retract = MindRequest::SubscriptionRetraction(SubscriptionIdentifier::new("sub-aab"));

    assert_eq!(
        subscribe_thoughts.opened_stream(),
        Some(MindStreamKind::MindEventStream),
    );
    assert_eq!(
        subscribe_relations.opened_stream(),
        Some(MindStreamKind::MindEventStream),
    );
    assert_eq!(
        subscribe_technical_nodes.opened_stream(),
        Some(MindStreamKind::MindEventStream),
    );
    assert_eq!(
        subscribe_technical_relations.opened_stream(),
        Some(MindStreamKind::MindEventStream),
    );
    assert_eq!(
        retract.closed_stream(),
        Some(MindStreamKind::MindEventStream),
    );
    assert_eq!(subscribe_thoughts.closed_stream(), None);
    assert_eq!(retract.opened_stream(), None);

    assert_eq!(round_trip_request(retract.clone()), retract);
}

#[test]
fn reference_identity_thought_body_round_trips() {
    let fixture = MindGraphFixture::new();
    let request = MindRequest::SubmitThought(SubmitThought {
        kind: ThoughtKind::Reference,
        body: fixture.reference_body(),
    });

    assert_eq!(round_trip_request(request.clone()), request);
}

#[test]
fn unimplemented_reply_round_trips_as_typed_reply() {
    let cases = [
        (
            MindUnimplementedReason::NotInPrototypeScope,
            "(MindRequestUnimplemented (NotInPrototypeScope))",
        ),
        (
            MindUnimplementedReason::ChoreographyPolicyMissing,
            "(MindRequestUnimplemented (ChoreographyPolicyMissing))",
        ),
        (
            MindUnimplementedReason::DependencyMissing(DependencyKind::Router),
            "(MindRequestUnimplemented ((DependencyMissing Router)))",
        ),
        (
            MindUnimplementedReason::ResourceUnavailable(ResourceKind::Database),
            "(MindRequestUnimplemented ((ResourceUnavailable Database)))",
        ),
    ];

    for (reason, expected_text) in cases {
        let reply = MindReply::MindRequestUnimplemented(MindRequestUnimplemented { reason });
        assert_eq!(round_trip_reply(reply.clone()), reply);
        round_trip_nota(reply, expected_text);
    }
}

// ─── Technical dependency memory contract variants ────────

#[test]
fn every_technical_node_kind_round_trips_through_nota_text() {
    let cases = [
        (TechnicalNodeKind::Component, "Component"),
        (TechnicalNodeKind::Repository, "Repository"),
        (TechnicalNodeKind::Crate, "Crate"),
        (TechnicalNodeKind::Contract, "Contract"),
        (TechnicalNodeKind::WorkItem, "WorkItem"),
        (TechnicalNodeKind::SourceArtifact, "SourceArtifact"),
        (TechnicalNodeKind::Report, "Report"),
        (TechnicalNodeKind::TechnicalClaim, "TechnicalClaim"),
        (TechnicalNodeKind::Witness, "Witness"),
        (TechnicalNodeKind::StorageResource, "StorageResource"),
        (TechnicalNodeKind::SchemaFamily, "SchemaFamily"),
        (TechnicalNodeKind::Table, "Table"),
    ];

    for (kind, expected) in cases {
        round_trip_nota(kind, expected);
    }
}

#[test]
fn every_technical_relation_kind_round_trips_through_nota_text() {
    let cases = [
        (TechnicalRelationKind::OwnsRepository, "OwnsRepository"),
        (TechnicalRelationKind::DefinesContract, "DefinesContract"),
        (TechnicalRelationKind::DefinesCrate, "DefinesCrate"),
        (TechnicalRelationKind::BuildDependency, "BuildDependency"),
        (
            TechnicalRelationKind::RuntimeDependency,
            "RuntimeDependency",
        ),
        (TechnicalRelationKind::WireDependency, "WireDependency"),
        (
            TechnicalRelationKind::StorageDependency,
            "StorageDependency",
        ),
        (TechnicalRelationKind::TaskDependency, "TaskDependency"),
        (
            TechnicalRelationKind::ProvenanceDependency,
            "ProvenanceDependency",
        ),
        (TechnicalRelationKind::Blocks, "Blocks"),
        (TechnicalRelationKind::Implements, "Implements"),
        (TechnicalRelationKind::Documents, "Documents"),
        (TechnicalRelationKind::ClaimsAbout, "ClaimsAbout"),
        (TechnicalRelationKind::ProvenBy, "ProvenBy"),
        (TechnicalRelationKind::Supersedes, "Supersedes"),
        (TechnicalRelationKind::LocatedAt, "LocatedAt"),
    ];

    for (kind, expected) in cases {
        round_trip_nota(kind, expected);
    }
}

#[test]
fn technical_node_body_kind_matches_declared_kind() {
    let fixture = TechnicalFixture::new();
    let valid_bodies = vec![
        (TechnicalNodeKind::Component, fixture.component_body()),
        (TechnicalNodeKind::Repository, fixture.repository_body()),
        (TechnicalNodeKind::Crate, fixture.crate_body()),
        (TechnicalNodeKind::Contract, fixture.contract_body()),
        (TechnicalNodeKind::WorkItem, fixture.work_item_body()),
        (
            TechnicalNodeKind::SourceArtifact,
            fixture.source_artifact_body(),
        ),
        (TechnicalNodeKind::Report, fixture.report_body()),
        (TechnicalNodeKind::TechnicalClaim, fixture.claim_body()),
        (TechnicalNodeKind::Witness, fixture.witness_body()),
        (TechnicalNodeKind::StorageResource, fixture.storage_body()),
        (TechnicalNodeKind::SchemaFamily, fixture.schema_body()),
        (TechnicalNodeKind::Table, fixture.table_body()),
    ];

    assert_eq!(TechnicalNodeKind::ALL.len(), valid_bodies.len());
    for (kind, body) in valid_bodies {
        kind.validate_body(&body)
            .unwrap_or_else(|mismatch| panic!("unexpected mismatch: {mismatch:?}"));
    }
}

#[test]
fn technical_node_kind_rejects_body_mismatch() {
    let fixture = TechnicalFixture::new();
    let mismatch = TechnicalNodeKind::Repository
        .validate_body(&fixture.component_body())
        .expect_err("repository kind cannot carry component body");

    assert_eq!(mismatch.expected_kind, TechnicalNodeKind::Repository);
    assert_eq!(mismatch.got_body_kind, TechnicalNodeKind::Component);
}

#[test]
fn technical_node_key_accepts_canonical_typed_families() {
    let cases = [
        (
            "component:mind",
            TechnicalNodeKeyFamily::Component,
            TechnicalNodeKind::Component,
        ),
        (
            "repo:signal-mind",
            TechnicalNodeKeyFamily::Repository,
            TechnicalNodeKind::Repository,
        ),
        (
            "contract:signal-mind:ordinary",
            TechnicalNodeKeyFamily::Contract,
            TechnicalNodeKind::Contract,
        ),
        (
            "storage:mind:sema",
            TechnicalNodeKeyFamily::StorageResource,
            TechnicalNodeKind::StorageResource,
        ),
        (
            "schema:mind:technical",
            TechnicalNodeKeyFamily::SchemaFamily,
            TechnicalNodeKind::SchemaFamily,
        ),
        (
            "table:mind:technical_nodes",
            TechnicalNodeKeyFamily::Table,
            TechnicalNodeKind::Table,
        ),
    ];

    for (text, family, node_kind) in cases {
        let key = TechnicalNodeKey::from_canonical(text).expect("canonical key");
        assert_eq!(key.as_str(), text);
        assert_eq!(key.family(), family);
        assert_eq!(key.expected_node_kind(), node_kind);
        round_trip_nota(key, text);
    }
}

#[test]
fn technical_node_key_rejects_invalid_canonical_shapes() {
    let cases = [
        (
            "mind",
            TechnicalNodeKeyRejectionReason::MissingFamilySeparator,
        ),
        (
            "repository:signal-mind",
            TechnicalNodeKeyRejectionReason::UnknownFamily,
        ),
        ("component:", TechnicalNodeKeyRejectionReason::EmptySegment),
        (
            "component:Mind",
            TechnicalNodeKeyRejectionReason::InvalidSegmentCharacter,
        ),
        (
            "contract:signal-mind",
            TechnicalNodeKeyRejectionReason::WrongSegmentCount,
        ),
    ];

    for (text, reason) in cases {
        let rejection = TechnicalNodeKey::from_canonical(text).expect_err("invalid key rejected");
        assert_eq!(rejection.supplied_key.as_str(), text);
        assert_eq!(rejection.reason, reason);
    }
}

#[test]
fn technical_relation_kind_domain_table_covers_every_relation_kind() {
    let valid_cases = [
        (
            TechnicalRelationKind::OwnsRepository,
            TechnicalNodeKind::Component,
            TechnicalNodeKind::Repository,
        ),
        (
            TechnicalRelationKind::DefinesContract,
            TechnicalNodeKind::Crate,
            TechnicalNodeKind::Contract,
        ),
        (
            TechnicalRelationKind::DefinesCrate,
            TechnicalNodeKind::Repository,
            TechnicalNodeKind::Crate,
        ),
        (
            TechnicalRelationKind::BuildDependency,
            TechnicalNodeKind::Crate,
            TechnicalNodeKind::Contract,
        ),
        (
            TechnicalRelationKind::RuntimeDependency,
            TechnicalNodeKind::Component,
            TechnicalNodeKind::StorageResource,
        ),
        (
            TechnicalRelationKind::WireDependency,
            TechnicalNodeKind::Component,
            TechnicalNodeKind::Contract,
        ),
        (
            TechnicalRelationKind::StorageDependency,
            TechnicalNodeKind::Component,
            TechnicalNodeKind::Table,
        ),
        (
            TechnicalRelationKind::TaskDependency,
            TechnicalNodeKind::WorkItem,
            TechnicalNodeKind::WorkItem,
        ),
        (
            TechnicalRelationKind::ProvenanceDependency,
            TechnicalNodeKind::TechnicalClaim,
            TechnicalNodeKind::Report,
        ),
        (
            TechnicalRelationKind::Blocks,
            TechnicalNodeKind::WorkItem,
            TechnicalNodeKind::WorkItem,
        ),
        (
            TechnicalRelationKind::Implements,
            TechnicalNodeKind::SourceArtifact,
            TechnicalNodeKind::TechnicalClaim,
        ),
        (
            TechnicalRelationKind::Documents,
            TechnicalNodeKind::Report,
            TechnicalNodeKind::Component,
        ),
        (
            TechnicalRelationKind::ClaimsAbout,
            TechnicalNodeKind::TechnicalClaim,
            TechnicalNodeKind::Contract,
        ),
        (
            TechnicalRelationKind::ProvenBy,
            TechnicalNodeKind::TechnicalClaim,
            TechnicalNodeKind::Witness,
        ),
        (
            TechnicalRelationKind::Supersedes,
            TechnicalNodeKind::Report,
            TechnicalNodeKind::Report,
        ),
        (
            TechnicalRelationKind::LocatedAt,
            TechnicalNodeKind::Crate,
            TechnicalNodeKind::SourceArtifact,
        ),
    ];

    for relation in TechnicalRelationKind::ALL {
        assert!(
            valid_cases
                .iter()
                .any(|(candidate, _, _)| *candidate == relation),
            "{relation:?} must have at least one valid witness case",
        );
    }

    for (relation, source, target) in valid_cases {
        relation
            .validate_endpoint_kinds(source, target)
            .unwrap_or_else(|mismatch| panic!("unexpected mismatch: {mismatch:?}"));
    }
}

#[test]
fn technical_relation_kind_rejects_wrong_domain() {
    let mismatch = TechnicalRelationKind::OwnsRepository
        .validate_endpoint_kinds(TechnicalNodeKind::Report, TechnicalNodeKind::Repository)
        .expect_err("reports cannot own repositories");

    assert_eq!(mismatch.relation, TechnicalRelationKind::OwnsRepository);
    assert_eq!(
        mismatch.expected_source_kinds,
        vec![TechnicalNodeKind::Component]
    );
    assert_eq!(
        mismatch.expected_target_kinds,
        vec![TechnicalNodeKind::Repository]
    );
    assert_eq!(mismatch.got_source_kind, TechnicalNodeKind::Report);
    assert_eq!(mismatch.got_target_kind, TechnicalNodeKind::Repository);
}

// ─── Accepted knowledge contract variants ─────────────────

#[test]
fn every_knowledge_record_kind_round_trips_through_nota_text() {
    let cases = [
        (KnowledgeRecordKind::Entity, "Entity"),
        (KnowledgeRecordKind::Statement, "Statement"),
        (KnowledgeRecordKind::Relation, "Relation"),
        (KnowledgeRecordKind::Domain, "Domain"),
        (KnowledgeRecordKind::Source, "Source"),
    ];

    for (kind, expected) in cases {
        round_trip_nota(kind, expected);
    }
}

#[test]
fn every_knowledge_relation_kind_round_trips_through_nota_text() {
    let cases = [
        (KnowledgeRelationKind::ClassifiedAs, "ClassifiedAs"),
        (KnowledgeRelationKind::BroaderThan, "BroaderThan"),
        (KnowledgeRelationKind::NarrowerThan, "NarrowerThan"),
        (KnowledgeRelationKind::RelatedTo, "RelatedTo"),
        (KnowledgeRelationKind::References, "References"),
        (KnowledgeRelationKind::SupportedBy, "SupportedBy"),
        (KnowledgeRelationKind::Contradicts, "Contradicts"),
        (KnowledgeRelationKind::Supersedes, "Supersedes"),
        (KnowledgeRelationKind::Defines, "Defines"),
        (KnowledgeRelationKind::Implements, "Implements"),
        (KnowledgeRelationKind::DependsOn, "DependsOn"),
    ];

    for (kind, expected) in cases {
        round_trip_nota(kind, expected);
    }
}

#[test]
fn knowledge_identities_round_trip_without_colon_pseudo_keys() {
    round_trip_nota(component_identity("mind"), "(Component mind)");
    round_trip_nota(
        repository_identity("signal-mind"),
        "(Repository signal-mind)",
    );
    round_trip_nota(
        contract_identity("signal-mind", ContractSurface::Ordinary),
        "(Contract (signal-mind Ordinary))",
    );
    round_trip_nota(
        KnowledgeIdentity::Domain(KnowledgeSubject::Component),
        "(Domain Component)",
    );
}

#[test]
fn knowledge_subjects_round_trip_as_typed_domain_atoms() {
    for (subject, expected) in [
        (KnowledgeSubject::Component, "Component"),
        (KnowledgeSubject::Contract, "Contract"),
        (KnowledgeSubject::Repository, "Repository"),
        (KnowledgeSubject::Architecture, "Architecture"),
        (KnowledgeSubject::Interface, "Interface"),
        (KnowledgeSubject::Storage, "Storage"),
        (KnowledgeSubject::Source, "Source"),
    ] {
        round_trip_nota(subject, expected);
    }
}

#[test]
fn knowledge_relation_kind_domain_table_covers_every_relation_kind() {
    let valid_cases = [
        (
            KnowledgeRelationKind::ClassifiedAs,
            KnowledgeRecordKind::Entity,
            KnowledgeRecordKind::Domain,
        ),
        (
            KnowledgeRelationKind::BroaderThan,
            KnowledgeRecordKind::Domain,
            KnowledgeRecordKind::Domain,
        ),
        (
            KnowledgeRelationKind::NarrowerThan,
            KnowledgeRecordKind::Domain,
            KnowledgeRecordKind::Domain,
        ),
        (
            KnowledgeRelationKind::RelatedTo,
            KnowledgeRecordKind::Entity,
            KnowledgeRecordKind::Entity,
        ),
        (
            KnowledgeRelationKind::References,
            KnowledgeRecordKind::Statement,
            KnowledgeRecordKind::Source,
        ),
        (
            KnowledgeRelationKind::SupportedBy,
            KnowledgeRecordKind::Relation,
            KnowledgeRecordKind::Source,
        ),
        (
            KnowledgeRelationKind::Contradicts,
            KnowledgeRecordKind::Statement,
            KnowledgeRecordKind::Statement,
        ),
        (
            KnowledgeRelationKind::Supersedes,
            KnowledgeRecordKind::Statement,
            KnowledgeRecordKind::Statement,
        ),
        (
            KnowledgeRelationKind::Defines,
            KnowledgeRecordKind::Source,
            KnowledgeRecordKind::Entity,
        ),
        (
            KnowledgeRelationKind::Implements,
            KnowledgeRecordKind::Entity,
            KnowledgeRecordKind::Entity,
        ),
        (
            KnowledgeRelationKind::DependsOn,
            KnowledgeRecordKind::Entity,
            KnowledgeRecordKind::Entity,
        ),
    ];

    for relation in KnowledgeRelationKind::ALL {
        assert!(
            valid_cases
                .iter()
                .any(|(candidate, _, _)| *candidate == relation),
            "{relation:?} must have at least one valid witness case",
        );
    }

    for (relation, source, target) in valid_cases {
        relation
            .validate_endpoint_kinds(source, target)
            .unwrap_or_else(|mismatch| panic!("unexpected mismatch: {mismatch:?}"));
    }
}

#[test]
fn knowledge_relation_kind_rejects_wrong_domain() {
    let mismatch = KnowledgeRelationKind::ClassifiedAs
        .validate_endpoint_kinds(KnowledgeRecordKind::Entity, KnowledgeRecordKind::Entity)
        .expect_err("classified-as target must be a domain");

    assert_eq!(mismatch.relation, KnowledgeRelationKind::ClassifiedAs);
    assert_eq!(mismatch.got_source_kind, KnowledgeRecordKind::Entity);
    assert_eq!(mismatch.got_target_kind, KnowledgeRecordKind::Entity);
    assert_eq!(
        mismatch.expected_target_kinds,
        vec![KnowledgeRecordKind::Domain]
    );
}

#[test]
fn knowledge_submit_and_query_requests_round_trip() {
    let fixture = KnowledgeFixture::new();
    let requests = vec![
        MindRequest::SubmitKnowledge(fixture.entity_submission()),
        MindRequest::QueryKnowledge(KnowledgeQuery::GetByIdentifier(KnowledgeIdentifier::new(
            "kn-component-mind",
        ))),
        MindRequest::QueryKnowledge(KnowledgeQuery::GetByIdentity(component_identity("mind"))),
        MindRequest::QueryKnowledge(KnowledgeQuery::ListByKind(
            KnowledgeRecordKind::Entity,
            CurrentView::CurrentOnly,
        )),
        MindRequest::QueryKnowledge(KnowledgeQuery::ListByDomain(
            KnowledgeDomainSelector::Direct(fixture.component_subject()),
            CurrentView::IncludeSuperseded,
        )),
        MindRequest::QueryKnowledge(KnowledgeQuery::ListRelations(
            RelationSelector {
                kind: Some(KnowledgeRelationKind::Defines),
                source: Some(KnowledgeIdentifier::new("kn-repo-signal-mind")),
                target: Some(KnowledgeIdentifier::new("kn-contract-signal-mind")),
                limit: QueryLimit::new(10),
            },
            CurrentView::CurrentOnly,
        )),
    ];

    for request in requests {
        let decoded = round_trip_request(request.clone());
        assert_eq!(decoded, request);
    }
}

#[test]
fn knowledge_verdicts_and_replies_round_trip() {
    let fixture = KnowledgeFixture::new();
    let accepted_records = vec![
        AcceptedKnowledge::Domain(fixture.component_domain()),
        AcceptedKnowledge::Entity(fixture.mind_entity()),
        AcceptedKnowledge::Entity(fixture.contract_entity()),
        AcceptedKnowledge::Statement(fixture.statement()),
        AcceptedKnowledge::Source(fixture.source()),
        AcceptedKnowledge::Relation(fixture.relation()),
    ];

    let accepted_reply = MindReply::KnowledgeAccepted(KnowledgeAccepted {
        accepted: AcceptedKnowledgeView {
            records: accepted_records.clone(),
        },
    });
    assert_eq!(round_trip_reply(accepted_reply.clone()), accepted_reply);

    let list_reply = MindReply::KnowledgeList(KnowledgeList {
        records: accepted_records,
        has_more: false,
    });
    assert_eq!(round_trip_reply(list_reply.clone()), list_reply);

    let rejected_reply = MindReply::KnowledgeRejected(KnowledgeRejection {
        reason: KnowledgeRejectionReason::ConflictsAcceptedKnowledge(vec![
            KnowledgeIdentifier::new("kn-statement-source-backed"),
        ]),
        candidate_summary: CandidateSummary {
            summary: TextBody::new("contradictory fixture candidate"),
        },
        retry_hint: None,
    });
    assert_eq!(round_trip_reply(rejected_reply.clone()), rejected_reply);

    let verdict = KnowledgeJudgeVerdict::Accept;
    let decoded = NotaSource::new(&verdict.to_nota())
        .parse::<KnowledgeJudgeVerdict>()
        .expect("decode knowledge verdict");
    assert_eq!(decoded, verdict);
}

#[test]
fn technical_node_requests_round_trip() {
    let fixture = TechnicalFixture::new();
    let requests = vec![
        MindRequest::SubmitTechnicalNode(SubmitTechnicalNode {
            stable_key: fixture.component_key(),
            kind: TechnicalNodeKind::Component,
            body: fixture.component_body(),
        }),
        MindRequest::QueryTechnicalNodes(QueryTechnicalNodes {
            query: TechnicalNodeQuery::Filter(TechnicalNodeFilter::Composite(
                CompositeTechnicalNodeFilter {
                    kinds: vec![TechnicalNodeKind::Component, TechnicalNodeKind::Crate],
                    stable_key: Some(fixture.component_key()),
                    source_locator: Some(TechnicalSourceLocator::Path(sample_path())),
                },
            )),
            limit: QueryLimit::new(25),
        }),
        MindRequest::QueryTechnicalNodes(QueryTechnicalNodes {
            query: TechnicalNodeQuery::About(AboutTechnicalNode {
                stable_key: fixture.component_key(),
            }),
            limit: QueryLimit::new(25),
        }),
        MindRequest::QueryTechnicalNodes(QueryTechnicalNodes {
            query: TechnicalNodeQuery::RelationNeighborhood(TechnicalRelationNeighborhoodQuery {
                stable_key: fixture.component_key(),
                direction: TechnicalRelationNeighborhoodDirection::Both,
                kinds: vec![TechnicalRelationKind::RuntimeDependency],
            }),
            limit: QueryLimit::new(25),
        }),
        MindRequest::QueryTechnicalNodes(QueryTechnicalNodes {
            query: TechnicalNodeQuery::DependencyClosure(TechnicalDependencyClosureQuery {
                stable_key: fixture.component_key(),
                kinds: vec![
                    TechnicalRelationKind::BuildDependency,
                    TechnicalRelationKind::RuntimeDependency,
                    TechnicalRelationKind::WireDependency,
                    TechnicalRelationKind::StorageDependency,
                    TechnicalRelationKind::TaskDependency,
                ],
            }),
            limit: QueryLimit::new(25),
        }),
        MindRequest::QueryTechnicalNodes(QueryTechnicalNodes {
            query: TechnicalNodeQuery::ProvenanceChain(TechnicalProvenanceChainQuery {
                stable_key: fixture.claim_key(),
                kinds: vec![
                    TechnicalRelationKind::ProvenanceDependency,
                    TechnicalRelationKind::ProvenBy,
                ],
            }),
            limit: QueryLimit::new(25),
        }),
        MindRequest::SubscribeTechnicalNodes(SubscribeTechnicalNodes {
            filter: TechnicalNodeFilter::BySourceLocator(ByTechnicalSourceLocator {
                locator: TechnicalSourceLocator::Repository(fixture.repository_key()),
            }),
            resume_after: Some(SubscriptionCursor::new(13)),
            initial_demand: sample_subscription_demand(),
        }),
    ];

    for request in requests {
        assert_eq!(round_trip_request(request.clone()), request);
    }
}

#[test]
fn technical_relation_requests_round_trip() {
    let fixture = TechnicalFixture::new();
    let requests = vec![
        MindRequest::SubmitTechnicalRelation(SubmitTechnicalRelation {
            kind: TechnicalRelationKind::DefinesContract,
            source: fixture.crate_key(),
            target: fixture.contract_key(),
            note: Some(TextBody::new("crate defines the public contract")),
        }),
        MindRequest::QueryTechnicalRelations(QueryTechnicalRelations {
            filter: TechnicalRelationFilter::BetweenEndpoints(ByTechnicalRelationEndpoints {
                source: fixture.crate_key(),
                target: fixture.contract_key(),
            }),
            limit: QueryLimit::new(10),
        }),
        MindRequest::SubscribeTechnicalRelations(SubscribeTechnicalRelations {
            filter: TechnicalRelationFilter::Composite(CompositeTechnicalRelationFilter {
                kinds: vec![
                    TechnicalRelationKind::ProvenanceDependency,
                    TechnicalRelationKind::Blocks,
                ],
                source: Some(fixture.claim_key()),
                target: Some(fixture.witness_key()),
            }),
            resume_after: None,
            initial_demand: sample_subscription_demand(),
        }),
    ];

    for request in requests {
        assert_eq!(round_trip_request(request.clone()), request);
    }
}

#[test]
fn technical_replies_and_events_round_trip() {
    let fixture = TechnicalFixture::new();
    let node = fixture.node();
    let relation = fixture.relation();
    let replies = vec![
        MindReply::TechnicalNodeCommitted(TechnicalNodeCommitted { node: node.clone() }),
        MindReply::TechnicalRelationCommitted(TechnicalRelationCommitted {
            relation: relation.clone(),
        }),
        MindReply::TechnicalNodeList(TechnicalNodeList {
            nodes: vec![node.clone()],
            has_more: false,
        }),
        MindReply::TechnicalRelationList(TechnicalRelationList {
            relations: vec![relation.clone()],
            has_more: false,
        }),
        MindReply::TechnicalNodeNeighborhood(TechnicalNodeNeighborhood {
            center: Some(node.clone()),
            incoming: vec![],
            outgoing: vec![relation.clone()],
            has_more: false,
        }),
        MindReply::TechnicalDependencyClosure(TechnicalDependencyClosure {
            root: Some(node.clone()),
            nodes: vec![node.clone()],
            relations: vec![relation.clone()],
            has_more: false,
        }),
        MindReply::TechnicalProvenanceChain(TechnicalProvenanceChain {
            root: Some(node.clone()),
            nodes: vec![node.clone()],
            relations: vec![relation.clone()],
            has_more: false,
        }),
        MindReply::TechnicalNodeRejected(TechnicalNodeRejected {
            reason: TechnicalNodeRejectionReason::DuplicateStableNodeKey(fixture.component_key()),
        }),
        MindReply::TechnicalNodeRejected(TechnicalNodeRejected {
            reason: TechnicalNodeRejectionReason::InvalidStableNodeKey(
                TechnicalNodeKeyRejection::new(
                    "component:Mind",
                    TechnicalNodeKeyRejectionReason::InvalidSegmentCharacter,
                ),
            ),
        }),
        MindReply::TechnicalRelationRejected(TechnicalRelationRejected {
            reason: TechnicalRelationRejectionReason::DomainRangeViolation(
                TechnicalRelationKindMismatch {
                    relation: TechnicalRelationKind::OwnsRepository,
                    expected_source_kinds: vec![TechnicalNodeKind::Component],
                    expected_target_kinds: vec![TechnicalNodeKind::Repository],
                    got_source_kind: TechnicalNodeKind::Report,
                    got_target_kind: TechnicalNodeKind::Repository,
                },
            ),
        }),
    ];

    for reply in replies {
        assert_eq!(round_trip_reply(reply.clone()), reply);
    }

    let accepted = MindReply::SubscriptionAccepted(SubscriptionAccepted {
        subscription: SubscriptionIdentifier::new("sub-technical"),
        stream: AcceptedSubscriptionStream::TechnicalNodes(TechnicalNodeStreamAccepted {
            cursor: SubscriptionCursor::new(17),
            buffer_bound: sample_subscription_bound(),
            snapshot: vec![node.clone()],
        }),
    });
    let event = MindEvent::SubscriptionDelta(SubscriptionEvent {
        subscription: SubscriptionIdentifier::new("sub-technical"),
        event: SubscriptionStreamEvent::TechnicalNodeCommitted(TechnicalNodeSubscriptionEvent {
            cursor: SubscriptionCursor::new(18),
            node,
        }),
    });

    assert_eq!(round_trip_reply(accepted.clone()), accepted);
    assert_eq!(round_trip_event(event.clone()), event);
    assert_eq!(event.stream_kind(), MindStreamKind::MindEventStream);
}

// ─── Request variants ─────────────────────────────────────

#[test]
fn role_name_parses_workspace_coordination_tokens() {
    let cases = [
        ("operator", RoleName::Operator),
        ("operator-assistant", RoleName::OperatorAssistant),
        (
            "second-operator-assistant",
            RoleName::SecondOperatorAssistant,
        ),
        ("designer", RoleName::Designer),
        ("designer-assistant", RoleName::DesignerAssistant),
        (
            "second-designer-assistant",
            RoleName::SecondDesignerAssistant,
        ),
        ("system-specialist", RoleName::SystemSpecialist),
        ("system-assistant", RoleName::SystemAssistant),
        ("second-system-assistant", RoleName::SecondSystemAssistant),
        ("poet", RoleName::Poet),
        ("poet-assistant", RoleName::PoetAssistant),
    ];

    assert_eq!(RoleName::ALL.len(), cases.len());
    for (token, role) in cases {
        assert_eq!(RoleName::from_wire_token(token), Ok(role));
        assert_eq!(token.parse::<RoleName>(), Ok(role));
        assert_eq!(role.as_wire_token(), token);
        assert_eq!(role.to_string(), token);
    }
}

#[test]
fn role_name_rejects_unregistered_workspace_roles() {
    assert!(RoleName::from_wire_token("").is_err());
    assert!(RoleName::from_wire_token("operator assistant").is_err());
    assert!(RoleName::from_wire_token("Operator").is_err());
    assert!(RoleName::from_wire_token("critic").is_err());
}

#[test]
fn open_request_round_trips_through_length_prefixed_frame() {
    let request = MindRequest::Opening(Opening {
        kind: ItemKind::Task,
        priority: Magnitude::High,
        title: Title::new("Replace BEADS"),
        body: TextBody::new("Open a typed mind item."),
    });
    let decoded = round_trip_request(request.clone());
    assert_eq!(decoded, request);
}

#[test]
fn add_note_request_round_trips() {
    let request = MindRequest::NoteSubmission(NoteSubmission {
        item: ItemReference::Display(DisplayIdentifier::new("aab")),
        body: TextBody::new("Append-only note."),
    });
    let decoded = round_trip_request(request.clone());
    assert_eq!(decoded, request);
}

#[test]
fn link_request_round_trips_with_typed_edge_kind() {
    let request = MindRequest::Link(Link {
        source: ItemReference::Display(DisplayIdentifier::new("abc")),
        kind: EdgeKind::DependsOn,
        target: LinkTarget::Item(ItemReference::Display(DisplayIdentifier::new("aab"))),
        body: None,
    });
    let decoded = round_trip_request(request.clone());
    assert_eq!(decoded, request);
}

#[test]
fn link_request_round_trips_with_external_report_reference() {
    let request = MindRequest::Link(Link {
        source: ItemReference::Display(DisplayIdentifier::new("aab")),
        kind: EdgeKind::References,
        target: LinkTarget::External(ExternalReference::Report(ReportPath::new(
            "reports/operator/100-mind-central-rename-plan.md",
        ))),
        body: Some(TextBody::new("Research basis for this work item.")),
    });
    let decoded = round_trip_request(request.clone());
    assert_eq!(decoded, request);
}

#[test]
fn status_change_request_round_trips() {
    let request = MindRequest::StatusChange(StatusChange {
        item: ItemReference::Alias(ExternalAlias::new("primary-aab")),
        status: ItemStatus::InProgress,
        body: Some(TextBody::new("Operator started it.")),
    });
    let decoded = round_trip_request(request.clone());
    assert_eq!(decoded, request);
}

#[test]
fn add_alias_request_round_trips() {
    let request = MindRequest::AliasAssignment(AliasAssignment {
        item: ItemReference::Stable(StableItemIdentifier::new("aab")),
        alias: ExternalAlias::new("primary-aab"),
    });
    let decoded = round_trip_request(request.clone());
    assert_eq!(decoded, request);
}

#[test]
fn every_query_kind_round_trips() {
    let fixture = MemoryFixture::new();
    let kinds = vec![
        QueryKind::Ready,
        QueryKind::Blocked,
        QueryKind::Open,
        QueryKind::RecentEvents,
        QueryKind::ByItem(ItemReference::Stable(fixture.item_id.clone())),
        QueryKind::ByKind(ItemKind::Decision),
        QueryKind::ByStatus(ItemStatus::Closed),
        QueryKind::ByAlias(ExternalAlias::new("primary-aab")),
    ];

    for kind in kinds {
        fixture.assert_request_round_trips(MindRequest::Query(Query {
            kind,
            limit: QueryLimit::new(25),
        }));
    }
}

#[test]
fn query_request_round_trips_through_nota_text() {
    round_trip_nota(
        MindRequest::Query(Query {
            kind: QueryKind::Ready,
            limit: QueryLimit::new(25),
        }),
        "(Query (Ready 25))",
    );
}

#[test]
fn every_edge_kind_round_trips_as_a_link_request() {
    let fixture = MemoryFixture::new();
    let kinds = vec![
        EdgeKind::DependsOn,
        EdgeKind::ParentOf,
        EdgeKind::RelatesTo,
        EdgeKind::Duplicates,
        EdgeKind::Supersedes,
        EdgeKind::Answers,
        EdgeKind::References,
    ];

    for kind in kinds {
        fixture.assert_request_round_trips(MindRequest::Link(Link {
            source: ItemReference::Stable(StableItemIdentifier::new("aac")),
            kind,
            target: LinkTarget::Item(ItemReference::Stable(fixture.item_id.clone())),
            body: None,
        }));
    }
}

#[test]
fn every_external_reference_variant_round_trips_as_a_link_target() {
    let fixture = MemoryFixture::new();
    let targets = vec![
        ExternalReference::Report(ReportPath::new("reports/operator/100-mind.md")),
        ExternalReference::GitCommit(CommitHash::new("7f0bf022")),
        ExternalReference::BeadsTask(BeadsToken::new("primary-aab")),
        ExternalReference::File(ReferencePath::new(
            "/git/github.com/LiGoldragon/mind/src/lib.rs",
        )),
    ];

    for target in targets {
        fixture.assert_request_round_trips(MindRequest::Link(Link {
            source: ItemReference::Stable(fixture.item_id.clone()),
            kind: EdgeKind::References,
            target: LinkTarget::External(target),
            body: Some(TextBody::new("typed external reference")),
        }));
    }
}

#[test]
fn adjudication_request_round_trips() {
    let request = MindRequest::AdjudicationRequest(AdjudicationRequest {
        request: sample_adjudication_request(),
        origin: MessageOrigin::local_connection(ConnectionClass::Owner),
        destination: sample_internal_endpoint(sample_router_component()),
        kind: ChannelMessageKind::MessageSubmission,
        body_summary: TextBody::new("owner asks router to deliver a prompt"),
    });
    let decoded = round_trip_request(request.clone());
    assert_eq!(decoded, request);
}

#[test]
fn channel_choreography_requests_round_trip() {
    let requests = vec![
        MindRequest::AdjudicationRequest(AdjudicationRequest {
            request: sample_adjudication_request(),
            origin: MessageOrigin::local_connection(ConnectionClass::Owner),
            destination: sample_internal_endpoint(sample_router_component()),
            kind: ChannelMessageKind::MessageSubmission,
            body_summary: TextBody::new("owner asks router to deliver a prompt"),
        }),
        MindRequest::ChannelList(ChannelList {
            filters: vec![
                ChannelFilter::Source(sample_internal_endpoint(sample_mind_component())),
                ChannelFilter::Destination(sample_internal_endpoint(sample_router_component())),
                ChannelFilter::Kind(ChannelMessageKind::MessageDelivery),
            ],
        }),
    ];

    for request in requests {
        let decoded = round_trip_request(request.clone());
        assert_eq!(decoded, request);
    }
}

#[test]
fn message_ingress_kind_is_distinct_from_generic_message_submission() {
    assert_ne!(
        ChannelMessageKind::MessageIngressSubmission,
        ChannelMessageKind::MessageSubmission
    );
    round_trip_nota(
        ChannelMessageKind::MessageIngressSubmission,
        "MessageIngressSubmission",
    );
    round_trip_nota(ChannelMessageKind::MessageSubmission, "MessageSubmission");
}

#[test]
fn channel_message_kinds_do_not_model_router_owner_orders() {
    let forbidden = [
        "ChannelGrant",
        "ChannelExtend",
        "ChannelRetract",
        "AdjudicationDeny",
        "AdjudicationDenial",
    ];

    let allowed = [
        ChannelMessageKind::MessageIngressSubmission,
        ChannelMessageKind::MessageSubmission,
        ChannelMessageKind::InboxQuery,
        ChannelMessageKind::FocusObservation,
        ChannelMessageKind::PromptBufferObservation,
        ChannelMessageKind::MessageDelivery,
        ChannelMessageKind::TerminalInput,
        ChannelMessageKind::TerminalCapture,
        ChannelMessageKind::TerminalResize,
        ChannelMessageKind::TranscriptEvent,
        ChannelMessageKind::AdjudicationRequest,
        ChannelMessageKind::DeliveryNotification,
    ];

    for kind in allowed {
        let encoded = kind.to_nota();
        assert!(!forbidden.contains(&encoded.as_str()));
    }
}

#[test]
fn mind_request_exposes_contract_owned_operation_kind() {
    let fixture = MemoryFixture::new();
    let cases = vec![
        (
            MindRequest::SubmitThought(SubmitThought {
                kind: ThoughtKind::Observation,
                body: MindGraphFixture::new().observation_body(),
            }),
            MindOperationKind::SubmitThought,
        ),
        (
            MindRequest::SubmitRelation(SubmitRelation {
                kind: RelationKind::Implements,
                source: RecordIdentifier::new("claim-aab"),
                target: RecordIdentifier::new("goal-aab"),
                note: None,
            }),
            MindOperationKind::SubmitRelation,
        ),
        (
            MindRequest::QueryThoughts(QueryThoughts {
                filter: ThoughtFilter::ByKind(ByThoughtKind {
                    kinds: vec![ThoughtKind::Goal],
                }),
                limit: QueryLimit::new(10),
            }),
            MindOperationKind::QueryThoughts,
        ),
        (
            MindRequest::QueryRelations(QueryRelations {
                filter: RelationFilter::ByKind(ByRelationKind {
                    kinds: vec![RelationKind::Implements],
                }),
                limit: QueryLimit::new(10),
            }),
            MindOperationKind::QueryRelations,
        ),
        (
            MindRequest::SubscribeThoughts(SubscribeThoughts {
                filter: ThoughtFilter::ByAuthor(ByThoughtAuthor {
                    author: ActorName::new("operator"),
                }),
                resume_after: None,
                initial_demand: sample_subscription_demand(),
            }),
            MindOperationKind::SubscribeThoughts,
        ),
        (
            MindRequest::SubscribeRelations(SubscribeRelations {
                filter: RelationFilter::ByTarget(ByRelationTarget {
                    target: RecordIdentifier::new("goal-aab"),
                }),
                resume_after: Some(SubscriptionCursor::new(21)),
                initial_demand: sample_subscription_demand(),
            }),
            MindOperationKind::SubscribeRelations,
        ),
        (
            MindRequest::SubscriptionDemand(SubscriptionDemand {
                subscription: SubscriptionIdentifier::new("sub-aab"),
                credit: SubscriptionDemandCredit::new(5),
            }),
            MindOperationKind::SubscriptionDemand,
        ),
        (
            MindRequest::Opening(Opening {
                kind: ItemKind::Task,
                priority: Magnitude::High,
                title: Title::new("Add operation kinds"),
                body: TextBody::new("Expose discriminants from the contract crate."),
            }),
            MindOperationKind::Opening,
        ),
        (
            MindRequest::NoteSubmission(NoteSubmission {
                item: ItemReference::Stable(fixture.item_id.clone()),
                body: TextBody::new("Contract-owned discriminant witness."),
            }),
            MindOperationKind::NoteSubmission,
        ),
        (
            MindRequest::Link(Link {
                source: ItemReference::Stable(fixture.item_id.clone()),
                kind: EdgeKind::References,
                target: LinkTarget::External(ExternalReference::File(ReferencePath::new(
                    "/git/github.com/LiGoldragon/signal-mind/src/lib.rs",
                ))),
                body: None,
            }),
            MindOperationKind::Link,
        ),
        (
            MindRequest::StatusChange(StatusChange {
                item: ItemReference::Stable(fixture.item_id.clone()),
                status: ItemStatus::InProgress,
                body: None,
            }),
            MindOperationKind::StatusChange,
        ),
        (
            MindRequest::AliasAssignment(AliasAssignment {
                item: ItemReference::Stable(fixture.item_id.clone()),
                alias: ExternalAlias::new("primary-aab"),
            }),
            MindOperationKind::AliasAssignment,
        ),
        (
            MindRequest::Query(Query {
                kind: QueryKind::Ready,
                limit: QueryLimit::new(10),
            }),
            MindOperationKind::Query,
        ),
        (
            MindRequest::AdjudicationRequest(AdjudicationRequest {
                request: sample_adjudication_request(),
                origin: MessageOrigin::local_connection(ConnectionClass::Owner),
                destination: sample_internal_endpoint(sample_router_component()),
                kind: ChannelMessageKind::MessageSubmission,
                body_summary: TextBody::new("owner request"),
            }),
            MindOperationKind::AdjudicationRequest,
        ),
        (
            MindRequest::ChannelList(ChannelList { filters: vec![] }),
            MindOperationKind::ChannelList,
        ),
        (
            MindRequest::SubmitTechnicalNode(SubmitTechnicalNode {
                stable_key: sample_technical_key("component:mind"),
                kind: TechnicalNodeKind::Component,
                body: TechnicalFixture::new().component_body(),
            }),
            MindOperationKind::SubmitTechnicalNode,
        ),
        (
            MindRequest::SubmitTechnicalRelation(SubmitTechnicalRelation {
                kind: TechnicalRelationKind::DefinesContract,
                source: sample_technical_key("crate:signal-mind"),
                target: sample_technical_key("contract:signal-mind:ordinary"),
                note: None,
            }),
            MindOperationKind::SubmitTechnicalRelation,
        ),
        (
            MindRequest::QueryTechnicalNodes(QueryTechnicalNodes {
                query: TechnicalNodeQuery::Filter(TechnicalNodeFilter::ByKind(
                    ByTechnicalNodeKind {
                        kinds: vec![TechnicalNodeKind::Component],
                    },
                )),
                limit: QueryLimit::new(10),
            }),
            MindOperationKind::QueryTechnicalNodes,
        ),
        (
            MindRequest::QueryTechnicalRelations(QueryTechnicalRelations {
                filter: TechnicalRelationFilter::ByKind(ByTechnicalRelationKind {
                    kinds: vec![TechnicalRelationKind::BuildDependency],
                }),
                limit: QueryLimit::new(10),
            }),
            MindOperationKind::QueryTechnicalRelations,
        ),
        (
            MindRequest::SubscribeTechnicalNodes(SubscribeTechnicalNodes {
                filter: TechnicalNodeFilter::ByStableKey(ByTechnicalNodeStableKey {
                    stable_key: sample_technical_key("component:mind"),
                }),
                resume_after: None,
                initial_demand: sample_subscription_demand(),
            }),
            MindOperationKind::SubscribeTechnicalNodes,
        ),
        (
            MindRequest::SubscribeTechnicalRelations(SubscribeTechnicalRelations {
                filter: TechnicalRelationFilter::BySource(ByTechnicalRelationSource {
                    source: sample_technical_key("component:mind"),
                }),
                resume_after: Some(SubscriptionCursor::new(23)),
                initial_demand: sample_subscription_demand(),
            }),
            MindOperationKind::SubscribeTechnicalRelations,
        ),
        (
            MindRequest::SubmitKnowledge(KnowledgeFixture::new().entity_submission()),
            MindOperationKind::SubmitKnowledge,
        ),
        (
            MindRequest::QueryKnowledge(KnowledgeQuery::ListByKind(
                KnowledgeRecordKind::Entity,
                CurrentView::CurrentOnly,
            )),
            MindOperationKind::QueryKnowledge,
        ),
    ];

    for (request, operation) in cases {
        assert_eq!(request.operation_kind(), operation);
    }
}

#[test]
fn mind_request_variants_declare_contract_local_operation_heads() {
    assert_eq!(
        <MindRequest as SignalOperationHeads>::HEADS,
        &[
            "SubmitThought",
            "SubmitRelation",
            "QueryThoughts",
            "QueryRelations",
            "SubscribeThoughts",
            "SubscribeRelations",
            "SubscriptionRetraction",
            "SubscriptionDemand",
            "Opening",
            "NoteSubmission",
            "Link",
            "StatusChange",
            "AliasAssignment",
            "Query",
            "AdjudicationRequest",
            "ChannelList",
            "SubmitTechnicalNode",
            "SubmitTechnicalRelation",
            "QueryTechnicalNodes",
            "QueryTechnicalRelations",
            "SubscribeTechnicalNodes",
            "SubscribeTechnicalRelations",
            "SubmitKnowledge",
            "QueryKnowledge",
        ]
    );
}

#[test]
fn mind_contract_has_no_sema_classification_dependency_or_roots() {
    let manifest = include_str!("../Cargo.toml");
    assert!(
        !manifest.contains("signal-sema"),
        "ordinary signal contracts must not depend on signal-sema for public wire vocabulary"
    );

    let heads = <MindRequest as SignalOperationHeads>::HEADS;
    for forbidden in [
        "Assert",
        "Mutate",
        "Retract",
        "Match",
        "Subscribe",
        "Validate",
    ] {
        assert!(
            !heads.contains(&forbidden),
            "Sema classification root {forbidden} must not appear on the public mind wire"
        );
    }
}

#[test]
fn mind_operation_kind_round_trips_through_nota_text() {
    round_trip_nota(
        MindOperationKind::AdjudicationRequest,
        "AdjudicationRequest",
    );
}

// ─── Reply variants ───────────────────────────────────────

#[test]
fn memory_receipt_replies_round_trip() {
    let fixture = MemoryFixture::new();
    let replies = vec![
        MindReply::OpeningReceipt(OpeningReceipt {
            event: fixture.opened_event(),
        }),
        MindReply::NoteReceipt(NoteReceipt {
            event: fixture.note_event(),
        }),
        MindReply::LinkReceipt(LinkReceipt {
            event: fixture.edge_event(),
        }),
        MindReply::StatusReceipt(StatusReceipt {
            event: fixture.status_event(),
        }),
        MindReply::AliasReceipt(AliasReceipt {
            event: fixture.alias_event(),
        }),
        MindReply::View(fixture.view()),
        MindReply::Rejection(Rejection {
            reason: RejectionReason::UnknownItem,
        }),
    ];

    for reply in replies {
        let decoded = round_trip_reply(reply.clone());
        assert_eq!(decoded, reply);
    }
}

#[test]
fn channel_choreography_replies_round_trip() {
    let replies = vec![
        MindReply::AdjudicationReceipt(AdjudicationReceipt {
            request: sample_adjudication_request(),
        }),
        MindReply::ChannelListView(ChannelListView {
            channels: vec![ChannelView {
                channel: sample_channel(),
                source: sample_internal_endpoint(sample_mind_component()),
                destination: sample_internal_endpoint(sample_router_component()),
                kinds: vec![
                    ChannelMessageKind::MessageDelivery,
                    ChannelMessageKind::AdjudicationRequest,
                ],
                duration: ChannelDuration::OneShot,
            }],
        }),
    ];

    for reply in replies {
        let decoded = round_trip_reply(reply.clone());
        assert_eq!(decoded, reply);
    }
}

#[test]
fn explicit_variant_lifts_opening_into_request() {
    let opening = Opening {
        kind: ItemKind::Question,
        priority: Magnitude::Medium,
        title: Title::new("Choose migration order"),
        body: TextBody::new("Need a decision before implementation."),
    };
    let request = MindRequest::Opening(opening.clone());
    assert_eq!(request, MindRequest::Opening(opening));
}

#[test]
fn explicit_variant_lifts_view_into_reply() {
    let view = MemoryFixture::new().view();
    let reply = MindReply::View(view.clone());
    assert_eq!(reply, MindReply::View(view));
}

// ─── Scope-reference variants ─────────────────────────────

#[test]
fn path_scope_round_trips() {
    round_trip_nota(
        ScopeReference::Path(sample_path()),
        "(Path /git/github.com/LiGoldragon/signal-mind/src/lib.rs)",
    );
}

#[test]
fn task_scope_round_trips() {
    round_trip_nota(ScopeReference::Task(sample_task()), "(Task primary-f99)");
}

// ─── Boundary validation ──────────────────────────────────

#[test]
fn wire_path_requires_absolute_normalized_path() {
    assert!(WirePath::from_absolute_path("/git/github.com/LiGoldragon/persona").is_ok());
    assert!(WirePath::from_absolute_path("relative/path").is_err());
    assert!(WirePath::from_absolute_path("").is_err());
    assert!(WirePath::from_absolute_path("/git/../persona").is_err());

    let normalized = WirePath::from_absolute_path("/git//github.com/./LiGoldragon/persona/")
        .expect("normalizable absolute path");
    assert_eq!(normalized.as_str(), "/git/github.com/LiGoldragon/persona");

    let root = WirePath::from_absolute_path("/").expect("root path");
    assert_eq!(root.as_str(), "/");
}

#[test]
fn wire_path_nota_decode_uses_boundary_validation() {
    let error = NotaSource::new("relative/path")
        .parse::<WirePath>()
        .expect_err("relative path must fail validation");
    let message = error.to_string();
    assert!(message.contains("absolute"), "message was: {message}");
}

#[test]
fn task_token_rejects_brackets_empty_and_whitespace() {
    assert!(TaskToken::from_wire_token("primary-f99").is_ok());
    assert!(TaskToken::from_wire_token("[primary-f99]").is_err());
    assert!(TaskToken::from_wire_token("").is_err());
    assert!(TaskToken::from_wire_token("primary f99").is_err());
}
