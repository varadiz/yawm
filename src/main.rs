pub mod wmctrl;

use gtk::prelude::*;
use gtk::{glib, Application};

const APP_ID: &str = "org.zorulab.Yawm";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
    glib::ExitCode::SUCCESS
}

// TODO: start running in the background
// TODO: add a tray icon
// TODO: listen for a key combination
// TODO: move back to backgroung

fn build_ui(app: &Application) {
    let input = gtk::Entry::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Yawm")
        .default_width(350)
        .build();

    window.set_child(Some(&input));
    input.connect_activate(|input| {
        let text = input.text();
        wmctrl::activate(text.as_str());
        input.set_text("");
    });
    window.present();
}
