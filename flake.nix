{
  description = "Bifrost developer environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # Pinned to a commit, not a tag: the single `tools` binary and the [workflow] block exist only on
    # main (the unreleased v0.3.0). Move the pin with `nix flake update tools` once v0.3.0 is tagged.
    tools = {
      url = "github:jaoppb/tools/69c732670a9b65831d3ea91de68e1a1b7b1f5149";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, tools, ... }:
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

          # nixpkgs' convco is built against a system libgit2 that is missing from the pkg-config
          # path, so its build fails (checked on nixos-unstable, 2026-11). Unsetting the flag makes
          # libgit2-sys build its vendored copy. Drop the override once nixpkgs builds convco again.
          convco-vendored-libgit2 = pkgs.convco.overrideAttrs (old: {
            preBuild = "unset LIBGIT2_NO_VENDOR\n" + (old.preBuild or "");
          });
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
              convco-vendored-libgit2
              rumdl
              cargo-deny
              cargo-nextest
              gh
              jq
              actionlint
              shellcheck
              pkg-config
            ]) ++ [
              # The `tools` binary (issues, repo and project checks), built from the pinned flake input.
              tools.packages.${system}.tools
            ] ++ pkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux (with pkgs; [
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
