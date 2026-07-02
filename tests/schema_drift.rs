use signal_frame::SignalOperationHeads;
use signal_mind::{KnowledgeSubject, MindRequest, TechnicalNodeKind, TechnicalRelationKind};

const CONCEPT_SCHEMA: &str = include_str!("../schema/signal-mind.concept.schema");

fn declaration_line(type_name: &str) -> &str {
    CONCEPT_SCHEMA
        .lines()
        .find(|line| line.trim_start().starts_with(type_name))
        .unwrap_or_else(|| panic!("concept schema must declare {type_name}"))
}

#[test]
fn concept_schema_declares_live_request_operation_heads() {
    for head in <MindRequest as SignalOperationHeads>::HEADS {
        let expected = format!("({head} ");
        assert!(
            CONCEPT_SCHEMA
                .lines()
                .any(|line| line.trim_start().starts_with(&expected)),
            "concept schema must declare live request operation head {head}"
        );
    }
}

#[test]
fn concept_schema_version_tracks_package_version() {
    let mut version_parts = env!("CARGO_PKG_VERSION").split('.');
    let major = version_parts.next().expect("major version");
    let minor = version_parts.next().expect("minor version");
    let patch = version_parts.next().expect("patch version");
    let expected = format!("(Version {major} {minor} {patch})");

    assert!(
        CONCEPT_SCHEMA
            .lines()
            .any(|line| line.trim_start() == expected),
        "concept schema must declare package version {expected}"
    );
}

#[test]
fn concept_schema_declares_technical_node_roots() {
    let key_family_line = declaration_line("TechnicalNodeKeyFamily ");
    let node_kind_line = declaration_line("TechnicalNodeKind ");

    for kind in TechnicalNodeKind::ALL {
        let name = format!("{kind:?}");
        assert!(
            node_kind_line.contains(&name),
            "concept schema must include TechnicalNodeKind::{name}"
        );
        assert!(
            key_family_line.contains(&name),
            "concept schema must include TechnicalNodeKeyFamily::{name}"
        );
    }

    for canonical_key in [
        "component:mind",
        "repo:signal-mind",
        "contract:signal-mind:ordinary",
        "storage:mind:sema",
        "schema:mind:technical",
        "table:mind:technical_nodes",
    ] {
        assert!(
            CONCEPT_SCHEMA.contains(canonical_key),
            "concept schema must document canonical technical key {canonical_key}"
        );
    }
}

#[test]
fn concept_schema_declares_split_technical_relation_roots() {
    let relation_kind_line = declaration_line("TechnicalRelationKind ");

    for kind in TechnicalRelationKind::ALL {
        let name = format!("{kind:?}");
        assert!(
            relation_kind_line.contains(&name),
            "concept schema must include TechnicalRelationKind::{name}"
        );
    }

    assert!(
        !relation_kind_line.contains("DependsOn"),
        "concept schema must not collapse split dependency kinds back to DependsOn"
    );
}

#[test]
fn concept_schema_declares_accepted_knowledge_roots() {
    for required in [
        "AcceptedKnowledge",
        "KnowledgeIdentity",
        "KnowledgeSubject",
        "KnowledgeSubmission",
        "KnowledgeJudgeVerdict",
        "KnowledgeAccepted",
        "KnowledgeFound",
        "KnowledgeNotFound",
        "KnowledgeRejectionReason",
        "(Submit KnowledgeSubmission)",
        "(Get KnowledgeIdentity)",
        "Accepted",
        "Rejected",
        "Found",
        "NotFound",
    ] {
        assert!(
            CONCEPT_SCHEMA.contains(required),
            "concept schema must document knowledge contract fact: {required}"
        );
    }

    let subject_line = declaration_line("KnowledgeSubject ");
    for subject in [
        KnowledgeSubject::Component,
        KnowledgeSubject::Contract,
        KnowledgeSubject::Repository,
        KnowledgeSubject::Architecture,
        KnowledgeSubject::Interface,
        KnowledgeSubject::Storage,
        KnowledgeSubject::Source,
    ] {
        let name = format!("{subject:?}");
        assert!(
            subject_line.contains(&name),
            "concept schema must include KnowledgeSubject::{name}"
        );
    }

    for forbidden in [
        "KnowledgeIdentitySlot",
        "KnowledgeCandidate",
        "Keyed",
        "Unkeyed",
        "GetByIdentity",
        "SubmitKnowledge",
        "QueryKnowledge",
    ] {
        assert!(
            !CONCEPT_SCHEMA.contains(forbidden),
            "concept schema must not retain old knowledge surface: {forbidden}"
        );
    }
}

#[test]
fn concept_schema_declares_technical_graph_query_shapes() {
    let technical_node_query_line = declaration_line("TechnicalNodeQuery ");
    for variant in [
        "Filter",
        "About",
        "RelationNeighborhood",
        "DependencyClosure",
        "ProvenanceChain",
    ] {
        assert!(
            technical_node_query_line.contains(variant),
            "concept schema must include TechnicalNodeQuery::{variant}"
        );
    }

    for type_name in [
        "AboutTechnicalNode ",
        "TechnicalRelationNeighborhoodQuery ",
        "TechnicalRelationNeighborhoodDirection ",
        "TechnicalDependencyClosureQuery ",
        "TechnicalProvenanceChainQuery ",
        "TechnicalNodeNeighborhood ",
        "TechnicalDependencyClosure ",
        "TechnicalProvenanceChain ",
    ] {
        declaration_line(type_name);
    }
}

#[test]
fn concept_schema_documents_subscription_lifecycle_bounds() {
    for required in [
        "SubscriptionRetraction is the typed close request",
        "SubscriptionRetracted is the final acknowledgement",
        "bounded buffer metadata",
        "does not promise a durable outbox",
    ] {
        assert!(
            CONCEPT_SCHEMA.contains(required),
            "concept schema must document subscription lifecycle fact: {required}"
        );
    }
}
