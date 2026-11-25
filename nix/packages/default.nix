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
          version = "0.3.1";
          src = self;
          cargoHash = "sha256-EjfeQuOWKs5/KeDnARkgCnveJxdFu6h6/Sq69aueR+U=";
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
