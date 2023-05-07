use gtk::traits::{WidgetExt, DialogExt, GtkWindowExt};
use gtk::{
    Button,
    Label,
    DropDown,
    StringList,
    Expression, 
    ApplicationWindow, 
    ResponseType, 
    DialogFlags, 
    MessageDialog
};

use crate::cmd::use_graphviz;
use crate::graphy_search::{deep_search, visit_all_edges};
use crate::models::graph::Representation;

pub fn setup_ui_components(
    graph: &Representation,
    verify_adjc_button: &Button,
    neighbors_button: &Button,
    verify_degree_button: &Button,
    edges_button: &Button,
    export_png_button: &Button,
    tree_detect_button: &Button,
    vertex_list1_dropdown: &DropDown,
    vertex_list2_dropdown: &DropDown,
    vertex_list3_dropdown: &DropDown,
    vertex_list4_dropdown: &DropDown
    ) {
    let model1 = StringList::new(&[]);

    for vertex in graph.get_vertex_list(){
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
    graph.print_graph();
    tree_detect_button.set_sensitive(true);
    verify_adjc_button.set_sensitive(true);
    verify_degree_button.set_sensitive(true);
    edges_button.set_sensitive(true);
    neighbors_button.set_sensitive(true);
    export_png_button.set_sensitive(true);
}

pub fn handle_vertex_neighbors(
    main_window: &ApplicationWindow,
    path_file_label: &Label,
    vertex_list_dropdown: &DropDown
) {
    let source = vertex_list_dropdown.selected();

    let mut g = Representation::new();
    g.load_from_file(path_file_label.text().as_str());
    
    let neighbors = g.get_neighbors_from(source.try_into().unwrap());

    let result = neighbors.iter()
    .map(|x| format!("v{}", x))
    .collect::<Vec<String>>()
    .join(",");

    let result_window = MessageDialog::new(
        Some(main_window),
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

pub fn handle_vertex_degree (
    main_window: &ApplicationWindow,
    path_file_label: &Label,
    vertex_list_dropdown: &DropDown
) {
    let source = vertex_list_dropdown.selected();

    let mut g = Representation::new();
    g.load_from_file(path_file_label.text().as_str());
    
    let result = g.verify_vertex_degree(source.try_into().unwrap());

    let result_window = MessageDialog::new(
        Some(main_window),
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

pub fn handle_vertex_adjacency (
    main_window: &ApplicationWindow,
    path_file_label: &Label,
    vertex_list1_dropdown: &DropDown,
    vertex_list2_dropdown: &DropDown
) {
    let source = vertex_list1_dropdown.selected();
    let destiny = vertex_list2_dropdown.selected();

    let mut g = Representation::new();
    g.load_from_file(path_file_label.text().as_str());
    let result = g.is_adjacent(source.try_into().unwrap(), destiny.try_into().unwrap());

    let message = if result {"São adjacentes"} else {"Não são adjacentes"};

    let result_window = MessageDialog::new(
        Some(main_window),
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
}

pub fn handle_export_svg (
    main_window: &ApplicationWindow,
    path_file_label: &Label
) {
    let mut g = Representation::new();
    g.load_from_file(path_file_label.text().as_str());
    g.export_graphviz_file();

    let mut result = String::new();

    match use_graphviz::export_svg("graphviz.dot") {
        Ok(()) => result.push_str("Arquivo exportado em: ./graphviz.svg"),
        Err(e) => result.push_str(e.to_string().as_str()),
    }

    let result_window = MessageDialog::new(
        Some(main_window),
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

pub fn handle_tree_detection(
    path_file_label: &Label,
    output_label: &Label,
) {
    let mut g = Representation::new();
    g.load_from_file(path_file_label.text().as_str());

    let path = deep_search(g.get_graph());

    let result = path.iter()
        .map(|edge| format!("(v{},v{})", edge.0,edge.1))
        .collect::<Vec<String>>()
        .join("->");

    output_label.set_text(result.clone().as_str());
    //output_frame.set_label_widget(Some(&output_label2));
}

pub fn handle_visit_edges(
    path_file_label: &Label,
    output_label: &Label,
) {
    let mut g = Representation::new();
    g.load_from_file(path_file_label.text().as_str());

    let path = visit_all_edges(g.get_graph(), g.get_graph_type());

    let result = path.iter()
        .map(|edge| format!("(v{},v{})", edge.0,edge.1))
        .collect::<Vec<String>>()
        .join("->");

    output_label.set_text(result.clone().as_str());
    //output_frame.set_label_widget(Some(&output_label2));
}