#pkg-config pango graphene gtk4
{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  # nativeBuildInputs is usually what you want -- tools you need to run
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
  ];

  buildInputs = with pkgs; [
    pkg-config
    atk
    pango
    graphene
    glib
    gtk3
    gtk-layer-shell
    gtk4
    gtk4-layer-shell
  ];
}
