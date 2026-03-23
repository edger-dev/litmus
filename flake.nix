{
  description = "litmus — terminal color theme previewer";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        toolchain = fenix.packages.${system}.combine [
          fenix.packages.${system}.stable.rustc
          fenix.packages.${system}.stable.cargo
          fenix.packages.${system}.stable.clippy
          fenix.packages.${system}.stable.rustfmt
          fenix.packages.${system}.targets.wasm32-unknown-unknown.stable.rust-std
        ];

        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        src = craneLib.cleanCargoSource ./.;

        commonArgs = {
          inherit src;
          pname = "litmus";
          strictDeps = true;
          # Build only the CLI (native target); web is built via dx in devShell
          cargoExtraArgs = "--package litmus-cli";
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        litmus-cli = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
        });
      in
      {
        packages = {
          inherit litmus-cli;
          default = litmus-cli;
        };

        checks = {
          inherit litmus-cli;

          clippy = craneLib.cargoClippy (commonArgs // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--package litmus-cli -- --deny warnings";
          });

          fmt = craneLib.cargoFmt {
            inherit src;
          };
        };

        devShells.default = craneLib.devShell {
          checks = self.checks.${system};
          packages = [
            pkgs.sccache
            pkgs.mise
            pkgs.dioxus-cli
            pkgs.wasm-bindgen-cli
            pkgs.cage             # Wayland kiosk compositor for headless capture
            pkgs.grim             # Wayland screenshot tool
            pkgs.foot             # Wayland terminal (SHM-based, no OpenGL needed)
            pkgs.kitty            # Kitty terminal (requires real display with OpenGL)
            pkgs.fira-code
            pkgs.mdbook
            pkgs.bacon
          ];
        };
      });
}
