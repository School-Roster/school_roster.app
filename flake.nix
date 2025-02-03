{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.11";
    # nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable"; # Pin to latest
    # nixpkgs-stable.url = "github:NixOS/nixpkgs/nixos-23.11"; # Queremos mantenernos actualizados, pero habra casos que necesitemos utilizar versiones estables
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        libraries = with pkgs; [
          webkitgtk
          gtk3
          cairo
          gdk-pixbuf
          glib
          dbus
          openssl
          librsvg
        ];
        packages = with pkgs; [
          rustc
          cargo
          curl
          wget
          pkg-config
          dbus
          openssl
          glib
          gtk3
          libsoup
          webkitgtk
          librsvg
          sqlx-cli
          gcc
          libiconv
          openssl.dev
        ];
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = packages;
          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
            export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
            export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
          '';
        };
      }
    );
}
