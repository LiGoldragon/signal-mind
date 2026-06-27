# signal-mind

The Signal contract between **`mind`** (the CLI agents invoke per call)
and **`mind`** (the central state actor that owns `mind.sema`).

Read `src/lib.rs` for the public interface — two enums
(`MindRequest`, `MindReply`) declared via the
`signal_channel!` macro. The variants ARE the messages this
channel carries:

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

Ordinary role claims, handoffs, observations, and activity log operations
belong to `signal-persona-orchestrate`.

## Quick reference

```rust
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, RequestPayload, SessionEpoch,
};
use signal_mind::{
    ItemKind, Magnitude, MindFrame, MindFrameBody, MindRequest, Opening,
    TextBody, Title,
};

let exchange = ExchangeIdentifier::new(
    SessionEpoch::new(1),
    ExchangeLane::Connector,
    LaneSequence::first(),
);
let request = MindRequest::Opening(Opening {
    kind: ItemKind::Task,
    priority: Magnitude::High,
    title: Title::new("wire command-line mind"),
    body: TextBody::new("replace transitional task storage with typed mind state"),
});
let frame = MindFrame::new(MindFrameBody::Request {
    exchange,
    request: request.into_request(),
});
let bytes = frame.encode_length_prefixed()?;
// hand to mind's daemon dispatcher
```

The state actor replies with `MindReply::OpeningReceipt` on success.

Use the public constructors for boundary strings before
building a frame: `WirePath::from_absolute_path` (which
stores a normalized absolute path) and `TaskToken::from_wire_token`.

## See also

- `ARCHITECTURE.md` — channel role + boundaries
- `~/primary/skills/contract-repo.md` — contract-repo
  discipline
- `signal-frame` — kernel that supplies `Frame`,
  `Request`, `Reply`, `signal_channel!`
- `mind` — the consumer that implements
  this contract
