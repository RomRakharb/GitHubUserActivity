{
  description = "Simple rust dev environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let
    pkgs = nixpkgs.legacyPackages.x86_64-linux;
  in
  {

    devShells.x86_64-linux.default = pkgs.mkShell {

      # shellHook = ''
      #   nu
      #   exit
      # '';

      packages = with pkgs; [
        rustc
        cargo
        clippy
        rustfmt
        rust-analyzer

        helix
        bacon
      ];

    };

  };
}
