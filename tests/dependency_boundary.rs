#[test]
fn mind_contract_declares_local_nota_text_feature_for_signal_frame_macros() {
    let cargo_toml = include_str!("../Cargo.toml");

    assert!(
        cargo_toml.contains("default = [\"nota-text\"]"),
        "direct signal-mind users keep the NOTA projection by default",
    );
    assert!(
        cargo_toml.contains("nota-text = [\"signal-frame/nota-text\"]"),
        "signal-frame macro-generated NOTA traits are gated through a local feature",
    );
}

#[test]
fn mind_contract_consumes_shared_domain_from_portable_remote_contract() {
    let cargo_toml = include_str!("../Cargo.toml");

    assert!(
        cargo_toml.contains("signal-domain")
            && cargo_toml.contains("https://github.com/LiGoldragon/signal-domain.git")
            && cargo_toml.contains("features = [\"nota-text\"]"),
        "signal-mind must consume the shared Domain contract from the portable remote with canonical NOTA support",
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
        signal_domain_line.contains("branch = \"main\"") && !signal_domain_line.contains("rev ="),
        "signal-domain must converge on the published canonical producer revision",
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
        signal_persona_line.contains("branch = \"main\"") && !signal_persona_line.contains("rev ="),
        "signal-mind and router must consume one published signal-persona producer revision",
    );

    let retired_direct_dependencies: Vec<_> = cargo_toml
        .lines()
        .filter(|line| {
            let trimmed = line.trim_start();
            !trimmed.starts_with("[patch.")
                && [
                    "schema-next.git",
                    "schema-rust-next.git",
                    "nota-next.git",
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
