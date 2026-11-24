{
  self,
  inputs,
  ...
}:
{
  perSystem =
    {
      config,
      pkgs,
      lib,
      ...
    }:
    {
      packages = {
        ## NOTE Using default packageWrapping
        basic = pkgs.rustPlatform.buildRustPackage (finalAttrs: {
          pname = "live_calc";
          version = "0.3.0";
          src = self;
          cargoHash = "sha256-Lqrbc/+ulmWD+Zntw3ISdO0Eo2k2Z6VwWbPhmrP6Bes=";
          meta = with lib; {
            description = "A live tui calculater, written in rust. Uses ratatui for tui.";
            license = licenses.eupl12;
            maintainers = [ ];
          };
        });

        ## $ nix build .
        default = config.packages.my-crate; # Crane
      };
    };
}
