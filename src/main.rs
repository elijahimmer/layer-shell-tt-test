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
        let label = Label::new(Some("Does the Tooltip work?"));

        let window = ApplicationWindow::builder()
            .application(app)
            .tooltip_text("test")
            .child(&label)
            .build();

        window.init_layer_shell();

        #[cfg(feature = "gtk3")]
        window.show_all();

        window.present();
    });

    application.run();
}
