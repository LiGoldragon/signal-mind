//! Signal contract — `mind` CLI ↔ `mind`.
//!
//! Read `ethos/signal.ethos` as the authored source of this contract; the
//! Rust in `src/generated/signal.rs` is generated from it and `build.rs`
//! asserts the two agree. The channel carries:
//!
//! - **Memory/work graph** — append typed item, note, edge, alias, and
//!   status events, then query the derived view.
//! - **Typed mind graph substrate** — submit/query/subscribe to closed
//!   Thought and Relation records (`Observation`, `Memory`, `Belief`,
//!   `Goal`, `Claim`, `Decision`, `Reference`).
//! - **Technical dependency memory** — submit/query/subscribe to closed
//!   TechnicalNode and TechnicalRelation records.
//! - **Knowledge** — submit and retrieve domain-scoped knowledge records.
//!
//! The channel is request/reply: every `Query` has a typed `Response`.
//! Subscription delivery rides the `SubscriptionDelta` response.
//!
//! The portable rkyv frame and its three kinds come from `signal` and are
//! re-exported here, so a mind frame is the same type as every other
//! contract's frame and one generic transport carries them all.

pub mod generated;
pub use generated::signal::*;

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");

pub use signal::{ByteViewable, Restorable, Signal, Signalizable};

/// `Magnitude` is an ordered qualitative strength: `Zero` is the neutral
/// bottom rung and `Maximum` the top. The order is the order the variants
/// are declared in `ethos/signal.ethos`.
impl Magnitude {
    const fn rank(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::Minimum => 1,
            Self::VeryLow => 2,
            Self::Low => 3,
            Self::Medium => 4,
            Self::High => 5,
            Self::VeryHigh => 6,
            Self::Maximum => 7,
        }
    }
}

impl Eq for Magnitude {}

impl PartialOrd for Magnitude {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Magnitude {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank().cmp(&other.rank())
    }
}
