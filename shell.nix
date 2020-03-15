{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
    buildInputs = [
      pkgs.cargo
      pkgs.rustc_1_41
    ];
}
