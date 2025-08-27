{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  inputsFrom = [ (pkgs.callPackage ./default.nix { }) ];

  buildInputs = with pkgs; [
    rust-analyzer
    rustfmt
    clippy
    trunk
    zlib
  ];

  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [ pkgs.zlib ]}"
  '';
}
