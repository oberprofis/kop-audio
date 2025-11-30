{
  lib,
  fetchFromGitHub,
  pkg-config,
  libopus,
  libpulseaudio,
  rustPlatform,
}:

rustPlatform.buildRustPackage (finalAttrs: {
  pname = "kop-audio";
  version = "0.0.1";

  src = ./.;
  buildInputs = [
    libopus
    libpulseaudio
  ];

  nativeBuildInputs = [
    pkg-config
  ];

  cargoHash = "sha256-S0vjZFqzCzY+dy/ds8ns0Qt6utE9kYPq19Lh5t02Lek=";

  meta = {
    description = "A voice chat application written in Rust";
    maintainers = [ ];
  };
})
