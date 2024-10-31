_: {
  perSystem =
    { self', pkgs, ... }:
    {
      packages = {
        protobuf = pkgs.protobuf_27;
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
