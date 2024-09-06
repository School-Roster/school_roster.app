{
  lib,
  pkgs,
  stdenv,
  rustPlatform,
  installShellFiles,
  darwin,
  version ? "git"
}:

rustPlatform.buildRustPackage rec {
  pname = "school_shedule";
  inherit version;

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  buildInputs = with pkgs; [
    vulkan-loader

    # for linux wayland
    wayland
    wayland-protocols

    # for linux x11
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi

    libxkbcommon

    # tauri deps
    curl
    wget
    pkg-config
    dbus
    openssl_3
    glib
    gtk3
    libsoup
    webkitgtk
    librsvg
  ]
  ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.SystemConfiguration
    darwin.apple_sdk.frameworks.Foundation
  ];

  nativeBuildInputs = with pkgs; [
    # needed rust tools for building the project
    installShellFiles
    pkg-config
    gtk-layer-shell
    cmake
    makeWrapper

    webkitgtk
    gtk3
    cairo
    gdk-pixbuf
    glib
    dbus
    openssl_3
    librsvg
  ];

  postInstall = ''
    installShellCompletion --cmd school_schedule \
      --bash <($out/bin/school_shedule completions bash) \
      --zsh <($out/bin/school_shedule completions zsh) \
      --fish <($out/bin/school_shedule completions fish)
  '';

  patchPhase = ''
    sed -i 's/env!("CARGO_PKG_VERSION")/\"${version}\"/g' src/main.rs
  '';

  postFixup = ''
    wrapProgram $out/bin/tudus \
      --suffix LD_LIBRARY_PATH : ${pkgs.lib.makeLibraryPath buildInputs}
  '';

  meta = with lib;{
    description = "Proyecto de titulacion para crear y gestionar horarios escolares";
    homepage = "https://github.com/awtgerry/school_schedule";
    license = licenses.lgpl3Only;
    mainProgram = "school_schedule";
  };
}
