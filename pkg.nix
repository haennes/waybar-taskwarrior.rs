{ rust-bin, lib, makeRustPlatform, ... }:

let
  rustPlatform = makeRustPlatform {
    cargo = rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
    rustc = rust-bin.selectLatestNightlyWith (toolchain: toolchain.default);
  };

in
rustPlatform.buildRustPackage {
  pname = "waybar-taskwarrior";
  version = "1.0";

  src = ./.;

  cargoHash = "sha256-C8XIJtS10w4tfbK1weKxy8XfXTKfnjZMYQaXaVtk4qU=";

  meta = with lib; {
    description = "A program to export taskwarrior to waybar";
    license = licenses.gpl2;
    platforms = platforms.all;
  };
}
