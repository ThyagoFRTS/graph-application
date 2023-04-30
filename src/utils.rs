use std::fs::File;
use itertools::Itertools;
use std::io::{BufRead, BufReader};

pub fn read_graph(file_path: &str) -> (Vec<Vec<i8>>,String) {
    let file = File::open(file_path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let mut graph_type: String = String::new() ;
    let mut vertex_list: Vec<usize> = Vec::new();
    let mut edge_list: Vec<(usize, usize)> = Vec::new();

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        lines.push(line);
    }

    graph_type = lines[0].clone();
    lines.remove(0);
    
    for line in lines {
        let processed_line = line.replace("v", "");
        let edge: Vec<usize>  = processed_line.split(",")
            .map(|element| element.to_string().parse::<usize>().unwrap())
            .collect();
            edge_list.push((edge[0],edge[1]));
        
        vertex_list.append(&mut edge.clone());
        
    }
    /*
    
    let vertex_list: Vec<usize> = vertex_list.iter().unique()
                        .collect::<Vec<&usize>>()
                        .iter().map(|&x| *x).collect();
     */
    vertex_list.sort();
    vertex_list.dedup();
    println!("{:?}", vertex_list);

    let total_vertex =  vertex_list.len();
    
    let mut graph: Vec<Vec<i8>> = vec![vec![0; total_vertex]; total_vertex];
    println!("{:?}",graph_type);

    for edge_pair in edge_list {
        graph[edge_pair.0][edge_pair.1] = 1;
    }
    for vertex_adjacency in graph.clone() {
        println!("{:?}",vertex_adjacency);
    }
    (graph, graph_type)
}