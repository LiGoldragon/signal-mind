# signal-persona-mind — architecture

*Typed Signal contract for the command-line mind and `persona-mind`.*

---

## 0 · TL;DR

`signal-persona-mind` is the public vocabulary for Persona's central mind. It
defines the typed request/reply channel used by the `mind` CLI and long-lived
`persona-mind` daemon.

## MUST IMPLEMENT — signal architecture migration

This contract is migrating to contract-local verbs per
`primary/reports/designer/238-signal-architecture-redirection-contract-local-verbs.md`
and `primary/reports/designer/239-signal-architecture-migration-plan.md`.

The current `MindRequest` enum mixes many SignalVerb-tagged variants
across three relations (typed mind graph; work-and-memory graph;
channel choreography). Drop the SignalVerb prefixes throughout. The
public verbs for the typed-graph relation become `Submit` (for
`SubmitThought`, `SubmitRelation` — already verb-form; payloads are
`Thought` / `Relation`), `Query` (lift the repeated `Query*` siblings —
`QueryThoughts`, `QueryRelations` — into one `Query` operation root
whose payload names the read target), `Watch` (for the subscribe
side; payload names the watch target — thoughts vs relations).
For the work-and-memory graph, `Submit` covers `Opening`,
`NoteSubmission` (rename payload to `Note`), `Link`, and
`AliasAssignment` (payload `AliasAssignment`); `Mutate StatusChange`
becomes a contract-local verb — likely `ChangeStatus` or `Transition`
since `Mutate` is grammatically wrong on a verb. The channel-choreography family splits into multiple
contract-local verbs (psyche-settled 2026-05-19T20:30Z — verbs are
cheap; the split makes each operation's intent visible at the call
site): `Grant` (issue a channel grant), `Extend` (extend an existing
grant), `Revoke` (retract a live grant; replaces `ChannelRetract`),
`Adjudicate` (open an adjudication for resolution; replaces
`AdjudicationRequest`), `Deny` (reject an adjudication; replaces
`AdjudicationDeny`), and `Query` (read the channel list; replaces
`ChannelList`). The prior "collapse under one Adjudicate verb"
option is retired. Move the verb-to-Sema lowering out of the
contract entirely — the daemon executor decides what Sema operations
each contract verb produces, possibly fanning out (e.g., `Grant` may
Assert the grant record and Mutate live-grant state in one
transaction).

References: `primary/reports/designer/238-signal-architecture-redirection-contract-local-verbs.md`,
`primary/reports/designer/239-signal-architecture-migration-plan.md`.

**Note to remover:** when the refactor lands, remove this section and
add a `## Migration history — contract-local verbs (2026-05-XX)`
paragraph noting the shape change.

> **Scope.** This contract sits on today's stack — `signal-core` wire,
> rkyv archives, `sema-db` storage in consumers. The
> eventually-self-hosting stack is Sema-on-Sema, in which signal-*
> as a separate vocabulary layer collapses. Today's contract is a
> realization step. See `~/primary/ESSENCE.md` §"Today and eventually".

This repo owns records, validation newtypes, rkyv round trips, and channel
shape. It does not own the CLI binary, actors, database, storage tables,
transport lifecycle, or lock-file migration.

It also owns the mapping from each `MindRequest` variant to the
`SignalVerb` root that frames it. The `signal_channel!` declaration carries
the root beside each request variant and emits `MindRequest::signal_verb()`,
so graph creation, graph queries, subscriptions, and channel retraction cannot
silently travel as `Assert`.

```mermaid
flowchart LR
    cli[mind CLI] --> request[MindRequest]
    request --> frame[signal frame]
    frame --> daemon[persona mind daemon]
    daemon --> reply[MindReply]
    reply --> cli
```

## 1 · Channel Boundary

| Side | Component |
|---|---|
| Request producer | `mind` CLI and future hosts that speak the same channel. |
| Request consumer | `persona-mind` daemon. |
| Reply producer | `persona-mind` daemon. |
| Reply consumer | the caller that submitted the operation. |

The CLI text surface is one NOTA record in and one NOTA record out. That text
projection must decode into the same `MindRequest` enum declared here. It must
not create a second CLI-only command language.

Rust-to-Rust boundaries use `signal-core` frames carrying rkyv archives. The
same typed request/reply vocabulary underlies both the NOTA projection and the
binary frame projection.

The local transport between CLI and daemon belongs to `persona-mind`, not this
contract. The likely first transport is a Unix socket carrying `signal-core`
frames.

## 2 · Channel Declaration

The channel is one `signal_channel!` invocation in `src/lib.rs`.

