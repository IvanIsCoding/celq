{ pkgs ? import <nixpkgs> {} }:

pkgs.callPackage ./nix/celq.nix { }