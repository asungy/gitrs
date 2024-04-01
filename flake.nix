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
          nativeBuildInputs = [ pkgs.pkg-config ];
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      in
    {
      packages.default = gitrs;

      apps.default = flake-utils.lib.mkApp { drv = gitrs; };

      devShells.default = {
        checks = self.checks.${system};
      };
    });
}
