use gtk::glib::clone;
use gtk::{prelude::*, MessageDialog, DialogFlags};
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
    Frame,
    StringList,
    Expression, 
    ResponseType
};
use crate::cmd::use_graphviz;
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

    let path_file_label = components::build_label("Nenhum arquivo carregado");

    let load_file_button = components::build_button("Carregar arquivo");

    let verify_adjc_button = components::build_button("Verificar adjacência");
    verify_adjc_button.set_sensitive(false);

    let verify_degree_button = components::build_button("Verificar grau");
    verify_degree_button.set_sensitive(false);

    let tree_detect_button =components::build_button("Detectar árvore");
    tree_detect_button.set_sensitive(false);

    let export_png_button =components::build_button("Exportar svg");
    export_png_button.set_sensitive(false);

    let neighbors_button =components::build_button("Procurar Vizinhos");
    neighbors_button.set_sensitive(false);

    let vertex_list1_dropdown = components::build_dropdown();
    vertex_list1_dropdown.set_sensitive(false);

    let vertex_list2_dropdown = components::build_dropdown();
    vertex_list2_dropdown.set_sensitive(false);

    let vertex_list3_dropdown = components::build_dropdown();
    vertex_list3_dropdown.set_sensitive(false);

    let vertex_list4_dropdown = components::build_dropdown();
    vertex_list4_dropdown.set_sensitive(false);

    let output_frame = Frame::builder()
        .label("")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let buttons_layout = Box::new(Orientation::Horizontal, 0);
    buttons_layout.set_halign(Align::Center);
    buttons_layout.append(&load_file_button);
    buttons_layout.append(&export_png_button);
    buttons_layout.append(&tree_detect_button);

    let degree_layout = Box::new(Orientation::Horizontal,0);
    degree_layout.set_halign(Align::Center);
    degree_layout.append(&vertex_list3_dropdown);
    degree_layout.append(&verify_degree_button);

    let vertex_layout = Box::new(Orientation::Horizontal, 0);
    vertex_layout.set_halign(Align::Center);
    vertex_layout.append(&vertex_list1_dropdown);
    vertex_layout.append(&vertex_list2_dropdown);
    vertex_layout.append(&verify_adjc_button);
    
    let neighbors_layout = Box::new(Orientation::Horizontal, 0);
    neighbors_layout.set_halign(Align::Center);
    neighbors_layout.append(&vertex_list4_dropdown);
    neighbors_layout.append(&neighbors_button);

    let main_layout = Box::new(Orientation::Vertical, 0);
    main_layout.append(&buttons_layout);
    main_layout.append(&path_file_label);
    main_layout.append(&line1);
    main_layout.append(&degree_layout);
    main_layout.append(&line2);
    main_layout.append(&neighbors_layout);
    main_layout.append(&line3);
    main_layout.append(&vertex_layout);
    main_layout.append(&line4);
    main_layout.append(&output_frame);

    verify_adjc_button.connect_clicked( clone!( 
        @weak main_window,
        @weak path_file_label,
        @weak vertex_list1_dropdown,
        @weak vertex_list2_dropdown
         => move |_button| {
            let source = vertex_list1_dropdown.selected();
            let destiny = vertex_list2_dropdown.selected();

            let mut g = Representation::new();
            g.load_from_file(path_file_label.text().as_str());
            let result = g.is_adjacent(source.try_into().unwrap(), destiny.try_into().unwrap());

            let message = if result {"São adjacentes"} else {"Não são adjacentes"};

            let result_window = MessageDialog::new(
                Some(&main_window),
                DialogFlags::empty(),
                gtk::MessageType::Info,
                gtk::ButtonsType::Ok,
                message
            );

            result_window.connect_response(|dialog_window, response| {
                if response == ResponseType::Ok {
                    dialog_window.destroy();
                }
                dialog_window.destroy();
            });

            result_window.show();
    }));

    verify_degree_button.connect_clicked( clone!( 
        @weak main_window,
        @weak path_file_label,
        @weak vertex_list3_dropdown
         => move |_button| {
            let source = vertex_list3_dropdown.selected();

            let mut g = Representation::new();
            g.load_from_file(path_file_label.text().as_str());
            
            let result = g.verify_vertex_degree(source.try_into().unwrap());

            //let message = if result {"São adjacentes"} else {"Não são adjacentes"};

            let result_window = MessageDialog::new(
                Some(&main_window),
                DialogFlags::empty(),
                gtk::MessageType::Info,
                gtk::ButtonsType::Ok,
                result
            );

            result_window.connect_response(|dialog_window, response| {
                if response == ResponseType::Ok {
                    dialog_window.destroy();
                }
                dialog_window.destroy();
            });

            result_window.show();
    }));

    neighbors_button.connect_clicked( clone!( 
        @weak main_window,
        @weak path_file_label,
        @weak vertex_list4_dropdown
         => move |_button| {
            let source = vertex_list4_dropdown.selected();

            let mut g = Representation::new();
            g.load_from_file(path_file_label.text().as_str());
            
            let neighbors = g.get_neighbors_from(source.try_into().unwrap());

            let result = neighbors.iter()
            .map(|x| format!("v{}", x))
            .collect::<Vec<String>>()
            .join(",");

            //let message = if result {"São adjacentes"} else {"Não são adjacentes"};

            let result_window = MessageDialog::new(
                Some(&main_window),
                DialogFlags::empty(),
                gtk::MessageType::Info,
                gtk::ButtonsType::Ok,
                result
            );

            result_window.connect_response(|dialog_window, response| {
                if response == ResponseType::Ok {
                    dialog_window.destroy();
                }
                dialog_window.destroy();
            });

            result_window.show();
    }));

    export_png_button.connect_clicked(clone!(
        @weak main_window,
        @weak path_file_label
        => move |_| {
            let mut g = Representation::new();
            g.load_from_file(path_file_label.text().as_str());
            g.export_graphviz_file();

            let mut result = String::new();


            match use_graphviz::export_svg("graphviz.dot") {
                Ok(()) => result.push_str("Arquivo exportado em: ./graphviz.svg"),
                Err(e) => result.push_str(e.to_string().as_str()),
            }

            let result_window = MessageDialog::new(
                Some(&main_window),
                DialogFlags::empty(),
                gtk::MessageType::Info,
                gtk::ButtonsType::Ok,
                result
            );

            result_window.connect_response(|dialog_window, response| {
                if response == ResponseType::Ok {
                    dialog_window.destroy();
                }
                dialog_window.destroy();
            });

            result_window.show();
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
            @weak export_png_button,
            @weak vertex_list1_dropdown,
            @weak vertex_list2_dropdown,
            @weak vertex_list3_dropdown,
            @weak vertex_list4_dropdown,
            @weak tree_detect_button => move |file_explorer, response| {
            if response == ResponseType::Ok {
                if let Some(file) = file_explorer.file() {
                    if let Some(path) = file.path() {
                        //println!("Caminho do arquivo selecionado: {:?}", path.as_os_str());
                        let mut g = Representation::new();

                        let path = path.as_os_str().to_str().unwrap();
                        g.load_from_file(path);
                        //.iter().map(|&x| format!("v{}", x)).collect();
                        path_file_label.set_text(path);

                        let model1 = StringList::new(&[]);

                        for vertex in g.get_vertex_list(){
                            let opt = &("v".to_owned() +vertex.to_string().as_str());
                            model1.append(opt);
                        }

                        vertex_list1_dropdown.set_model(Some(&model1));
                        vertex_list1_dropdown.set_expression(Expression::NONE);
                        vertex_list1_dropdown.set_sensitive(true);
                        vertex_list2_dropdown.set_model(Some(&model1));
                        vertex_list2_dropdown.set_expression(Expression::NONE);
                        vertex_list2_dropdown.set_sensitive(true);
                        vertex_list3_dropdown.set_model(Some(&model1));
                        vertex_list3_dropdown.set_expression(Expression::NONE);
                        vertex_list3_dropdown.set_sensitive(true);
                        vertex_list4_dropdown.set_model(Some(&model1));
                        vertex_list4_dropdown.set_expression(Expression::NONE);
                        vertex_list4_dropdown.set_sensitive(true);
                        g.print_graph();
                        tree_detect_button.set_sensitive(true);
                        verify_adjc_button.set_sensitive(true);
                        verify_degree_button.set_sensitive(true);
                        neighbors_button.set_sensitive(true);
                        export_png_button.set_sensitive(true);
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