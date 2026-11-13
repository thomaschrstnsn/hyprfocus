{
  description = "hyprfocus: focus Hyprland windows by class/title, or spawn them";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
    in
    {
      packages = forAllSystems (pkgs: rec {
        hyprfocus = pkgs.rustPlatform.buildRustPackage {
          pname = cargoToml.package.name;
          version = cargoToml.package.version;

          src = pkgs.lib.cleanSource ./.;
          cargoLock.lockFile = ./Cargo.lock;

          meta = with pkgs.lib; {
            description = "Hyprland focus helper";
            homepage = "https://github.com/thomaschrstnsn/hyprfocus";
            license = licenses.mit;
            platforms = platforms.linux;
            mainProgram = "hyprfocus";
          };
        };
        default = hyprfocus;
      });

      overlays.default = final: _prev: {
        hyprfocus = self.packages.${final.stdenv.hostPlatform.system}.hyprfocus;
      };

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          inputsFrom = [ self.packages.${pkgs.stdenv.hostPlatform.system}.hyprfocus ];
          packages = with pkgs; [ rust-analyzer clippy rustfmt ];
        };
      });

      formatter = forAllSystems (pkgs: pkgs.nixpkgs-fmt);
    };
}
