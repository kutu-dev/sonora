#This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.
{
  # TODO: Add description
  description = "";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    allSystems = [
      "x86_64-linux"
      "aarch64-darwin"
    ];

    forAllSystems = fn:
      nixpkgs.lib.genAttrs allSystems
      (system: fn {pkgs = import nixpkgs {inherit system;};});
  in {
    formatter = forAllSystems ({pkgs}: pkgs.alejandra);

    devShells = forAllSystems ({pkgs}: {
      default = pkgs.mkShell {
        name = "datafall";
        nativeBuildInputs = with pkgs; [
          rustup
          addlicense
          just
          ripgrep

          alsa-lib.dev 
          pkg-config
        ];
      };
    });
  };
}
