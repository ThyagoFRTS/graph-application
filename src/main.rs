pub mod frontend;
pub mod models;
pub mod utils;
pub mod cmd;
use std::io;
use gtk::prelude::*;
use gtk::{glib, Application};
use models::graph::Representation;

use crate::frontend::interface::build_ui;

use std::process::Command;


const APP_ID: &str = "git.ThyagoFRTS.GraphAPP";

//bin\dot -Tsvg graphviz.dot -o output.svg
//fn main() -> glib::ExitCode {
fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run();

}
