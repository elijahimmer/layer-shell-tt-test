#[cfg(all(feature = "gtk3", feature = "gtk4"))]
compile_error!("You can only enable gtk3 or gtk4, not both");

#[cfg(feature = "gtk4")]
use gtk4 as gtk;

#[cfg(feature = "gtk4")]
use gtk4_layer_shell as gtk_layer_shell;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};
use gtk_layer_shell::LayerShell;

fn main() {
    let application = Application::builder()
        .application_id("segfault.please")
        .build();

    application.connect_activate(|app| {
        let label = Label::builder()
            .label("Does the Tooltip work?")
            .has_tooltip(true)
            .build();

        let window = ApplicationWindow::builder()
            .application(app)
            .child(&label)
            .build();

        window.init_layer_shell();

        label.connect_query_tooltip(|_wgt, _x, _y, _cursor_inside, tt| {
            tt.set_text(Some("Can you see it?"));

            println!("Received Query");

            if cfg!(feature = "no_display") {
                false
            } else {
                true
            }
        });

        #[cfg(feature = "gtk3")]
        window.show_all();

        window.present();
    });

    application.run();
}
