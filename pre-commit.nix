{ inputs, ... }:
{
  imports = with inputs; [ pre-commit-hooks.flakeModule ];

  perSystem =
    {
      self',
      pkgs,
      lib,
      ...
    }:
    {
      pre-commit = {
        check.enable = false;
        settings = {
          src = lib.cleanSource ./.;
          hooks = {
            convco.enable = true;
            treefmt.enable = true;
            buf-lint = {
              enable = true;
              name = "buf-lint";
              description = "Lint Protocol Buffers";
              pass_filenames = false;
              entry = lib.getExe (
                pkgs.writeShellApplication {
                  name = "buf-lint";
                  runtimeInputs = [ self'.packages.buf-lint ];
                  text = ''
                    buf-lint
                  '';
                }
              );
              files = "\\.proto$";
            };
            buf-generate = {
              enable = true;
              name = "buf-generate";
              description = "Generate Protobuf code";
              pass_filenames = false;
              entry = lib.getExe (
                pkgs.writeShellApplication {
                  name = "buf-generate";
                  runtimeInputs = [ self'.packages.buf-generate ];
                  text = ''
                    buf-generate
                  '';
                }
              );
              files = "^(flake\\.(nix|lock)|proto/.*\\.proto)$";
            };
          };
        };
      };
    };
}
