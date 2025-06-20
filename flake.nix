{
  description = "VPS control tools";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      self,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = (import nixpkgs) {
          inherit system;
        };
        rpcWeb = pkgs.writeShellScriptBin "rpcWeb" ''
          PORT=5005
          ${pkgs.grpcui}/bin/grpcui -plaintext localhost:$PORT || echo "is the gRPC server running on port $PORT ?"
        '';
        layout = pkgs.writeShellScriptBin "layout" ''
          ${pkgs.zellij}/bin/zellij -l .zellij/servers.kdl
        '';
        # TODO: trunk cmd
        # `trunk serve --open --port 5010`
      in
      {
        packages = {
          inherit rpcWeb layout;
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
            # TODO: fix autocomplete error
            # rustc
            # cargo
            tailwindcss_4
            rustup
            bacon
            protobuf
            grpcui
            grpcurl
            sqlx-cli
            cargo-generate
            trunk
            cargo-leptos
            leptosfmt
          ];
        };
      }
    );
}
