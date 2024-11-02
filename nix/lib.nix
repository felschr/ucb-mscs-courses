{ lib, flake-parts-lib, ... }:
{
  options.perSystem = flake-parts-lib.mkPerSystemOption (_: {
    options.umc.lib = lib.mkOption {
      type = lib.types.anything;
      default = { };
    };

    config.umc.lib = rec {
      mapKeys =
        f:
        lib.mapAttrs' (
          k: v: {
            name = f k;
            value = v;
          }
        );
      mkFilter =
        regexp: path: _type:
        builtins.match regexp path != null;
      combineFilters =
        filters: path: type:
        builtins.any (filter: filter path type) filters;
      cleanSource =
        filters: src:
        lib.cleanSourceWith {
          src = lib.cleanSource src;
          filter = combineFilters filters;
        };
      filterProtoSources = mkFilter ".*/proto(/(.*.proto)?)?$";
    };
  });
}
