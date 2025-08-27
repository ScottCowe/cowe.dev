{
  system ? builtins.currentSystem,
  pkgs ? import <nixpkgs> { inherit system; },
}:
let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  fenix = import (fetchTarball {
    url = "https://github.com/nix-community/fenix/archive/main.tar.gz";
    sha256 = "sha256:1mjmp9c5x3j6jqy50ymr07nxl4nrlp3iininxhnzc1ragdrg8pxn";
  }) { inherit system; };
  rust-toolchain =
    with fenix;
    combine [
      default.toolchain
      complete.rust-src
      targets.wasm32-unknown-unknown.latest.rust-std
    ];

  rust-platform = pkgs.makeRustPlatform {
    rustc = rust-toolchain;
    cargo = rust-toolchain;
  };
in
rust-platform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;
  src = pkgs.lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;
  cargoLock.outputHashes = {
    "yew-0.21.0" = pkgs.lib.fakeSha256;
  };

  nativeBuildInputs = with pkgs; [
    trunk
    wasm-bindgen-cli
    nodePackages.sass
  ];

  buildPhase = "trunk build";
  installPhase = "cp -r dist $out";

  # TODO: Use temp dir
  XDG_CACHE_HOME = "/build/cache";
}
