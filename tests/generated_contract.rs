use signal_mind::{
    ByteViewable, Magnitude, MemoryQuery, Query, QueryKind, Response, Restorable, Signal,
    Signalizable, ThoughtCommitted,
};

#[test]
fn query_and_response_round_trip_through_received_bytes() {
    let query = Query::Query(MemoryQuery {
        query_kind: QueryKind::Ready,
        query_limit: 20,
    });
    let received =
        Signal::<Query>::from(query.signalize().expect("query archives").bytes().to_vec());
    assert_eq!(received.restore().expect("query restores"), query);

    let response = Response::ThoughtCommitted(ThoughtCommitted {
        record_identifier: "record-1".into(),
        display_identifier: "mind-7".into(),
        timestamp_nanos: 1_757_600_000_000_000_000,
    });
    let received = Signal::<Response>::from(
        response
            .signalize()
            .expect("response archives")
            .bytes()
            .to_vec(),
    );
    assert_eq!(received.restore().expect("response restores"), response);
}

#[test]
fn malformed_archive_is_rejected() {
    assert!(Signal::<Query>::from(vec![255, 0, 1]).restore().is_err());
    assert!(Signal::<Response>::from(vec![255, 0, 1]).restore().is_err());
}

#[test]
fn magnitude_orders_zero_below_every_other_rung() {
    assert!(Magnitude::Zero < Magnitude::Minimum);
    assert!(Magnitude::Minimum < Magnitude::VeryLow);
    assert!(Magnitude::VeryLow < Magnitude::Low);
    assert!(Magnitude::Low < Magnitude::Medium);
    assert!(Magnitude::Medium < Magnitude::High);
    assert!(Magnitude::High < Magnitude::VeryHigh);
    assert!(Magnitude::VeryHigh < Magnitude::Maximum);
}

#[cfg(feature = "datom")]
fn budget() -> datom_codec::Budget {
    datom_codec::Budget {
        remaining: 1 << 20,
        reader: protos::ReaderBudget { remaining: 1 << 20 },
        depth: 0,
        maximum_depth: 1024,
    }
}

#[cfg(feature = "datom")]
#[test]
fn query_and_response_round_trip_as_datom_text() {
    use datom_codec::{Actualizing, Datomizable, Potential};
    use protos::{Protosizable, Textualizable};

    let query = Query::Query(MemoryQuery {
        query_kind: QueryKind::ByKind(signal_mind::ItemKind::Defect),
        query_limit: 5,
    });
    let text = query.clone().datomize(vec![]).protosize().textualize();
    assert_eq!(
        Potential::<Query>::from(text)
            .actualize(&mut budget())
            .expect("query actualizes"),
        query
    );

    let response = Response::NotFound;
    let text = response.clone().datomize(vec![]).protosize().textualize();
    assert_eq!(
        Potential::<Response>::from(text)
            .actualize(&mut budget())
            .expect("response actualizes"),
        response
    );
}

/// Every non-comment line of `examples/canonical.datom` must actualize as a
/// `Query` or a `Response`. An example that no longer parses fails the gate.
#[cfg(feature = "datom")]
#[test]
fn every_canonical_example_actualizes() {
    use datom_codec::{Actualizing, Potential};

    let canonical = include_str!("../examples/canonical.datom");
    let mut seen = 0usize;
    for (number, line) in canonical.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') {
            continue;
        }
        seen += 1;
        let as_query = Potential::<Query>::from(line).actualize(&mut budget());
        if as_query.is_ok() {
            continue;
        }
        let as_response = Potential::<Response>::from(line).actualize(&mut budget());
        assert!(
            as_response.is_ok(),
            "examples/canonical.datom line {} is neither a Query nor a Response: {line}\n\
             as Query: {:?}\nas Response: {:?}",
            number + 1,
            as_query.err(),
            as_response.err()
        );
    }
    assert!(seen >= 33, "expected the canonical examples, saw {seen}");
}
