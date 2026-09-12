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

pub mod generated;
pub use generated::signal::*;

use std::marker::PhantomData;

pub const ETHOS: &str = include_str!("../ethos/signal.ethos");

/// A portable rkyv Signal frame whose target contract is carried in its type.
pub struct Signal<T> {
    bytes: Vec<u8>,
    target: PhantomData<fn() -> T>,
}

/// Data that can form a portable Signal frame.
pub trait Signalizable: Sized {
    fn signalize(&self) -> Result<Signal<Self>, rkyv::rancor::Error>;
}

/// A frame exposes its peer-wire bytes for transport framing.
pub trait ByteViewable {
    fn bytes(&self) -> &[u8];
}

/// A typed portable Signal can restore the contract value it carries.
pub trait Restorable<T> {
    fn restore(&self) -> Result<T, rkyv::rancor::Error>;
}

impl Signalizable for Query {
    fn signalize(&self) -> Result<Signal<Self>, rkyv::rancor::Error> {
        Ok(Signal {
            bytes: rkyv::to_bytes::<rkyv::rancor::Error>(self)?.to_vec(),
            target: PhantomData,
        })
    }
}

impl Signalizable for Response {
    fn signalize(&self) -> Result<Signal<Self>, rkyv::rancor::Error> {
        Ok(Signal {
            bytes: rkyv::to_bytes::<rkyv::rancor::Error>(self)?.to_vec(),
            target: PhantomData,
        })
    }
}

impl<T> From<Vec<u8>> for Signal<T> {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            target: PhantomData,
        }
    }
}

impl<T> ByteViewable for Signal<T> {
    fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl Restorable<Query> for Signal<Query> {
    fn restore(&self) -> Result<Query, rkyv::rancor::Error> {
        rkyv::from_bytes(self.bytes())
    }
}

impl Restorable<Response> for Signal<Response> {
    fn restore(&self) -> Result<Response, rkyv::rancor::Error> {
        rkyv::from_bytes(self.bytes())
    }
}

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
