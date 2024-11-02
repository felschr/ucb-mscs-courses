_: {
  perSystem =
    { config, ... }:
    let
      pname = "ucb-mscs-courses-importer";

      outputs = config.umc.rust.mkServiceOutputs pname {
        inherit pname;
        cargoExtraArgs = "-p ${pname}";
        nativeBuildInputs = [ ];
        buildInputs = [ ];
      };
    in
    {
      inherit (outputs) packages apps;
    };
}
