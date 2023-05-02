{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self
    , utils
    , naersk
    , nixpkgs
    ,
    }:
    utils.lib.eachDefaultSystem
      (system:
      let
        pkgs = (import nixpkgs) { inherit system; };

        naersk' = pkgs.callPackage naersk { };
      in
      {
        # For `nix build` & `nix run`:
        packages = rec {
          kradalbyBin = naersk'.buildPackage {
            src = ./.;
            nativeBuildInputs = with pkgs; [ pkg-config openssl ];
          };

          kradalbyDhall = pkgs.stdenv.mkDerivation {
            pname = "kradalby-dhall";
            inherit (kradalbyBin) version;
            src = ./dhall;
            nativeBuildInputs = with pkgs; [ dhall ];

            phases = "installPhase";

            installPhase = ''
              mkdir -p $out/dhall
              dhall resolve --file $src/salaries.dhall >> $out/dhall/salaries.dhall
            '';
          };

          kradalbyMarkdown = pkgs.stdenv.mkDerivation {
            pname = "kradalby-markdown";
            inherit (kradalbyBin) version;
            src = ./md;

            phases = "installPhase";

            installPhase = ''
              mkdir -p $out/md
              cp -r $src/* $out/md/.
            '';
          };

          kradalbyOldMarkdown = pkgs.stdenv.mkDerivation {
            pname = "kradalby-old-markdown";
            inherit (kradalbyBin) version;
            src = ./old_md;

            phases = "installPhase";

            installPhase = ''
              mkdir -p $out/old_md
              cp -r $src/* $out/old_md/.
            '';
          };

          kradalby = pkgs.symlinkJoin {
            name = "kradalby-${kradalbyBin.version}";
            paths = [
              kradalbyBin
              kradalbyDhall
              kradalbyMarkdown
              kradalbyOldMarkdown
            ];
          };

          default = kradalby;
        };

        # For `nix develop`:
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs;
            [
              rustc
              cargo
              cargo-watch
              rust-analyzer
              rustfmt
              python3
              pkg-config
              openssl
              dhall
            ]
            ++ pkgs.lib.optionals
              pkgs.stdenv.isDarwin [
              darwin.apple_sdk.frameworks.Security

              # Used for html2maud-bin
              darwin.apple_sdk.frameworks.Carbon
              darwin.apple_sdk.frameworks.AppKit
              darwin.apple_sdk.frameworks.WebKit
            ];
        };
      })
    // {
      nixosModules.default =
        { pkgs
        , lib
        , config
        , ...
        }:
        let
          cfg = config.services.kradalby;
        in
        {
          options = with lib; {
            services.kradalby = {
              enable = mkEnableOption "Enable kradalby site";

              package = mkOption {
                type = types.package;
                description = ''
                  kradalby package to use
                '';
                default = pkgs.kradalby;
              };

              port = mkOption {
                type = types.port;
                default = 30010;
              };
            };
          };
          config = lib.mkIf cfg.enable {
            systemd.services.kradalby = {
              enable = true;
              script = ''
                ${cfg.package}/bin/kradalby
              '';
              wantedBy = [ "multi-user.target" ];
              after = [ "network-online.target" ];
              serviceConfig = {
                DynamicUser = true;
                Restart = "always";
                RestartSec = "15";
                WorkingDirectory = cfg.package;
              };
              environment = {
                PORT = toString cfg.port;
              };
            };
          };
        };
    };
}