```rust
signal_channel! {
    channel Mind {
        request MindRequest {
            Assert SubmitThought(SubmitThought),
            Assert SubmitRelation(SubmitRelation),
            Match QueryThoughts(QueryThoughts),
            Match QueryRelations(QueryRelations),
            Subscribe SubscribeThoughts(SubscribeThoughts) opens MindEventStream,
            Subscribe SubscribeRelations(SubscribeRelations) opens MindEventStream,
            Retract SubscriptionRetraction(SubscriptionId),
            Assert Opening(Opening),
            Assert NoteSubmission(NoteSubmission),
            Assert Link(Link),
            Mutate StatusChange(StatusChange),
            Assert AliasAssignment(AliasAssignment),
            Match Query(Query),
            Assert AdjudicationRequest(AdjudicationRequest),
            Assert ChannelGrant(ChannelGrant),
            Mutate ChannelExtend(ChannelExtend),
            Retract ChannelRetract(ChannelRetract),
            Assert AdjudicationDeny(AdjudicationDeny),
            Match ChannelList(ChannelList),
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
            ChannelReceipt(ChannelReceipt),
            AdjudicationDenyReceipt(AdjudicationDenyReceipt),
            ChannelListView(ChannelListView),
            MindRequestUnimplemented(MindRequestUnimplemented),
        }
        event MindEvent {
            SubscriptionDelta(SubscriptionEvent) belongs MindEventStream,
        }
        stream MindEventStream {
            token SubscriptionId;
            opened SubscriptionAccepted;
            event SubscriptionDelta;
            close SubscriptionRetraction;
        }
    }
}
```

Closed enums are intentional. There is no `Unknown` escape hatch. New
operations are schema changes coordinated through this contract.

The request enum exposes two contract-owned discriminants:

- `operation_kind()` names the domain operation for audit and UI surfaces.
- `signal_verb()` is emitted by `signal_channel!` from the root written beside
  each request variant and names the operation root used in the
  `signal-core::Request` envelope.

The second mapping belongs here because this contract owns the request
vocabulary. Runtime components execute the mapped verb; they do not infer it
from strings or default every payload to `Assert`.

## 3 · Record Families

### 3.1 Typed mind graph substrate

| Request | Reply |
|---|---|
| `SubmitThought` | `ThoughtCommitted` |
| `SubmitRelation` | `RelationCommitted` |
| `QueryThoughts` | `ThoughtList` |
| `QueryRelations` | `RelationList` |
| `SubscribeThoughts` | `SubscriptionAccepted`, then `SubscriptionDelta` events, terminated by `SubscriptionRetracted` |
| `SubscribeRelations` | `SubscriptionAccepted`, then `SubscriptionDelta` events, terminated by `SubscriptionRetracted` |

Subscription close follows the `signal_channel!` streaming grammar. The
`Subscribe` request opens the stream; the consumer sends a typed
`MindRequest::SubscriptionRetraction(SubscriptionId)` request to close it;
the producer emits `MindReply::SubscriptionRetracted` as the final
acknowledgement before the stream ends. Both the retract request and the
retracted reply are first-class — `signal_channel!` derives the
`MindRequest::closed_stream()` discriminant from this pairing.

The graph surface is the first typed substrate for replacing BEADS and later
rendering reports/architecture/skills from mind state. The closed node family
is `ThoughtKind`: `Observation`, `Memory`, `Belief`, `Goal`, `Claim`,
`Decision`, `Reference`. The closed edge family is `RelationKind`:
`Implements`, `Realizes`, `Requires`, `Supports`, `Refutes`, `Supersedes`,
`Authored`, `References`, `Decides`, `Considered`, `Belongs`.

`RelationKind` owns the domain/range validator for this graph vocabulary. The
validator is contract code, not runtime folklore: producers and consumers call
the same table before accepting a relation. The full endpoint validator also
checks relation-specific body constraints; `Authored` requires a source Thought
whose body is `Reference(Identity)`, not just any Reference.

`RecordId` and `RelationId` are opaque contract values. `persona-mind` owns
their minting, collision handling, durable indices, and short display-id
projection. The contract owns only the typed records that cross the channel.

### 3.2 Work and memory graph

| Request | Reply |
|---|---|
| `Opening` | `OpeningReceipt` |
| `NoteSubmission` | `NoteReceipt` |
| `Link` | `LinkReceipt` |
| `StatusChange` | `StatusReceipt` |
| `AliasAssignment` | `AliasReceipt` |
| `Query` | `View` |

