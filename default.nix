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

  cargoHash = "sha256-NYzy58PR7SMY1nlAWiESraPod2Wam1KVtgr16q9jm60=";

  meta = {
    description = "A voice chat application written in Rust";
    maintainers = [ ];
  };
})
