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
/* 
fn build_ui(app: &Application) {
    //let graph_instance = models::graph::Graph::new();
    let mut enable_btn = false;

    let main_window = ApplicationWindow::new(app);
    main_window.set_title(Some("Graph App"));
    main_window.set_default_size(500, 500);

    let horiozontal_line = Separator::builder()
        .orientation(Orientation::Horizontal)
        .height_request(2)
        .margin_start(12)
        .margin_end(12)
        .build();

    let path_file_label = Label::builder()
        .label("Nenhum arquivo carregado")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let load_file_button = Button::builder()
        .label("Carregar arquivo")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let tree_detect_button = Button::builder()
        .label("Detectar árvore")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .sensitive(enable_btn)
        .build();

    let buttons_layout = Box::new(Orientation::Horizontal, 0);
    buttons_layout.append(&load_file_button);
    buttons_layout.append(&tree_detect_button);

    let main_layout = Box::new(Orientation::Vertical, 0);
    main_layout.append(&buttons_layout);
    main_layout.append(&path_file_label);
    main_layout.append(&horiozontal_line);

    load_file_button.connect_clicked(clone!(
        @weak main_window, 
        @weak path_file_label,
        @weak tree_detect_button
          => move |button| {
        let file_explorer = FileChooserDialog::new(
            Some("Selecione o arquivo de texto"),
            Some(&main_window),
            FileChooserAction::Open,
            &[
                ("Cancelar", ResponseType::Cancel),
                ("Abrir", ResponseType::Ok)
            ]
        );

        let filter = FileFilter::new();
        filter.add_pattern("*.txt");
        file_explorer.add_filter(&filter);

        file_explorer.connect_response(clone!(
            @weak path_file_label,
            @weak tree_detect_button => move |file_explorer, response| {
            if response == ResponseType::Ok {
                if let Some(file) = file_explorer.file() {
                    if let Some(path) = file.path() {
                        println!("Caminho do arquivo selecionado: {:?}", path.as_os_str());
                        let mut g = models::graph::Graph::new();
                        path_file_label.set_text(path.as_os_str().to_str().unwrap());
                        g.load_from_file(path.as_os_str().to_str().unwrap());
                        g.print_graph();
                        tree_detect_button.set_sensitive(true);
                    }
                }
            }
            file_explorer.destroy();
        }));
        file_explorer.show();
    }));

    main_window.set_child(Some(&main_layout));
    
    main_window.present();
}
*/