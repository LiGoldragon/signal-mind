use dotos::{DotosDecode, DotosEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use signal_domain::Domain;

use crate::{ActorName, TextBody, TimestampNanos};

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    DotosEncode,
    DotosDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct KnowledgeIdentity(String);

impl KnowledgeIdentity {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, DotosEncode, DotosDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeSubmission {
    pub domain: Domain,
    pub statement: TextBody,
}

impl KnowledgeSubmission {
    pub fn new(domain: Domain, statement: impl Into<String>) -> Self {
        Self {
            domain,
            statement: TextBody::new(statement),
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, DotosEncode, DotosDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AcceptedKnowledge {
    pub identity: KnowledgeIdentity,
    pub domain: Domain,
    pub statement: TextBody,
    pub accepted_by: ActorName,
    pub accepted_at: TimestampNanos,
}

impl AcceptedKnowledge {
    pub fn public_record(&self) -> KnowledgeRecord {
        KnowledgeRecord {
            identity: self.identity.clone(),
            domain: self.domain.clone(),
            statement: self.statement.clone(),
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, DotosEncode, DotosDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeRecord {
    pub identity: KnowledgeIdentity,
    pub domain: Domain,
    pub statement: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, DotosEncode, DotosDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeJudgePacket {
    pub domain: Domain,
    pub statement: TextBody,
    pub relevant_neighbors: Vec<AcceptedKnowledge>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, DotosEncode, DotosDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeJudgeResponse {
    pub verdict: KnowledgeJudgeVerdict,
    pub diagnostic_message: Option<TextBody>,
}

impl KnowledgeJudgeResponse {
    pub fn new(verdict: KnowledgeJudgeVerdict) -> Self {
        Self {
            verdict,
            diagnostic_message: None,
        }
    }

    pub fn with_diagnostic_message(mut self, message: impl Into<String>) -> Self {
        self.diagnostic_message = Some(TextBody::new(message));
        self
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, DotosEncode, DotosDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeJudgeVerdict {
    Accept,
    Reject(KnowledgeRejectionReason),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, DotosEncode, DotosDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeRejectionReason {
    NotKnowledge,
    PrivateOrUnauthorized,
    MeaningUnclear,
    SemanticDuplicate(KnowledgeIdentity),
    ConflictsAcceptedKnowledge(Vec<KnowledgeIdentity>),
    WrongDomain(Domain),
    NeedsMoreSpecificShape,
    PersistenceRejected,
}
