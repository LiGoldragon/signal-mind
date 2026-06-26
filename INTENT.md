# INTENT — signal-mind

*The wire vocabulary contract for Persona's central mind. Defines the typed
request/reply/event channel that the `mind` CLI and peer components use to submit
work-graph, typed-graph, and typed technical dependency memory operations, query
thoughts/relations and memory state, observe channel choreography, and subscribe
to state changes.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-mind` contract.
Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Component daemon intent stays in `mind/INTENT.md`. Meta mind policy stays in
`meta-signal-mind`.

## Why this repo exists

`signal-mind` is the **ordinary peer-callable wire contract** for the `mind`
daemon — Persona's central state holder. It carries the typed request/reply
channel used by the `mind` CLI and the long-lived `mind` daemon for four named
relations: the typed mind graph (Thought / Relation records), typed technical
dependency memory (TechnicalNode / TechnicalRelation records), the
work-and-memory graph (openings, notes, links, status, aliases), and channel
choreography observation. Ordinary role claims, handoffs, observations, and
activity-log operations belong to `signal-orchestrate`, not here. Runtime
actors, the `mind.sema` store, choreography decision logic, and authority orders
live in `mind` and `meta-signal-mind`.

## The channel shape

The Mind channel carries:

- **Typed mind graph:** `SubmitThought`, `SubmitRelation`, `QueryThoughts`,
  `QueryRelations`, `SubscribeThoughts`, `SubscribeRelations`,
  `SubscriptionRetraction`.
- **Typed technical dependency memory:** `SubmitTechnicalNode`,
  `SubmitTechnicalRelation`, `QueryTechnicalNodes`, `QueryTechnicalRelations`,
  `SubscribeTechnicalNodes`, `SubscribeTechnicalRelations`.
- **Work and memory graph:** `Opening`, `NoteSubmission`, `Link`, `StatusChange`,
  `AliasAssignment`, `Query`.
- **Channel choreography (read/observe side):** `AdjudicationRequest`,
  `ChannelList`.
- **Replies:** the committed/receipt/view records corresponding to each
  operation (`ThoughtCommitted`, `RelationCommitted`, technical committed/list
  replies, receipts, `View`, `AdjudicationReceipt`, `ChannelListView`),
  typed rejections, and `MindRequestUnimplemented` (skeleton honesty).
- **Events:** a `MindEventStream` delivering `SubscriptionDelta` events for open
  subscriptions; retraction closes the stream.

The wire vocabulary is contract-local: the daemon lowers these public operations
into component-local Nexus commands and SEMA reads or writes. Database-action
classification never crosses this public wire.

## Channels are closed, boundaries are named

- Wire enums are closed. No `Unknown` escape hatch; unimplemented paths reply
  `MindRequestUnimplemented`.
- Request payloads do not mint thought IDs, relation IDs, technical node IDs,
  technical relation IDs, event sequence, timestamps, or sender identity.
- `mind` mints those values at the daemon; request records accept submitted
  thought/relation bodies, technical stable node keys, and metadata only. Graph
  IDs are compact sequence-derived tokens minted from the store, not content
  hashes or payload fields.
- No stringly-typed dispatch. Graph kinds, channel endpoints, and reason fields
  are typed closed enums.

## Wire vocabulary discipline

Per `primary/skills/contract-repo.md` §"Public contracts use contract-local
operation verbs":

- Operation roots are domain verbs in verb form: `Submit` (thoughts, relations,
  technical nodes and relations, notes, links, aliases), `Query` (typed reads),
  `Adjudicate` (open an adjudication) — not Sema class words.
- Reply success variants name the outcome of the operation; rejections are
  `Rejection` carrying a typed reason.
- Payload record names are domain nouns the operation carries (`Thought`,
  `Relation`, `Note`, `Opening`, `Query`), not `Request` or generic containers.
- `src/lib.rs` now declares bare contract-local operation heads on
  `signal-frame`; the old `Assert`/`Match`/`Mutate`/`Subscribe`/`Retract`
  request tags are gone from the wire. The remaining follow-up is the mandatory
  `Tap`/`Untap` observability surface, where it replaces or augments
  domain-local subscription operations.
- Router channel authority orders (`Grant`, `Extend`, `Revoke`, `Deny`) are NOT
  ordinary mind working requests; they live in `meta-signal-router` and are
  issued by orchestrate. Mind decides at the cognitive level and orders through
  `meta-signal-mind` → orchestrate.

## Constraints

- This crate carries only typed wire vocabulary, NOTA codecs, and round-trip
  witnesses.
- No runtime code: no actors, no tokio, no socket binding, no redb, no
  choreography policy logic.
- Contract types derive NOTA in this crate. Consumers do not carry shadow types.
- Every operation, reply, and event variant round-trips through both rkyv frames
  and NOTA text.
- Request payloads cannot carry IDs, timestamps, or sequence numbers; the daemon
  supplies those.
- Technical dependency memory uses separate `TechnicalNode` /
  `TechnicalRelation` records. It is not another `ThoughtBody` variant.
- `TechnicalNodeKey` is the caller-stable technical identity; compact
  `TechnicalNodeIdentifier` and `TechnicalRelationIdentifier` values are minted
  by the daemon.
- Channel choreography observation is read-only in this contract; authority
  orders live in `meta-signal-mind`.

## Daemon lowering boundary

The contract names the public action at the boundary. The daemon decides what
internal work, durable read, durable write, effect, rejection, or reply each
action becomes. Public contracts do not mirror `Assert`, `Mutate`, `Retract`,
`Match`, `Subscribe`, or `Validate`, and this crate does not depend on
`signal-sema`.

## Code map

```text
src/lib.rs                     — shared newtypes, request/reply/event channel, signal_channel! invocation
src/graph.rs                   — typed Thought/Relation graph records and snapshot/delta shapes
src/technical.rs               — typed TechnicalNode/TechnicalRelation records, filters, validators, replies
schema/signal-mind.concept.schema — concept-schema source for the contract
tests/round_trip.rs            — rkyv frame and NOTA round-trip witnesses per operation
```

## Non-ownership

This crate does not own:

- `mind` daemon runtime, Kameo actors, or component lifecycle;
- `mind.sema` or any storage tables, graph indices, or choreography state;
- socket binding, transport, version handshake, or signature validation;
- choreography policy logic, channel grant execution, or adjudication decisions;
- ordinary role/activity orchestration (that is `signal-orchestrate`);
- CLI formatting, audit wrapping, or Nexus record composition.

## See also

- `ARCHITECTURE.md` — detailed channel shape, the three-layer migration, the
  three relations, and closed-enum discipline.
- `../mind/INTENT.md` — daemon-side intent (schema-driven planes, actor topology,
  state schema).
- `../meta-signal-mind/INTENT.md` — meta mind policy contract.
- `../signal-orchestrate/INTENT.md` — ordinary role/activity orchestration contract.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and wire layers.
