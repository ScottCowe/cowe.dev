{
  # system ? builtins.currentSystem,
  pkgs,
  # ? import <nixpkgs> { inherit system; },
  fenix,
}:
let
  # fenix = import (fetchTarball {
  #   url = "https://github.com/nix-community/fenix/archive/main.tar.gz";
  #   sha256 = "sha256:1mjmp9c5x3j6jqy50ymr07nxl4nrlp3iininxhnzc1ragdrg8pxn";
  # }) { inherit system; };

  rust-toolchain =
    with fenix;
    combine [
      default.toolchain
      complete.rust-src
      targets.wasm32-unknown-unknown.latest.rust-std
    ];
in
pkgs.mkShell {
  buildInputs = with pkgs; [
    rust-toolchain
    rust-analyzer
    rustfmt
    clippy
    trunk
    zlib
    cargo-watch
    postgresql
    sqlx-cli
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [ pkgs.zlib ]}"
    ./postgres.sh start

    trap "./postgres.sh stop" EXIT
  '';
}
