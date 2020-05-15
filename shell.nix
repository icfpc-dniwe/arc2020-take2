{ pkgs ? import <nixpkgs> {} }:

(pkgs.callPackage ./. { }).overrideAttrs (self: {
  nativeBuildInputs = self.nativeBuildInputs or [] ++ [ pkgs.rustracer ];
  RUST_SRC_PATH = pkgs.rustPlatform.rustcSrc;
})
