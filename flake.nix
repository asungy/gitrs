{
  description = "Git written in Rust for fun.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        craneLib = crane.lib.${system};

        gitrs = craneLib.buildPackage {
          src = craneLib.cleanCargoSource (craneLib.path ./.);
          nativeBuildInputs = with pkgs; [ pkg-config openssl ];
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      in
    {
      checks = {
        inherit gitrs;
      };

      packages.default = gitrs;

      apps.default = flake-utils.lib.mkApp { drv = gitrs; };

      devShells.default = craneLib.devShell {
        checks = self.checks.${system};
      };
    });
}
