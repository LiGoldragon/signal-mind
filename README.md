# signal-mind

The Signal contract between **`mind`** (the CLI agents invoke per call)
and **`mind`** (the central state actor that owns mind's durable store).

Read `ethos/signal.ethos` for the authored contract. It is the source of
truth: `src/generated/signal.rs` is generated from it by `ethos-zero`, and
`build.rs` asserts the committed generation matches a fresh one. Never
hand-edit the generated Rust, and never hand-write a `Datomic` impl for a
declared type.

The generation yields two enums — `Query` (requests) and `Response`
(replies). Their variants ARE the messages this channel carries:

- **Memory/work graph:** `Opening`, `NoteSubmission`, `Link`,
  `StatusChange`, `AliasAssignment`, `Query`.
- **Typed mind graph:** `SubmitThought`, `SubmitRelation`,
  `QueryThoughts`, `QueryRelations`, `SubscribeThoughts`,
  `SubscribeRelations`, `SubscriptionRetraction`, `SubscriptionDemand`.
- **Typed technical dependency graph:** `SubmitTechnicalNode`,
  `SubmitTechnicalRelation`, `QueryTechnicalNodes`,
  `QueryTechnicalRelations`, and technical subscriptions. Subscription opens
  and deltas are family-typed, carry resume cursors, and advertise bounded
  producer-side buffers. This contract does not promise a durable outbox;
  reconnect completeness is a daemon/storage behavior above the wire shape.
  Stable public technical keys are canonical family keys such as
  `component:mind`, `repo:signal-mind`, and
  `contract:signal-mind:ordinary`.
- **Channel choreography:** `AdjudicationRequest`, `ChannelList`.
- **Knowledge:** `Submit`, `Get`.

Ordinary role claims, handoffs, observations, and activity log operations
belong to `signal-persona-orchestrate`.

## Quick reference

A value crosses the wire as an rkyv archive inside a typed `Signal<T>`:

```rust
use signal_mind::{
    ByteViewable, ItemKind, Magnitude, Opening, Query, Restorable, Signal,
    Signalizable,
};

let query = Query::Opening(Opening {
    item_kind: ItemKind::Task,
    magnitude: Magnitude::High,
    title: "wire command-line mind".into(),
    text_body: "replace transitional task storage with typed mind state".into(),
});
let frame = query.signalize()?;
let bytes = frame.bytes().to_vec(); // hand to mind's transport
let restored: Query = Signal::<Query>::from(bytes).restore()?;
```

The state actor replies with `Response::OpeningReceipt` on success.

With the `datom` feature the same values render and read back as Datom
text through `Datomizable` / `Potential`. `examples/canonical.datom` holds
one example per area of the contract, and `tests/generated_contract.rs`
actualizes every line of it, so an example that stops parsing fails the
gate.

## See also

- `ARCHITECTURE.md` — channel role + boundaries
- `ethos/signal.ethos` — the authored contract
- `signal-persona`, `signal-domain` — the contracts this one imports from
- `mind` — the consumer that implements this contract
