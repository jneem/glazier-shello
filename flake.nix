{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
    unstable.url = "nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, unstable, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        unstablePkgs = import unstable {
          localSystem = { inherit system; };
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            clang
            cmake
            libxkbcommon
            xorg.libxcb
            pkg-config
            fontconfig
            wayland
            unstablePkgs.rust-analyzer
            rust-bin.stable.latest.default
            vulkan-headers
            vulkan-loader
            vulkan-tools
          ];
          
          shellHook = ''
            alias ls=exa
            alias find=fd
          '';
          
          LD_LIBRARY_PATH="${vulkan-loader}/lib:${wayland}/lib:${libxkbcommon}/lib";
          LIBCLANG_PATH = lib.makeLibraryPath [ libclang ];
        };
      }
    );
}