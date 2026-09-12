{
  description = "signal-mind — Signal contract for `mind` CLI ↔ mind work graph, mind graph, technical dependency memory, and channel choreography";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-build = {
      url = "github:LiGoldragon/rust-build";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-build }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust = rust-build.lib.${system}.fromToolchainFile pkgs {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-gh/xTkxKHL4eiRXzWv8KP7vfjSk61Iq48x47BEDFgfk=";
        };

        inherit (rust) craneLib toolchain;

        # The authored ethos source and the committed generation are both
        # build inputs: `src/lib.rs` reads the ethos with `include_str!` and
        # `build.rs` asserts the committed Rust matches a fresh generation.
        # `examples/canonical.datom` is read by the contract test.
        examplesFilter = path: _type: builtins.match ".*/examples(/.*)?$" path != null;
        contractFilter = path: type:
          type == "regular" && (
            pkgs.lib.hasSuffix ".ethos" path ||
            pkgs.lib.hasSuffix "/build.rs" path ||
            builtins.match ".*/src/generated(/.*)?$" path != null
          );
        src = rust.cleanSource {
          root = ./.;
          extraFilters = [ examplesFilter contractFilter ];
        };
        commonArgs = { inherit src; strictDeps = true; };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in
      {
        packages.default = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
        checks = {
          build = craneLib.cargoBuild (commonArgs // { inherit cargoArtifacts; });
          test = craneLib.cargoTest (commonArgs // { inherit cargoArtifacts; });
          test-generated-contract = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
            cargoTestExtraArgs = "--test generated_contract";
          });
          test-datom = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
            cargoTestExtraArgs = "--all-features --test generated_contract";
          });
          test-canonical-examples = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
            cargoTestExtraArgs = "--all-features --test generated_contract every_canonical_example_actualizes";
          });
          test-doc = craneLib.cargoTest (commonArgs // {
            inherit cargoArtifacts;
            cargoTestExtraArgs = "--doc";
          });
          doc = craneLib.cargoDoc (commonArgs // {
            inherit cargoArtifacts;
            RUSTDOCFLAGS = "-D warnings";
          });
          fmt = craneLib.cargoFmt { inherit src; };
          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets --all-features -- -D warnings";
          });
        };
        devShells.default = pkgs.mkShell {
          name = "signal-mind";
          packages = [ pkgs.jujutsu pkgs.pkg-config toolchain ];
        };
      });
}
