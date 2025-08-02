{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    libiconv
  ];

  LIBRARY_PATH = "${pkgs.libiconv}/lib";
  LD_LIBRARY_PATH = "${pkgs.libiconv}/lib";
}
