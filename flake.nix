{
  description = "Bifrost developer environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      # nixos-unstable (26.11) dropped x86_64-darwin; Intel Macs use rustup with rust-toolchain.toml.
      forAllSystems = nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
    in
    {
      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };

          # One pin for the flake and for rustup users: rust-toolchain.toml.
          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          default = pkgs.mkShell {
            # Checked by the justfile's `_devshell` guard: recipes run only inside this shell.
            BIFROST_DEVSHELL = "1";

            packages = [
              rustToolchain
            ] ++ (with pkgs; [
              just
              lefthook
              rumdl
              cargo-deny
              cargo-nextest
              gh
              jq
              actionlint
              shellcheck
              pkg-config
            ]) ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux (with pkgs; [
              # Linux only: mold is not available on macOS, and a Nix clang
              # would shadow Apple's toolchain there. Linker wiring is #40.
              clang
              mold
            ]);
          };
        }
      );
    };
}
