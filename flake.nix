{
  # I apologize in advance for any mistakes, I am new to Nix flakes
  description = "celq - A Common Expression Language (CEL) CLI Tool";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      # I have no clue what I am doing here but I assume this covers 99% of users
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      
      # 
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
    in
    {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          # Compiles from source with Cargo and installs celq. That part I understood!
          default = pkgs.callPackage ./nix/celq.nix { };
        });
    };
}