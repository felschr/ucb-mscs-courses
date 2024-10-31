{ inputs, ... }:
{
  imports = with inputs; [ treefmt-nix.flakeModule ];

  perSystem =
    {
      self',
      config,
      pkgs,
      lib,
      ...
    }:
    {
      treefmt = {
        flakeFormatter = true;
        flakeCheck = true;

        projectRootFile = "flake.nix";

        programs = {
          nixfmt.enable = true;
          rustfmt.enable = true;
          # rustfmt.package = self'.packages.rust-toolchain;
          formatjson5.enable = true;
          shfmt.enable = true;
          shellcheck.enable = true;
          mdformat.enable = true;
        };

        # ignore generated code
        settings.global.excludes = [ "proto/gen/**" ];
        settings.formatter = {
          dart-format = {
            command = "${pkgs.dart}/bin/dart";
            options = [ "format" ];
            includes = [ "*.dart" ];
          };
          buf-format = {
            command = lib.getExe self'.packages.buf-format;
            includes = [ "*.proto" ];
          };
          buf-lint = {
            command = lib.getExe self'.packages.buf-lint;
            includes = [ "*.proto" ];
          };
        };
      };

      devenv.shells.default = {
        packages = [ config.treefmt.build.wrapper ];
      };

      formatter = config.treefmt.build.wrapper;
    };
}
