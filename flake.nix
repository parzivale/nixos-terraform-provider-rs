{
  description = "Rust project built with crane + rust-overlay";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    crane.url = "github:ipetkov/crane";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} ({self, ...}: {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];

      perSystem = {system, ...}: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [inputs.rust-overlay.overlays.default];
        };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustToolchain;

        src = craneLib.cleanCargoSource ./.;

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src;
          nativeBuildInputs = [pkgs.protobuf];
        };

        myPackage = craneLib.buildPackage {inherit src cargoArtifacts;};
      in {
        packages.default = myPackage;

        checks = {
          inherit myPackage;
          clippy = craneLib.cargoClippy {
            inherit src cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          };
          fmt = craneLib.cargoFmt {inherit src;};
          tests = craneLib.cargoTest {inherit src cargoArtifacts;};
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
          packages = with pkgs; [
            protobuf
            rustToolchain
            cargo-watch
            cargo-expand
          ];
        };
      };
    });
}
