{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    pkg-config
  ];

  nativeBuildInputs = with pkgs; [
    libpulseaudio
  ];

  shellHook =
    let
      libraries = with pkgs; [

        libGL
        # X11 dependencies
        xorg.libX11
        xorg.libX11.dev
        xorg.libXcursor
        xorg.libXi
        xorg.libXinerama
        xorg.libXrandr
        libpulseaudio
      ];
    in
    ''
      export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
    '';
}
