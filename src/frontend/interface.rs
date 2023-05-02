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
    Button,
    FileFilter,
    Orientation, 
    Label,
    Frame,
    DropDown,
    StringList,
    Expression,
    Separator, 
    ResponseType
};
use crate::models::graph::Graph;

pub fn build_ui(app: &Application) {
    let main_window = ApplicationWindow::new(app);
    main_window.set_title(Some("Graph App"));
    main_window.set_default_size(500, 500);

    let horiozontal_line1 = Separator::builder()
        .orientation(Orientation::Horizontal)
        .height_request(2)
        .margin_start(12)
        .margin_end(12)
        .build();

    let horiozontal_line2 = Separator::builder()
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

    let verify_adjc_button = Button::builder()
        .label("Verificar adjacência")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .sensitive(false)
        .build();

    let tree_detect_button = Button::builder()
        .label("Detectar árvore")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .sensitive(false)
        .build();

    

    //let vertex_list1_dropdown = DropDown::new(Some(model), gtk::Expression::NONE);
    let vertex_list1_dropdown = DropDown::builder()
        //.expression(gtk::Expression::NONE)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .sensitive(false)
        .build();

    let vertex_list2_dropdown = DropDown::builder()
        //.expression(gtk::Expression::NONE)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .sensitive(false)
        .build();

    let vertex_list3_dropdown = DropDown::builder()
        //.expression(gtk::Expression::NONE)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .sensitive(false)
        .build();

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
    buttons_layout.append(&tree_detect_button);

    let vertex_layout = Box::new(Orientation::Horizontal, 0);
    vertex_layout.set_halign(Align::Center);
    vertex_layout.append(&vertex_list1_dropdown);
    vertex_layout.append(&vertex_list2_dropdown);
    vertex_layout.append(&verify_adjc_button);
    //vertex_layout.append(&tree_detect_button);

    let main_layout = Box::new(Orientation::Vertical, 0);
    main_layout.append(&buttons_layout);
    main_layout.append(&path_file_label);
    main_layout.append(&vertex_layout);
    main_layout.append(&horiozontal_line1);
    //main_layout.append(&horiozontal_line);
    main_layout.append(&output_frame);

    verify_adjc_button.connect_clicked( clone!( 
        @weak main_window,
        @weak path_file_label,
        @weak vertex_list1_dropdown,
        @weak vertex_list2_dropdown
         => move |_button| {
            let source = vertex_list1_dropdown.selected();
            let destiny = vertex_list2_dropdown.selected();

            let mut g = Graph::new();
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

    load_file_button.connect_clicked(clone!(
        @weak main_window, 
        @weak path_file_label,
        @weak tree_detect_button,
        @weak verify_adjc_button,
        @weak vertex_list1_dropdown,
        @weak vertex_list2_dropdown
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
            @weak vertex_list1_dropdown,
            @weak vertex_list2_dropdown,
            @weak vertex_list3_dropdown,
            @weak tree_detect_button => move |file_explorer, response| {
            if response == ResponseType::Ok {
                if let Some(file) = file_explorer.file() {
                    if let Some(path) = file.path() {
                        //println!("Caminho do arquivo selecionado: {:?}", path.as_os_str());
                        let mut g = Graph::new();

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
                        //vertex_list3_dropdown.set_model(Some(&model1));
                        //vertex_list3_dropdown.set_expression(Expression::NONE);
                        //vertex_list3_dropdown.set_sensitive(true);
                        g.print_graph();
                        tree_detect_button.set_sensitive(true);
                        verify_adjc_button.set_sensitive(true);
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