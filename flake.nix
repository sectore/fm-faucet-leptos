{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        target = "wasm32-unknown-unknown";
        fnx = fenix.packages.${system};
        toolchainSpec = {
          channel = "nightly";
          date = "2023-07-21";
          sha256 = "sha256-8ve5pIwa7Z6Sr/bL83w27K2OpxzZcFzdiF5YxKup8Y4=";
        };
        nightly = fnx.toolchainOf toolchainSpec;
        nightly-std = fnx.targets.${target}.toolchainOf toolchainSpec;
        fenixToolchain = fnx.combine [
          nightly.cargo
          nightly.rustc
          nightly-std.rust-std
        ];
        craneLib = crane.lib.${system}.overrideToolchain fenixToolchain;
      in
    {
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ fenixToolchain trunk just tailwindcss pkg-config openssl.dev openssl ];
      };
    });
}
