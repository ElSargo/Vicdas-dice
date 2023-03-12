{
  description = "Dev shell the project";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs = nixpkgs.legacyPackages.${system}; in
        {
          nixpkgs.overlays = [ fenix.overlays.complete ];
          devShells.default = pkgs.mkShell {
            buildInputs =  [ fenix.packages.x86_64-linux.complete.toolchain pkgs.lldb_9 pkgs.sccache pkgs.mold pkgs.clang pkgs.fish];
            shellHook = /*bash*/ "[ $0 = 'bash' ] && exec fish";
          };
        }
      );
}

