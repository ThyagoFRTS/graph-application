pub mod frontend;
pub mod models;
pub mod utils;
use gtk::prelude::*;
use gtk::{glib, Application};

use crate::frontend::interface::build_ui;



const APP_ID: &str = "git.ThyagoFRTS.GraphAPP";

fn remove_letters(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if c.is_numeric() || c == ',' {
            result.push(c);
        }
    }
    result
}

fn main() -> glib::ExitCode {
//fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()

}
