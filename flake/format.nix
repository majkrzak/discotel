{ inputs, ... }:
{
  imports = [
    inputs.treefmt-nix.flakeModule
  ];
  perSystem =
    { ... }:
    {
      treefmt.programs.nixfmt.enable = true;
      treefmt.programs.rustfmt.enable = true;
      treefmt.programs.mdformat.enable = true;
      treefmt.programs.taplo.enable = true;
    };
}
