# GTK4 Layer Shell Tooltip bug

run `make gtk3` for testing the gtk3 version, 
and `make gtk4` for the gtk4 version.

This seems to only crash when the tooltip is actually displayed:
see `make query-no-display` which is the same as `make query4` but
`WidgetExt::connect_query_tooltip()`'s callback returns false, so it is not displayed.

There is also `make set3` and `make set4` that use the `WidgetExt::set_tooltip_text()`
instead of using the builder. The issue is still there.

I'm pretty sure it is just displaying the tooltip that is the issue, I don't know enough
about gtk or gtk-layer-shell to do more then speculate.

In the GTK4 version, the moment the tooltip should be shown,
the app has a segmentation fault and instantly stops.

I still need to look at the coredump to see if I can get
anything from it.