These records are the active native replacement for BEADS as a work/memory
graph. Imported BEADS IDs are represented as aliases or external references;
the contract does not model a live BEADS backend.

### 3.4 Channel choreography

| Request | Reply |
|---|---|
| `AdjudicationRequest` | `AdjudicationReceipt` |
| `ChannelGrant` | `ChannelReceipt` |
| `ChannelExtend` | `ChannelReceipt` |
| `ChannelRetract` | `ChannelReceipt` |
| `AdjudicationDeny` | `AdjudicationDenyReceipt` |
| `ChannelList` | `ChannelListView` |

These records are the typed boundary between `persona-router` and
`persona-mind` for channel choreography. Router parks a message whose
channel is missing or inactive and submits `AdjudicationRequest`. Mind replies
by recording the request, deciding policy internally, and later emitting a
grant, extension, retraction, deny, or channel view through the same closed
contract vocabulary.

The destination handler set inside `persona-mind` is a single stateful
`ChoreographyAdjudicator` actor that owns policy, the live grant table, and the
adjudication log. `AdjudicationRequest`, `ChannelGrant`, `ChannelExtend`,
`ChannelRetract`, `AdjudicationDeny`, and `ChannelList` all route to that one
actor; it answers with the receipt or view reply for each. This contract owns
only the request/reply vocabulary that crosses the channel; the actor and its
state shape belong to `persona-mind`. Until that actor lands, mind replies
`MindRequestUnimplemented(ChoreographyPolicyMissing)` for this family.

The endpoint and kind vocabulary is typed:

- `ChannelEndpoint` is either `Internal(ComponentName)` or
  `External(ConnectionClass)`.
- `ChannelMessageKind` is a closed enum for first-stack route categories such
  as message submission, inbox query, message delivery, terminal input, prompt
  observation, adjudication, and channel grant/retract traffic. **Includes
  `MessageIngressSubmission`** — the channel kind for the
  `Internal(Message) → Internal(Router)` structural channel that
  `persona-message-daemon` forwards user-typed messages over. This
  variant must be distinct from the generic delivery kinds so audit and
  choreography can tell message ingress from other internal traffic.
- `ChannelDuration` is `OneShot`, `Permanent`, or `TimeBound(TimestampNanos)`.

## 4 · Boundary Newtypes

The contract validates boundary strings before they become wire values.

| Type | Invariant |
|---|---|
| `RoleName` | closed role set plus canonical wire-token parsing/rendering: operator, operator-assistant, second-operator-assistant, designer, designer-assistant, second-designer-assistant, system-specialist, system-assistant, second-system-assistant, poet, poet-assistant. |
| `WirePath` | absolute normalized slash-separated path; rejects `..`. |
| `TaskToken` | raw unbracketed token, non-empty, no whitespace or brackets. |
| `TimestampNanos` | store-supplied timestamp type; request records do not mint it. |
| `ActorName` | event/caller identity after infrastructure resolution. |
| `RecordId` | opaque durable thought identifier minted by `persona-mind`. |
| `RelationId` | opaque durable relation identifier minted by `persona-mind`. |
| `StableItemId` | internal work graph identity. |
| `DisplayId` | short human identity for work graph references. |
| `ExternalAlias` | imported or external identifiers. |
| `AdjudicationRequestId` | short router-minted identifier for one parked adjudication request. |
| `ChannelEndpoint` | typed internal/external route endpoint using `signal-persona-auth`. |
| `ChannelMessageKind` | closed set of first-stack route categories. |
| `ChannelDuration` | channel lifetime requested or emitted by mind choreography. |

Strings in `Title`, `Body`, and path-like wrappers are provisional where the
semantic shape is still evolving. They are still typed at the boundary; callers
do not pass unstructured maps.

## 5 · Text Projection

The required text surface is NOTA. Nexus may supply the semantic content shape
inside NOTA, but there is no second text syntax.

The contract records implement NOTA directly. Root `MindRequest` and
`MindReply` text decoding dispatches through `signal_core::signal_channel!`;
payload records derive or implement NOTA in this crate. Validating boundary
newtypes such as `WirePath` and `TaskToken` decode through their constructors,
so text input cannot bypass boundary validation.

```mermaid
flowchart LR
    text[NOTA record] --> decode[contract decoder]
    decode --> request[MindRequest]
    request --> encode[contract encoder]
    encode --> text_again[NOTA record]
```

Representative contract text shapes:

```text
(Query (Ready) 25)
(Opening Task High "wire command-line mind" "replace lock helper with typed state")
(ChannelGrant (Internal Router) (Internal Harness) [Delivery])
```

