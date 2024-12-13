{
  description = "The engine that builds Ray Peat Rodeo from markdown to HTML";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk.url = "github:nix-community/naersk";
  };

  outputs = inputs: let
    pkgs = inputs.nixpkgs.legacyPackages.x86_64-linux;
    fenixPkgs = inputs.fenix.packages.x86_64-linux;
    rustToolchain = fenixPkgs.combine [
      fenixPkgs.latest.toolchain
      fenixPkgs.targets.wasm32-unknown-unknown.latest.rust-std
    ];
    naersk = pkgs.callPackage inputs.naersk {
      cargo = rustToolchain;
      rustc = rustToolchain;
    };
    rustCrate = (pkgs.lib.importTOML ./Cargo.toml).package;
  in {
    packages.x86_64-linux.rpr = naersk.buildPackage {
      name = rustCrate.name;
      version = rustCrate.version;
      src = pkgs.lib.cleanSource ./.;
      LEPTOS_OUTPUT_NAME = rustCrate.metadata.leptos.output-name;
    };
    packages.x86_64-linux.default = inputs.self.packages.x86_64-linux.rpr;

    devShells.x86_64-linux.default = pkgs.mkShell {
      packages = [
        pkgs.rust-analyzer 
        rustToolchain
        pkgs.bacon
        pkgs.cargo-leptos
        pkgs.sass
        pkgs.tailwindcss
      ];
      RUST_BACKTRACE = "full";
      RUST_LOG = "debug";
    };
  };
}
