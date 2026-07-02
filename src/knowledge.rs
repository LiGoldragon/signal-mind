use nota::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

use crate::{ActorName, TextBody, TimestampNanos};

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
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
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum KnowledgeSubject {
    Component,
    Contract,
    Repository,
    Architecture,
    Interface,
    Storage,
    Source,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeSubmission {
    pub subject: KnowledgeSubject,
    pub statement: TextBody,
}

impl KnowledgeSubmission {
    pub fn new(subject: KnowledgeSubject, statement: impl Into<String>) -> Self {
        Self {
            subject,
            statement: TextBody::new(statement),
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct AcceptedKnowledge {
    pub identity: KnowledgeIdentity,
    pub subject: KnowledgeSubject,
    pub statement: TextBody,
    pub accepted_by: ActorName,
    pub accepted_at: TimestampNanos,
}

impl AcceptedKnowledge {
    pub fn public_record(&self) -> KnowledgeRecord {
        KnowledgeRecord {
            identity: self.identity.clone(),
            subject: self.subject,
            statement: self.statement.clone(),
        }
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeRecord {
    pub identity: KnowledgeIdentity,
    pub subject: KnowledgeSubject,
    pub statement: TextBody,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct KnowledgeJudgePacket {
    pub subject: KnowledgeSubject,
    pub statement: TextBody,
    pub relevant_neighbors: Vec<AcceptedKnowledge>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeJudgeVerdict {
    Accept,
    Reject(KnowledgeRejectionReason),
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub enum KnowledgeRejectionReason {
    NotKnowledge,
    PrivateOrUnauthorized,
    MeaningUnclear,
    FalseOrUnsupported,
    SemanticDuplicate(KnowledgeIdentity),
    ConflictsAcceptedKnowledge(Vec<KnowledgeIdentity>),
    WrongSubject(KnowledgeSubject),
    NeedsMoreSpecificShape,
    SourceRequired,
    PersistenceRejected,
}
