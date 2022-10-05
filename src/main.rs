mod ui;

use adw::{
    prelude::*,gtk::gio
};
use ui::main_window::MainWindow;

const APP_ID: &str = "org.bil4x4.gnome-phone-companion";

fn main() {
    gio::resources_register_include!("gnome-phone-companion.gresource")
        .expect("Failed to register resources.");

    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &adw::Application) {
    let window = MainWindow::new(app);
    window.show();
}