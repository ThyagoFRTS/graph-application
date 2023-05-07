pub mod frontend;
pub mod models;
pub mod utils;
pub mod graphy_search;
pub mod controllers;
pub mod cmd;
use gtk::prelude::*;
use gtk::Application;


use crate::frontend::interface::build_ui;


const APP_ID: &str = "git.ThyagoFRTS.GraphAPP";

//bin\dot -Tsvg graphviz.dot -o output.svg
//fn main() -> glib::ExitCode {
fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run();

}
