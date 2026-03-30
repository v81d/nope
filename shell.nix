{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  packages = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    pkg-config
    gcc
    cargo-watch
    cargo-edit
  ];

  RUST_BACKTRACE = 1;
  RUST_LOG = "debug";

  shellHook = ''
    zsh
  '';
}
