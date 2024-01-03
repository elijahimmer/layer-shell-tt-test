# GTK4 Layer Shell Tooltip bug

run `make gtk3` for testing the gtk3 version, 
and `make gtk4` for the gtk4 version.

In the GTK4 version, the moment the tooltip is shown,
the app has a segmentation fault and instantly stops.

I still need to look at the coredump to see if I can get
anything from it.
