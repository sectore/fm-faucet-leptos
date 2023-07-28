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
    fedimint.url = "github:fedimint/fedimint";
  };

  outputs = { self, nixpkgs, crane, flake-utils, fenix, fedimint, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        target = "wasm32-unknown-unknown";
        fnx = fenix.packages.${system};
        toolchainSpec = {
          channel = "nightly";
          date = "2023-07-28";
          sha256 = "sha256-c0GN2qV5sJYl6/QoM9IQDByWyzs2rRZ6nBMSTghqkvc=";
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
        inputsFrom = [ fedimint.devShells."${system}".crossWasm ];
        nativeBuildInputs = with pkgs; [ fenixToolchain trunk just tailwindcss pkg-config openssl.dev openssl ];
      };
    });
}
