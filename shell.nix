{ nixpkgs ? import <nixpkgs> {} }:

nixpkgs.callPackage ./. {
}
