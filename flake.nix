{
  description = "Baby's first software renderer";
   
  inputs = {
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
  };
  
  outputs = { self, flake-compat, nixpkgs, rust-overlay, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        pkg-version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
        rust-version = "1.71.0";
        rust-dist = pkgs.rust-bin.stable.${rust-version}.default.override {
          extensions = [ "clippy" "rust-src" "rustfmt" "rust-analyzer" ];
          targets = [ "x86_64-unknown-linux-gnu" ];
        };
      in {
        defaultPackage = with pkgs; rustPlatform.buildRustPackage {
          pname = "babygfx";
          version = pkg-version;
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [
            rust-dist
          ];
          buildInputs = [
            SDL2
          ];
        };
  
        defaultApp = utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };

        devShell = with pkgs; mkShell {
          buildInputs = [
            # Project tools
            cargo-msrv
            nixfmt
            rust-dist

	    # Deps
	    SDL2
          ];

          shellHook = ''
  
          '';
  
          RUST_BACKTRACE = "1";
        };
      }
    );
  }
