
{ pkgs ? import <nixpkgs> {} }:
with pkgs;
mkShell {

  nativeBuildInputs = [
    rustc 
    cargo 
    rustfmt
  ];
}

