{
  rustPlatform,
  lib,
  ...
}:

rustPlatform.buildRustPackage {
  pname = "waybar-taskwarrior";
  version = "1.0";

  src = ./.;

  cargoHash = "sha256-EvXD9aLFyY/M7y2W0uTpkJlQIZuNg1YpB+SaHYlwpyE=";

  meta = with lib; {
    description = "A program to export taskwarrior to waybar";
    license = licenses.gpl2;
    platforms = platforms.all;
  };
}
