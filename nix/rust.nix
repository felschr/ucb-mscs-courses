{
  inputs,
  flake-parts-lib,
  lib,
  ...
}:
{
  options.perSystem = flake-parts-lib.mkPerSystemOption (_: {
    options.umc.rust = lib.mkOption {
      type = lib.types.anything;
      default = { };
    };
  });

  config.perSystem =
    {
      self',
      config,
      pkgs,
      lib,
      ...
    }:
    let
      rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;

      craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rust-toolchain;

      src = config.umc.lib.cleanSource [
        craneLib.filterCargoSources
        config.umc.lib.filterProtoSources
      ] ./..;

      crateName = craneLib.crateNameFromCargoToml { cargoToml = ../Cargo.toml; };

      commonArgs = {
        inherit src;
        pname = "workspace";
        inherit (crateName) version;
        nativeBuildInputs = [ ];
        buildInputs =
          with pkgs;
          [ protobuf ]
          ++ lib.optionals stdenv.isDarwin (
            with pkgs.darwin.apple_sdk.frameworks;
            [
              pkgs.libiconv
              Security
              SystemConfiguration
            ]
          );
        CARGO_BUILD_TARGET = "x86_64-unknown-linux-musl";
        CARGO_BUILD_RUSTFLAGS = "-C target-feature=+crt-static";
      };
      commonDebugArgs.CARGO_PROFILE = "dev";

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      cargoArtifactsDev = craneLib.buildDepsOnly (commonArgs // commonDebugArgs);

      buildPackage =
        args@{
          nativeBuildInputs ? [ ],
          buildInputs ? [ ],
          ...
        }:
        craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            PROTOC = "${pkgs.protobuf}/bin/protoc";
            PROTOC_INCLUDE = "${pkgs.protobuf}/include";
          }
          // args
          // {
            nativeBuildInputs = commonArgs.nativeBuildInputs ++ nativeBuildInputs;
            buildInputs = commonArgs.buildInputs ++ buildInputs;
          }
        );
      buildPackageDev =
        args: buildPackage (commonDebugArgs // { cargoArtifacts = cargoArtifactsDev; } // args);

      cargoWorkspace = buildPackage { };
      cargoWorkspaceDev = buildPackageDev { };

      cargoWorkspaceTest = craneLib.cargoNextest (commonArgs // { inherit cargoArtifacts; });
      cargoWorkspaceTestCI = craneLib.cargoNextest (
        commonArgs
        // {
          inherit cargoArtifacts;
          cargoNextestExtraArgs = "--profile ci || true";
          postInstall = ''
            cp ./target/nextest/ci/junit.xml $out/junit.xml
          '';
        }
      );

      mkServiceOutputs =
        pname: packageArgs:
        let
          mapOutputs = pname: config.umc.lib.mapKeys (k: if k == "default" then pname else "${pname}-${k}");

          packages = {
            default = buildPackage packageArgs;
            dev = buildPackageDev packageArgs;
          };

          apps = {
            default = inputs.flake-utils.lib.mkApp { drv = packages.default; };
            dev = inputs.flake-utils.lib.mkApp { drv = packages.dev; };
          };
        in
        {
          packages = mapOutputs pname packages;
          apps = mapOutputs pname apps;
        };
    in
    {
      umc.rust = {
        inherit
          craneLib
          commonArgs
          commonDebugArgs
          buildPackage
          buildPackageDev
          mkServiceOutputs
          ;
      };

      packages = {
        inherit
          rust-toolchain
          cargoArtifacts
          cargoArtifactsDev
          cargoWorkspace
          cargoWorkspaceDev
          cargoWorkspaceTest
          cargoWorkspaceTestCI
          ;
      };
      checks = {
        inherit (self'.packages) cargoWorkspace cargoWorkspaceTest;
        cargo-fmt = craneLib.cargoFmt (commonArgs // { inherit src; });
        cargo-doc = craneLib.cargoDoc (commonArgs // { inherit cargoArtifacts; });
        clippy = craneLib.cargoClippy (
          commonArgs
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          }
        );
      };

      devenv.shells.default = {
        languages.rust.enable = true;
        languages.rust.toolchain = {
          cargo = rust-toolchain;
          clippy = rust-toolchain;
          rust-analyzer = rust-toolchain;
          rustc = rust-toolchain;
          rustfmt = rust-toolchain;
        };

        packages = commonArgs.nativeBuildInputs ++ commonArgs.buildInputs;
      };
    };
}
