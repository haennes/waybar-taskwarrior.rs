{ rustPlatform, lib, ... }:

rustPlatform.buildRustPackage {
  pname = "waybar-taskwarrior";
  version = "1.0";

  src = ./.;

  cargoHash = "sha256-tBrYZooXYLWMBCMMat/aSqV6fZOcF74SxgBNtO7Vpdg=";

  useFetchCargoVendor = true;

  meta = with lib; {
    description = "A program to export taskwarrior to waybar";
    license = licenses.gpl2;
    platforms = platforms.all;
  };
}
