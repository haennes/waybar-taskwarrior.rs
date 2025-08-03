{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

 rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };  };

  outputs =
    { self, nixpkgs, rust-overlay }@inputs:
    let
      pkgs = import nixpkgs { system = "x86_64-linux"; overlays = [(import rust-overlay)];};
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          rust-bin.selectLatestNightlyWith (toolchain: toolchain.default)
          cargo
          openssl
          pkg-config
          rust-analyzer
        ];

        # PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
      };

      packages.x86_64-linux.default = pkgs.callPackage ./pkg.nix {};
    };
}
