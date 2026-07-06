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
}
