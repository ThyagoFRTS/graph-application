use std::fs::File;
use std::io::{BufRead, BufReader};


pub struct Graph {
    is_digraph: bool,
    vertex_list: Vec<usize>,
    graph: Vec<Vec<i8>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            vertex_list: Vec::new(),
            graph: Vec::new(),
            is_digraph: false,
        }
    }

    pub fn print_graph(&self) {
        for vertex_adjacency in &self.graph {
            println!("{:?}",vertex_adjacency);
        }
        
    }

    pub fn get_graph(&self) ->  Vec<Vec<i8>> {
        self.graph.clone()
    }

    pub fn get_vertex_list(&self) ->  Vec<usize> {
        self.vertex_list.clone()
    }
    
    pub fn load_from_file(&mut self, file_path: &str) {
        let file = File::open(file_path).unwrap();
        let reader: BufReader<File> = BufReader::new(file);
        self.vertex_list = Vec::new();
        let mut edge_list: Vec<(usize, usize)> = Vec::new();
    
        let mut lines = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            lines.push(line);
        }
    
        let graph_type = lines[0].clone();
        lines.remove(0);
        
        for line in lines {
            let processed_line = line.replace("v", "");
            let edge: Vec<usize>  = processed_line.split(",")
                .map(|element| element.to_string().parse::<usize>().unwrap())
                .collect();
                edge_list.push((edge[0],edge[1]));
            
            self.vertex_list.append(&mut edge.clone());
            
        }

        self.vertex_list.sort();
        self.vertex_list.dedup();
    
        let total_vertex =  self.vertex_list.len();
        
        self.graph = vec![vec![0; total_vertex]; total_vertex];
        //println!("{:?}",graph_type);
    
        for edge_pair in edge_list {
            self.graph[edge_pair.0][edge_pair.1] = 1;
        }
        


        self.is_digraph = if graph_type.eq("D") { true } else { false };

    }

    pub fn is_adjacent(&self,source: usize, destiny: usize) -> bool{
        if self.graph[source][destiny] == 1 { true } else { false }
    }

    pub fn verify_vertex_degree(&self, vertex: usize) -> String {
        if self.is_digraph {
            let mut total_in = 0;
            let mut total_out = 0;

            for element in self.graph[vertex].clone().into_iter() {
                total_out = total_out + element as i32;
            }

            for line in self.graph.clone().into_iter() {
                total_in = total_in + line[vertex] as i32;
            }

            let output = format!("Grau de saída: {} Grau de entrada: {}",total_out,total_in);
            return output;

        } else {
            let mut total = 0;
            for element in self.graph[vertex].clone().into_iter() {
                total = total + element as i32;
            }

            let output = format!("Grau do vétice: {}", total);
            return output;
        }
    }

    pub fn get_neighbors_from(&self, vertex: usize) -> Vec<usize> {
        let mut neighbors:Vec<usize> = Vec::new();
        for (i, &val) in self.graph[vertex].clone().iter().enumerate() {
            if val == 1 {
                neighbors.push(i);
            }
        }
        neighbors
    }



    
}