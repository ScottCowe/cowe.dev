{
  description = "cowe.dev";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    # nixpkgs.url = "path:/home/cowe/repos/nixpkgs";
    # nixpkgs.url = "github:ScottCowe/nixpkgs/add-wasm-bindgen-cli-0-2-103"; # cuz 2.101 aint even in nixpkgs yet

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    wasm-bindgen-cli-flake = {
      url = "github:fusion44/wasm-bindgen-cli-flake";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      self,
      nixpkgs,
      fenix,
      ...
    }@inputs:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;
      fenixFor = fenix.packages;
    in
    {
      packages = forAllSystems (
        system:
        let
          rust-toolchain =
            with fenixFor.${system};
            combine [
              default.toolchain
              complete.rust-src
              targets.wasm32-unknown-unknown.latest.rust-std
            ];

          rust-platform = pkgsFor.${system}.makeRustPlatform {
            rustc = rust-toolchain;
            cargo = rust-toolchain;
          };
        in
        {
          frontend =
            let
              manifest = (pkgsFor.${system}.lib.importTOML ./frontend/Cargo.toml).package;
            in
            rust-platform.buildRustPackage {
              pname = manifest.name;
              version = manifest.version;
              src = pkgsFor.${system}.lib.cleanSource ./.;

              cargoLock.lockFile = ./Cargo.lock;

              nativeBuildInputs = with pkgsFor.${system}; [
                trunk
                inputs.wasm-bindgen-cli-flake.packages.${system}.wasm-bindgen-cli
                nodePackages.sass
                binaryen
              ];

              buildPhase = ''
                cd frontend
                trunk build --offline --release
              '';
              installPhase = ''
                cp -r dist $out
              '';

              # TODO: Use temp dir
              XDG_CACHE_HOME = "/build/cache";
            };

          backend =
            let
              manifest = (pkgsFor.${system}.lib.importTOML ./backend/Cargo.toml).package;
            in
            rust-platform.buildRustPackage {
              pname = manifest.name;
              version = manifest.version;
              cargoLock.lockFile = ./Cargo.lock;
              src = pkgsFor.${system}.lib.cleanSource ./.;

              buildPhase = "cargo build -p backend";
              installPhase = ''
                mkdir -p $out/bin
                cp -r ./target/debug/backend $out/bin
              '';
            };
        }
      );

      devShells = forAllSystems (system: {
        default = pkgsFor.${system}.callPackage ./shell.nix {
          pkgs = (pkgsFor.${system});
          fenix = (fenixFor.${system});
        };
      });
    };
}
