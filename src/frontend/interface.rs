use gtk::glib::clone;
use gtk::prelude::*;
use gtk::{
    glib,
    Application,
    ApplicationWindow,
    FileChooserDialog, 
    FileChooserAction,
    Box,
    Align,
    FileFilter,
    Orientation, 
    ScrolledWindow,
    PolicyType,
    ResponseType
};
use crate::controllers::handlers::*;
use crate::models::graph::Representation;
use crate::frontend::components;

pub fn build_ui(app: &Application) {
    let main_window = ApplicationWindow::new(app);
    main_window.set_title(Some("Graph App"));
    main_window.set_default_size(500, 500);

    let line1 = components::build_separator();
    let line2 = components::build_separator();
    let line3 = components::build_separator();
    let line4 = components::build_separator();
    let vline1 = components::build_vseparator();

    let path_file_label = components::build_label("Nenhum arquivo carregado");

    let load_file_button = components::build_button("Carregar arquivo");

    let verify_adjc_button = components::build_button("Verificar adjacência");
    verify_adjc_button.set_sensitive(false);

    let verify_degree_button = components::build_button("Verificar grau");
    verify_degree_button.set_sensitive(false);

    let tree_detect_button = components::build_button("Detectar árvore");
    tree_detect_button.set_sensitive(false);

    let edges_button = components::build_button("Visitar arestas");
    edges_button.set_sensitive(false);

    let export_png_button = components::build_button("Exportar svg");
    export_png_button.set_sensitive(false);

    let neighbors_button = components::build_button("Procurar Vizinhos");
    neighbors_button.set_sensitive(false);

    let vertex_list1_dropdown = components::build_dropdown();
    vertex_list1_dropdown.set_sensitive(false);

    let vertex_list2_dropdown = components::build_dropdown();
    vertex_list2_dropdown.set_sensitive(false);

    let vertex_list3_dropdown = components::build_dropdown();
    vertex_list3_dropdown.set_sensitive(false);

    let vertex_list4_dropdown = components::build_dropdown();
    vertex_list4_dropdown.set_sensitive(false);

    let output_label = components::build_clean_label(" ");
    output_label.set_wrap(true);
    output_label.set_wrap_mode(gtk::pango::WrapMode::Word);
     
    let scolled_window = components::build_scrollwindow(" ");
    scolled_window.set_child(Some(&output_label));

    let output_frame = components::build_frame();

    output_frame.set_child(Some(&scolled_window));
    output_frame.set_vexpand(true);

    let buttons_layout = Box::new(Orientation::Horizontal, 0);
    buttons_layout.set_halign(Align::Center);
    buttons_layout.append(&load_file_button);
    buttons_layout.append(&export_png_button);

    let neighborhood_layout = Box::new(Orientation::Horizontal,0);
    neighborhood_layout.set_halign(Align::Center);
    neighborhood_layout.append(&vertex_list3_dropdown);
    neighborhood_layout.append(&verify_degree_button);
    neighborhood_layout.append(&vline1);
    neighborhood_layout.append(&vertex_list4_dropdown);
    neighborhood_layout.append(&neighbors_button);

    let adjacency_layout = Box::new(Orientation::Horizontal, 0);
    adjacency_layout.set_halign(Align::Center);
    adjacency_layout.append(&vertex_list1_dropdown);
    adjacency_layout.append(&vertex_list2_dropdown);
    adjacency_layout.append(&verify_adjc_button);
    
    let tree_egdes_layout = Box::new(Orientation::Horizontal, 0);
    tree_egdes_layout.set_halign(Align::Center);
    tree_egdes_layout.append(&tree_detect_button);
    tree_egdes_layout.append(&edges_button);

    let main_layout = Box::new(Orientation::Vertical, 0);
    main_layout.append(&buttons_layout);
    main_layout.append(&path_file_label);
    main_layout.append(&line1);
    main_layout.append(&neighborhood_layout);
    main_layout.append(&line2);
    main_layout.append(&tree_egdes_layout);
    main_layout.append(&line3);
    main_layout.append(&adjacency_layout);
    main_layout.append(&line4);
    //main_layout.append(&scolled_window);
    main_layout.append(&output_frame);

    verify_adjc_button.connect_clicked( clone!( 
        @weak main_window,
        @weak path_file_label,
        @weak vertex_list1_dropdown,
        @weak vertex_list2_dropdown
         => move |_button| {
            handle_vertex_adjacency(&main_window, &path_file_label, &vertex_list1_dropdown, &vertex_list2_dropdown);
    }));

    verify_degree_button.connect_clicked( clone!( 
        @weak main_window,
        @weak path_file_label,
        @weak vertex_list3_dropdown
         => move |_button| {
            handle_vertex_degree(&main_window, &path_file_label, &vertex_list3_dropdown);
    }));

    neighbors_button.connect_clicked( clone!( 
        @weak main_window,
        @weak path_file_label,
        @weak vertex_list4_dropdown
         => move |_button| {
            handle_vertex_neighbors(&main_window, &path_file_label, &vertex_list4_dropdown);
    }));

    export_png_button.connect_clicked(clone!(
        @weak main_window,
        @weak path_file_label
        => move |_| {
            handle_export_svg(&main_window, &path_file_label);
        }
    ));

    tree_detect_button.connect_clicked( clone!( 
        @weak path_file_label,
        @weak output_label
         => move |_button| {
            handle_tree_detection(&path_file_label, &output_label);
    }));

    edges_button.connect_clicked(clone!(
        @weak path_file_label,
        @weak output_label
        => move |_button| {
            handle_visit_edges(&path_file_label, &output_label);
        }
    ));

    load_file_button.connect_clicked(clone!(
        @weak main_window
          => move |_button| {
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
            @weak verify_adjc_button,
            @weak neighbors_button,
            @weak verify_degree_button,
            @weak edges_button,
            @weak export_png_button,
            @weak vertex_list1_dropdown,
            @weak vertex_list2_dropdown,
            @weak vertex_list3_dropdown,
            @weak vertex_list4_dropdown,
            @weak tree_detect_button => move |file_explorer, response| {
            if response == ResponseType::Ok {
                if let Some(file) = file_explorer.file() {
                    if let Some(path) = file.path() {
                        let mut g = Representation::new();
                        let path = path.as_os_str().to_str().unwrap();
                        g.load_from_file(path);

                        path_file_label.set_text(path);

                        setup_ui_components(
                            &g, 
                            &verify_adjc_button,
                            &neighbors_button,
                            &verify_degree_button,
                            &edges_button,
                            &export_png_button,
                            &tree_detect_button,
                            &vertex_list1_dropdown,
                            &vertex_list2_dropdown,
                            &vertex_list3_dropdown,
                            &vertex_list4_dropdown
                        );
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