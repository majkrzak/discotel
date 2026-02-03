{ inputs, ... }:
{
  perSystem =
    { pkgs, self', ... }:
    let
      craneLib = inputs.crane.mkLib pkgs;
    in
    {
      packages.docker = pkgs.dockerTools.buildLayeredImage {
        name = "discolog";
        config.Cmd = [ self'.packages.default ];
      };
    };
}
