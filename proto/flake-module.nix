_: {
  perSystem =
    {
      self',
      pkgs,
      lib,
      ...
    }:
    {
      packages = {
        protobuf = pkgs.protobuf_27;
        buf-format = pkgs.writeShellApplication {
          name = "buf-format";
          runtimeInputs = with pkgs; [ buf ];
          text = ''
            for f in "$@"; do
              buf format -w "$f" & pids+=($!)
            done
            wait "''${pids[@]}"
          '';
        };
        buf-lint = pkgs.writeShellApplication {
          name = "buf-lint";
          runtimeInputs = with pkgs; [ buf ];
          text = ''
            if [ $# -eq 0 ]; then
              buf lint
            else
              for f in "$@"; do
                buf lint "$f" &
              done
              wait
            fi
          '';
        };
        buf-generate = pkgs.writeShellApplication {
          name = "buf-generate";
          runtimeInputs = with pkgs; [ buf ];
          text = ''
            shopt -s extglob

            rm -rf ./proto/gen/*/src/!(index.ts)
            buf generate
          '';
        };
      };

      devenv.shells.default = {
        scripts = {
          buf-format.exec = ''${lib.getExe self'.packages.buf-format} "$@"'';
          buf-lint.exec = ''${lib.getExe self'.packages.buf-lint} "$@"'';
          buf-generate.exec = ''${lib.getExe self'.packages.buf-generate} "$@"'';
        };
      };

      devenv.shells.default = {
        env.PROTOBUF_PROTOC = "${self'.packages.protobuf}/bin/protoc";

        packages =
          [ self'.packages.protobuf ]
          ++ (with pkgs; [
            grpc
            buf
            protoc-gen-prost
            protoc-gen-prost-crate
            protoc-gen-tonic

            buf-language-server
          ]);
      };
    };
}
