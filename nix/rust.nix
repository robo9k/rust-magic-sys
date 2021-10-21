{ sources ? import ./sources.nix }:

let
  pkgs =
    import sources.nixpkgs { overlays = [ (import sources.rust-overlay) ]; };
  rust = pkgs.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;
in rust
