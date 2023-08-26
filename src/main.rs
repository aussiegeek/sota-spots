mod spot_data;
mod spot_object;
mod spot_row;
mod window;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

fn main() -> glib::ExitCode {
    gio::resources_register_include!("sota_spots.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder()
        .application_id("com.vk3xe.sota_spots")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
