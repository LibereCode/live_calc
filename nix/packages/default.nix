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
          version = "0.0.1";
          src = "${self}";
          cargoHash = "sha256-iUIfILIRCVB87SjvxLTikptBm2A0f25uitEKfZZ+C7k=";
          meta = with lib; {
            description = "This is my preferred rust-development setup.";
            license = licenses.unlicense;
            maintainers = [ ];
          };
        });

        #NOTE: Prefer ../naersk.nix or (better) ../crane.nix

        ## $ nix build .
        default = config.packages.my-crate; # Crane
      };
    };
}
