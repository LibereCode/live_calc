{
  self,
  inputs,
  ...
}:
{
  perSystem =
    {
      pkgs,
      # self',
      ...
    }:
    let
      naersk' = pkgs.callPackage inputs.naersk { };
    in
    {
      packages = {
        naersk = naersk'.buildPackage {
          src = self;

          # #: See [source -> USAGE for more](https://github.com/nix-community/naersk#usage)
          # buildInputs = with pkgs; [ cmake ];
          # SOME_ENV_VAR = "yes";

          #: [Examples](https://github.com/nix-community/naersk/blob/master/examples)
        };

        # default = self'.packages.naersk; # NOTE See ./crane.nix
      };
    };
}
