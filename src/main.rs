mod models;
//mod graphy_search;



fn main() {
    // --snip--
    //let file_path = "src/data/ordering.txt";

    //let graph: Vec<Vec<i8>> = utils::read_graph(file_path);
    //let path = graphy_search::deep_search(graph.clone());
    //let path = graphy_search::topological_ordering(graph.clone());
    //println!("{:?}", path);
    //graphy_search::lenght_search(graph.clone(), 3);
    //let (graph, graph_tyo) = utils::read_graph("src/data/input.txt");
    
    let mut g = models::graph::Graph::new();
    g.load_from_file("src/data/input.txt");
    g.print_graph();
     

    
}
