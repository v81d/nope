{pkgs ? import <nixpkgs> {}}: let
  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {};
in
  pkgs.mkShell {
    packages = with pkgs; [
      fenix.stable.toolchain
      pkg-config
      gcc
      cargo-watch
      cargo-edit
    ];

    RUST_BACKTRACE = 1;
    RUST_LOG = "debug";
    shellHook = ''zsh'';
  }
