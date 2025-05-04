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
      in
      {
        packages = {
          inherit rpcWeb;
        };

        apps.rpcWeb = {
          type = "app";
          program = "${self.packages.${system}.rpcWeb}/bin/rpcWeb";
        };

        # nix develop
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            # TODO: fix autocomplete error
            # rustc
            # cargo
            rustup
            bacon
            protobuf
            grpcui
            grpcurl
          ];
        };
      }
    );
}
