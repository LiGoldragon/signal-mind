#[test]
fn mind_contract_declares_local_dotos_text_feature_for_signal_frame_macros() {
    let cargo_toml = include_str!("../Cargo.toml");

    assert!(
        cargo_toml.contains("default = [\"dotos-text\"]"),
        "direct signal-mind users keep the DOTOS projection by default",
    );
    assert!(
        cargo_toml.contains("dotos-text = [\"signal-frame/dotos-text\"]"),
        "signal-frame macro-generated DOTOS traits are gated through a local feature",
    );
}

#[test]
fn mind_contract_consumes_shared_domain_from_portable_remote_contract() {
    let cargo_toml = include_str!("../Cargo.toml");

    assert!(
        cargo_toml.contains("signal-domain")
            && cargo_toml.contains("https://github.com/LiGoldragon/signal-domain.git")
            && cargo_toml.contains("features = [\"dotos-text\"]"),
        "signal-mind must consume the shared Domain contract from the portable remote with canonical DOTOS support",
    );
    let signal_domain_line = cargo_toml
        .lines()
        .find(|line| line.trim_start().starts_with("signal-domain"))
        .expect("signal-domain dependency line");
    assert!(
        !signal_domain_line.contains("path =") && !signal_domain_line.contains("git+file://"),
        "shared contract dependency must not be a local filesystem input",
    );
    assert!(
        signal_domain_line.contains("rev = \"1890f33174f3637b2d605b9c79e584e127580d13\""),
        "signal-domain must pin the published canonical DOTOS producer revision",
    );
}

#[test]
fn mind_contract_pins_trueschema_family_without_retired_direct_dependencies() {
    let cargo_toml = include_str!("../Cargo.toml");

    let signal_persona_line = cargo_toml
        .lines()
        .find(|line| line.trim_start().starts_with("signal-persona"))
        .expect("signal-persona dependency line");
    assert!(
        signal_persona_line.contains("rev = \"51ee97b12fef5dfd8c4bbf876dc3e4f0ee14df5a\""),
        "signal-mind must consume the published canonical DOTOS signal-persona producer revision",
    );

    let retired_direct_dependencies: Vec<_> = cargo_toml
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !trimmed.starts_with("[patch.")
                && [
                    "schema-next.git",
                    "schema-rust-next.git",
                    "drop-next",
                ]
                .iter()
                .any(|retired| line.contains(retired))
        })
        .collect();
    assert!(
        retired_direct_dependencies.is_empty(),
        "retired producer repositories may only appear as patched source identities: {retired_direct_dependencies:?}",
    );
}
