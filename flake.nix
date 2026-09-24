{
  description = "VPS control tools";

  inputs = {
    self.submodules = true;

    crane.url = "github:ipetkov/crane";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      crane,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        inherit (pkgs) lib;

        craneLib = crane.mkLib pkgs;
        src = craneLib.cleanCargoSource ./.;

        # Common arguments can be set here to avoid repeating them later
        commonArgs = {
          inherit src;
          strictDeps = true;

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          buildInputs =
            with pkgs;
            [
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
              # (craneLib.fileset.commonCargoSources ./crates/my-common)
              # (craneLib.fileset.commonCargoSources ./crates/my-workspace-hack)
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

        # TODO: leptos build
        # admin-site = naersk'.buildPackage {
        #   pname = "admin-site";
        #   src = ./.;
        #   gitSubmodules = true;
        #   cargoBuild = "cargo leptos build";
        #   PROTOC = with pkgs; lib.getExe protobuf;
        # };
      in
      {
        checks = {
          inherit cron-ddns;

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
          inherit cron-ddns;
        };

        apps = {
          cron-ddns = flake-utils.lib.mkApp {
            drv = cron-ddns;
          };
        };

        # nix develop
        devShells.default = craneLib.devShell {
          # Inherit inputs from checks.
          checks = self.checks.${system};

          # TODO: better env

          shellHook = ''
            export DATABASE_URL=postgres://postgres:postgres@localhost/mydatabase
            export RUSTFLAGS="--cfg erase_components"
          '';
          packages = with pkgs; [
            rust-analyzer

            just
            # TODO: fix autocomplete error
            # rustc
            # cargo
            tailwindcss_4
            rustup
            bacon
            grpcui
            grpcurl
            sqlx-cli

            rustup
            # cron-ddns dep
            dig
            protobuf
            # TLS
            pkg-config
            openssl

            # for wasm-opt building on release
            binaryen
            cargo-generate
            cargo-leptos
            leptosfmt
          ];
        };
      }
    );
}
