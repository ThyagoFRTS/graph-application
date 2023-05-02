pub mod frontend;
pub mod models;
use gtk::prelude::*;
use gtk::{glib, Application};

use crate::frontend::interface::build_ui;



const APP_ID: &str = "git.ThyagoFRTS.GraphAPP";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()
}
