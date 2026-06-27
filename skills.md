# skills — signal-mind

*Per-repo agent guide.*

## Checkpoint — read before editing

Before changing code in this repo, read:

- `~/primary/skills/contract-repo.md` — contract-repo
  discipline (what goes here vs. doesn't).
- `~/primary/skills/architecture-editor.md` — `ARCHITECTURE.md`
  conventions.
- `~/primary/skills/architectural-truth-tests.md` — every
  contract change needs a witness test.
- `~/primary/skills/nix-discipline.md` — flake-input rules,
  `nix flake check` is the gate.
- this repo's `ARCHITECTURE.md`.
- the consumer's `ARCHITECTURE.md` (`mind/`).

If your change adds a new request or reply variant, edit
`src/lib.rs` first, push, then update the consumer
(`mind`) to handle it.

## What this repo owns

- `RoleName` (closed enum: Operator, OperatorAssistant,
  SecondOperatorAssistant, Designer, DesignerAssistant,
  SecondDesignerAssistant, SystemSpecialist, SystemAssistant,
  SecondSystemAssistant, Poet, PoetAssistant, plus canonical
  workspace role token parsing/rendering) where mind graph records
  need role identity.
- `ScopeReference` (closed enum: Path | Task) plus
  `WirePath` and `TaskToken` newtypes.
- `TimestampNanos` (store-supplied; never agent-supplied).
- The typed mind graph substrate: `ThoughtKind` / `ThoughtBody`,
  `RelationKind`, `Thought`, `Relation`, `RecordIdentifier`, `RelationIdentifier`,
  thought/relation filters, subscription records, and graph
  commit/list replies.
- The typed technical dependency substrate: canonical `TechnicalNodeKey`
  families (`component`, `repo`, `crate`, `contract`, `task`, `artifact`,
  `report`, `claim`, `witness`, `storage`, `schema`, `table`), technical node
  bodies including storage/schema/table nodes, and split technical dependency
  relations (`BuildDependency`, `RuntimeDependency`, `WireDependency`,
  `StorageDependency`, `TaskDependency`, `ProvenanceDependency`).
- The closed `MindRequest` enum (`SubmitThought`,
  `SubmitRelation`, `QueryThoughts`, `QueryRelations`,
  `SubscribeThoughts`, `SubscribeRelations`,
  `SubscriptionRetraction`, `Opening`, `NoteSubmission`,
  `Link`, `StatusChange`, `AliasAssignment`, `Query`,
  `AdjudicationRequest`, `ChannelList`).
- The closed `MindReply` enum (`ThoughtCommitted`,
  `RelationCommitted`, `ThoughtList`, `RelationList`,
  `SubscriptionAccepted`, `SubscriptionRetracted`,
  `OpeningReceipt`, `NoteReceipt`, `LinkReceipt`,
  `StatusReceipt`, `AliasReceipt`, `View`, `Rejection`,
  `AdjudicationReceipt`, `ChannelListView`,
  `MindRequestUnimplemented`).
- The mind memory/work record vocabulary: `Item`, `Note`, `Edge`,
  `Event`, aliases, references, and ready-query records.
- The `Frame` type alias and round-trip tests.

## What this repo does not own

- The state actor or the database — that's
  `mind`.
- The CLI binary parsing — that's the `mind` bin target inside
  `mind`.
- Ordinary role claims, role release, handoff, role observation, and activity
  log operations — those live in `signal-persona-orchestrate`.
- Lock-file projection writing — outside this implementation
  target; `mind` replaces lock files instead of
  projecting them.
- Runtime storage tables — those live in `mind`'s SEMA engine integration.
  This contract owns only the typed vocabulary for storage/schema/table nodes
  and dependency facts about them.
- Public Sema-operation mirrors — no `AuthorizedSignalVerb`,
  `SemaOperation`, `ToSemaOperation`, `signal-sema`, or Sema-class root
  belongs in this contract.