Surface owners decide where this NOTA is accepted or rendered. This crate owns
the codec on the contract types, and the parsed value is one of the
`MindRequest` or `MindReply` variants declared here.

## 6 · Versioning

`signal-core::Frame` carries protocol version. Schema changes that add/remove
variants or change fields require coordinated upgrades of producers and
consumers.

Backward compatibility is handled by explicit conversion code, not by weak
catch-all records.

## 6.5 · Skeleton honesty (Unimplemented reply)

`MindReply` carries a typed `MindRequestUnimplemented(MindUnimplementedReason)`
variant. Prototype-time mind decodes every `MindRequest` variant; for choreography
ops or other variants whose behavior is not yet built (e.g., the
`ChannelGrant` / `ChannelRetract` / `AdjudicationDeny` family until the
choreography policy engine lands), mind replies
`MindRequestUnimplemented(NotInPrototypeScope)` — a typed answer, not a panic
and not a parse error.

```text
MindUnimplementedReason
  | NotInPrototypeScope                  -- variant exists in contract; behavior not yet built
  | ChoreographyPolicyMissing            -- specific reason for the choreography family
  | DependencyMissing(DependencyKind)     -- Router, Harness, Terminal, DurableStore
  | ResourceUnavailable(ResourceKind)     -- SocketPath, StateDirectory, Database
```

## 7 · Constraints

- The channel is one closed `MindRequest` enum and one closed `MindReply`
  enum emitted by `signal_channel!`.
- The architecture's channel declaration matches the implemented
  `signal_channel!` invocation in `src/lib.rs`.
- `RoleName` covers every workspace coordination role in
  `~/primary/protocols/orchestration.md`.
- Request payloads do not mint `ActorName`, `TimestampNanos`, `EventSeq`,
  `OperationId`, stable item IDs, or display IDs.
- Lock files and BEADS are represented only as temporary external references or
  aliases, never as live backend protocol.
- Channel choreography records use `signal-persona-auth` endpoint and origin
  types; they do not carry proof material.
- Channel choreography is closed vocabulary; there is no stringly "kind" or
  catch-all request.
- This contract crate contains no CLI, daemon, actor runtime, database table,
  transport, or migration implementation.
- The text surface is NOTA projected into these exact records; there is no
  second command language.
- Subscription close uses the streaming grammar: a `Subscribe` request opens
  the stream; a typed `Retract SubscriptionRetraction(SubscriptionId)`
  request closes it; the producer emits `MindReply::SubscriptionRetracted`
  as the final acknowledgement before the stream ends.
- Channel-choreography requests route inside `persona-mind` to one stateful
  `ChoreographyAdjudicator` actor; this contract closes the vocabulary, the
  actor owns the policy and grant state.

## 8 · Tests

Existing tests in `tests/round_trip.rs` cover:

- request/reply frame round trips;
- representative NOTA text round trips for root requests and replies:
  `Query`, `Opening`, and `ChannelGrant`;
- memory/work variants;
- every `QueryKind`;
- every `EdgeKind`;
- channel choreography request/reply variants;
- typed unimplemented reason variants;
- `MessageIngressSubmission` distinct from generic `MessageSubmission`;
- scope variants;
- external references;
- boundary validation, including `WirePath` NOTA decode rejection.
- workspace role coverage.
- relation-kind domain/range validation and table coverage.

Additional architecture guards still worth adding:

| Test | Proves |
|---|---|
| `nota_projection_rejects_cli_only_command` | no second command language. |
| `request_payload_cannot_carry_timestamp` | store mints time. |
| `request_payload_cannot_carry_event_sequence` | store mints sequence. |
| `contract_crate_cannot_spawn_actor_runtime` | contract crate stays behavior-free. |
| `subscription_retracted_round_trips_as_reply` | Path A reply-side close is wired in `MindReply` and `MindEventStream::close`. |

## 9 · Non-ownership

This repo does not own:

- `mind` binary implementation;
- Kameo actors;
- `mind.redb`;
- daemon lifecycle and local socket path;
- `sema` table declarations;
- caller identity resolution policy;
- time/ID minting policy;
- lock-file migration workflow;
- BEADS import code.

## Code Map

```text
src/lib.rs              payload records, NOTA codecs, and signal_channel! declaration
tests/round_trip.rs     frame round trips, NOTA witnesses, and validation tests
```

## See Also

- `../persona-mind/ARCHITECTURE.md`
- `../signal-core/ARCHITECTURE.md`
- `~/primary/protocols/orchestration.md`
- `~/primary/skills/contract-repo.md`
