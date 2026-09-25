{
  description = "VPS control tools";

  inputs = {
    self.submodules = true;

    cargo-leptos = {
      url = "github:benwis/cargo-leptos";
      flake = false;
    };
    crane.url = "github:ipetkov/crane";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ (import inputs.rust-overlay) ];
        };

        inherit (pkgs) lib;

        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain (
          p: p.rust-bin.nightly.latest.default.override { targets = [ "wasm32-unknown-unknown" ]; }
        );
        src = craneLib.cleanCargoSource ./.;

        commonNativeBuildInputs = with pkgs; [
          pkg-config
          openssl
          # TODO: sqlx
          # wasm
          wasm-bindgen-cli
          binaryen
          cargo-generate
          cargo-leptos
          tailwindcss_4
        ];

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;
          strictDeps = true;

          nativeBuildInputs = commonNativeBuildInputs;
          buildInputs =
            with pkgs;
            [
              pkg-config
              openssl
            ]
            ++ lib.optionals pkgs.stdenv.hostPlatform.isDarwin [
              # Additional darwin specific inputs can be set here
              pkgs.libiconv
            ];
          # Additional environment variables can be set directly
          PROTOC = with pkgs; lib.getExe protobuf;
        };

        # Build *just* the cargo dependencies (of the entire workspace),
        # so we can reuse all of that work (e.g. via cachix) when running in CI
        # It is *highly* recommended to use something like cargo-hakari to avoid
        # cache misses when building individual top-level-crates
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        individualCrateArgs = commonArgs // {
          inherit cargoArtifacts;
          inherit (craneLib.crateNameFromCargoToml { inherit src; }) version;
          # NB: we disable tests since we'll run them all via cargo-nextest
          doCheck = false;
        };

        fileSetForCrate =
          crate:
          lib.fileset.toSource {
            root = ./.;
            fileset = lib.fileset.unions [
              ./Cargo.toml
              ./Cargo.lock
              (craneLib.fileset.commonCargoSources ./admin-site)
              ./user-root
              (craneLib.fileset.commonCargoSources ./cron-ddns)
              (craneLib.fileset.commonCargoSources ./database)
              (craneLib.fileset.commonCargoSources ./proto-build-help)
              (craneLib.fileset.commonCargoSources ./vps-api)
              (craneLib.fileset.commonCargoSources ./vps-rpc)
              (craneLib.fileset.commonCargoSources ./proto-types)
              (craneLib.fileset.commonCargoSources ./load-env)
              # build.rs reads .proto files
              ./proto-types/proto
              (craneLib.fileset.commonCargoSources crate)
            ];
          };

        # Build the top-level crates of the workspace as individual derivations.
        # This allows consumers to only depend on (and build) only what they need.
        # Though it is possible to build the entire workspace as a single derivation,
        # so this is left up to you on how to organize things
        #
        # Note that the cargo workspace must define `workspace.members` using wildcards,
        # otherwise, omitting a crate (like we do below) will result in errors since
        # cargo won't be able to find the sources for all members.
        cron-ddns = craneLib.buildPackage (
          individualCrateArgs
          // {
            pname = "cron-ddns";
            cargoExtraArgs = "-p cron-ddns";
            src = fileSetForCrate ./cron-ddns;
          }
        );
        user-root = craneLib.buildPackage (
          individualCrateArgs
          // {
            pname = "user-root";
            buildPhaseCargoCommand = ''
              cargoBuildLog=$(mktemp cargoBuildLogXXXX.json)
              cargo leptos build -p user-root --release >"$cargoBuildLog"
            '';
            installPhaseCommand = ''
              mkdir -p $out/bin
              cp target/release/user-root $out/bin/
              cp -r target/site $out/bin/
            '';
            src = fileSetForCrate ./user-root;
          }
        );
      in
      {
        checks = {
          inherit cron-ddns user-root;

          # Run clippy (and deny all warnings) on the workspace source,
          # again, reusing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          my-workspace-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );
        };

        # prod binaries
        packages = {
          inherit cron-ddns user-root;
        };

        apps = {
          cron-ddns = inputs.flake-utils.lib.mkApp { drv = cron-ddns; };
          user-root = inputs.flake-utils.lib.mkApp { drv = user-root; };
        };

        # nix develop
        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = inputs.self.checks.${system};

          # TODO: better env
          shellHook = ''
            export DATABASE_URL=postgres://postgres:postgres@localhost/mydatabase
            export RUSTFLAGS="--cfg erase_components"
          '';
          packages =
            with pkgs;
            [
              rust-analyzer
              just
              bacon
              grpcui
              grpcurl
              sqlx-cli
              dig
              protobuf
              leptosfmt
            ]
            ++ commonNativeBuildInputs;
        };
      }
    );
}
