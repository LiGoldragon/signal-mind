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

/// One generic transport, written against `signal`'s kinds alone, carries a
/// mind frame and a Persona frame. That is only possible because both
/// contracts speak `signal`'s frame rather than each vendoring its own; a
/// vendored copy is a distinct Rust type and this function would not accept
/// both.
#[test]
fn one_generic_transport_carries_mind_and_persona_frames() {
    fn ship<T>(value: &T) -> Vec<u8>
    where
        T: signal::Signalizable,
        signal::Signal<T>: signal::ByteViewable,
    {
        use signal::ByteViewable;
        value.signalize().expect("archive").bytes().to_vec()
    }
    fn land<T>(bytes: Vec<u8>) -> T
    where
        signal::Signal<T>: signal::Restorable<T>,
    {
        use signal::Restorable;
        signal::Signal::<T>::from(bytes).restore().expect("restore")
    }

    let mind_query = Query::Query(MemoryQuery {
        query_kind: QueryKind::Ready,
        query_limit: 20,
    });
    let persona_query = signal_persona::Query::Stop(String::from("router"));

    let landed_mind: Query = land(ship(&mind_query));
    let landed_persona: signal_persona::Query = land(ship(&persona_query));
    assert_eq!(landed_mind, mind_query);
    assert_eq!(landed_persona, persona_query);

    // The two contracts' re-exported frame names denote one type.
    let framed: Signal<Query> = signal::Signal::<Query>::from(ship(&mind_query));
    let persona_framed: signal_persona::Signal<signal_persona::Query> =
        signal::Signal::from(ship(&persona_query));
    assert_eq!(
        <Signal<Query> as Restorable<Query>>::restore(&framed).expect("restore"),
        mind_query
    );
    assert_eq!(
        signal::Restorable::restore(&persona_framed).expect("restore"),
        persona_query
    );
}
