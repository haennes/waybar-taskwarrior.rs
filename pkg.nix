{
  rustPlatform,
  lib,
  ...
}:

rustPlatform.buildRustPackage {
  pname = "waybar-taskwarrior";
  version = "1.0";

  src = ./.;

  cargoHash = "sha256-xyedScPgEjV6wNr78p07xEmdkRHNu25YSducXEHpkB8=";

  meta = with lib; {
    description = "A program to export taskwarrior to waybar";
    license = licenses.gpl2;
    platforms = platforms.all;
  };
}
