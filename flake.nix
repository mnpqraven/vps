{
  description = "VPS control tools";

  inputs = {
    self.submodules = true;

    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs =
    {
      nixpkgs,
      naersk,
      flake-utils,
      self,
      nixpkgs-mozilla,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
          overlays = [
            (import nixpkgs-mozilla)
          ];
        };
        toolchain =
          (pkgs.rustChannelOf {
            rustToolchain = ./rust-toolchain.toml;
            sha256 = "sha256-442fNe+JZCKeR146x4Nh0O00XeAfPWMalJDbV+vJQNg=";
          }).rust;
        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };
        buildPackage =
          pname:
          naersk'.buildPackage {
            inherit pname;
            src = ./.;
            gitSubmodules = true;
            cargoBuildOptions =
              opts:
              opts
              ++ [
                "-p"
                pname
              ];
            cargoTestOptions =
              opts:
              opts
              ++ [
                "-p"
                pname
              ];
            PROTOC = with pkgs; lib.getExe protobuf;
          };

        vps-rpc = buildPackage "vps-rpc";
        cron-ddns = buildPackage "cron-ddns";

        # TODO: leptos build
        admin-site = naersk'.buildPackage {
          pname = "admin-site";
          src = ./.;
          gitSubmodules = true;
          cargoBuild = ''cargo leptos build'';
          PROTOC = with pkgs; lib.getExe protobuf;
        };

        rpcWeb = pkgs.writeShellScriptBin "rpcWeb" ''
          PORT=5005
          ${pkgs.grpcui}/bin/grpcui -port 5006 -plaintext localhost:$PORT || echo "is the gRPC server running on port $PORT ?"
        '';
        layout = pkgs.writeShellScriptBin "layout" ''
          ${pkgs.zellij}/bin/zellij -l .zellij/servers.kdl
        '';
      in
      {
        packages = {
          inherit
            # dev binaries
            rpcWeb
            layout
            # prod binaries
            admin-site
            vps-rpc
            cron-ddns
            ;
        };

        apps.rpcWeb = {
          type = "app";
          program = "${self.packages.${system}.rpcWeb}/bin/rpcWeb";
        };
        apps.layout = {
          type = "app";
          program = "${self.packages.${system}.layout}/bin/layout";
        };

        # nix develop
        devShell = pkgs.mkShell {
          # TODO: better env
          shellHook = ''
            export DATABASE_URL=postgres://postgres:postgres@localhost/mydatabase
            export RUSTFLAGS="--cfg erase_components"
          '';
          nativeBuildInputs = with pkgs; [
            tailwindcss_4
            rustup
            bacon
            # cron-ddns dep
            dig
            protobuf
            grpcui
            grpcurl
            sqlx-cli
            cargo-generate
            cargo-leptos
            leptosfmt
          ];
        };
      }
    );
}
