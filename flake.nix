{
  description = "cowe.dev microvm";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    microvm.url = "github:microvm-nix/microvm.nix";
    microvm.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      microvm,
      ...
    }:
    {
      nixosConfigurations.cowe-dev = nixpkgs.lib.nixosSystem rec {
        system = "x86_64-linux";
        pkgs = import nixpkgs { system = "${system}"; };
        modules = [
          microvm.nixosModules.microvm
          {
            networking.hostName = "cowe-dev";
            microvm.hypervisor = "qemu";

            users.users.root.initialPassword = "password";

            networking.firewall.allowedTCPPorts = [
              80
              443
            ];

            users.users.nginx.extraGroups = [ "acme" ];

            security.acme = {
              acceptTerms = true;
              certs = {
                "cowe.dev".email = "scott.t.cowe@gmail.com";
              };
            };

            services.nginx = {
              enable = true;
              virtualHosts = {
                "cowe.dev" =
                  let
                    cowe-dev-root = pkgs.runCommandLocal "cowe-dev-root" { } ''
                      mkdir -p $out 
                      cp ${./index.html} $out/index.html
                      cp ${./index.css} $out/index.css
                    '';
                  in
                  {
                    enableACME = true;
                    forceSSL = true;
                    root = cowe-dev-root;
                  };
              };
            };
          }
        ];
      };
    };
}
