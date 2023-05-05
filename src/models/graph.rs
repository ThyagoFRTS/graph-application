use std::fs::File;
use std::io::{BufRead, BufReader };


use crate::utils::{create_mapper, create_file, remove_letters};

pub struct Representation {
    is_digraph: bool,
    vertex_list: Vec<usize>,
    labels_list: Vec<usize>,
    graph: Vec<Vec<i8>>,
}

impl Representation {
    pub fn new() -> Self {
        Self {
            vertex_list: Vec::new(),
            graph: Vec::new(),
            is_digraph: false,
            labels_list: Vec::new(),
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
        let mut edge_label_list: Vec<(usize, usize)> = Vec::new();
    
        let mut lines = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            lines.push(line);
        }
        //let graph_type = lines[0].clone();
        self.is_digraph = if lines[0].clone().eq("D") { true } else { false };

        lines.remove(0);
        
        for line in lines {
            let processed_line = remove_letters(&line);
            let edge: Vec<usize>  = processed_line.split(",")
                //.map(|element| element.to_string().parse::<usize>().unwrap())
                .map(|element| element.to_string().parse::<usize>().unwrap())
                .collect();
            edge_label_list.push((edge[0],edge[1]));
            self.labels_list.push(edge[0]);
            self.labels_list.push(edge[1]);
        }
            
        self.labels_list.sort();
        self.labels_list.dedup();
        
        let total_vertex =  self.labels_list.len();
        
        self.vertex_list = Vec::with_capacity(total_vertex);
        for i in 0..total_vertex {
            self.vertex_list.push(i);
        }

        let vertex_mapper = create_mapper(self.labels_list.clone());
        
        self.graph = vec![vec![0; total_vertex]; total_vertex];
        
        if self.is_digraph {
            for edge_pair in edge_label_list {
                let source = vertex_mapper(edge_pair.0).unwrap();
                let destiny = vertex_mapper(edge_pair.1).unwrap();
                self.graph[source][destiny] = 1;
            }
        } else {
            for edge_pair in edge_label_list {
                let source = vertex_mapper(edge_pair.0).unwrap();
                let destiny = vertex_mapper(edge_pair.1).unwrap();
                self.graph[source][destiny] = 1;
                self.graph[destiny][source] = 1;
            }
        }
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

    pub fn export_graphviz_file(&self){
        let g_type = if self.is_digraph {"digraph "} else {"graph "};

        let mut temp_graph = self.get_graph();

        let mut edges:Vec<(usize,usize)> = Vec::new();

        for i in 0..temp_graph.len() {
            for j in 0..temp_graph[i].len() {
                if temp_graph[i][j] == 1 {
                    edges.push((i,j));
                    // eu sei que dava pra colocar esse if fora do laço, de modo a fazer
                    // só uma comparação ao invés de NxN
                    // mas fica muito feio kkk
                    if !self.is_digraph {
                        temp_graph[j][i] = 0;
                    }
                }
            }
        }
        let separator = if self.is_digraph { "->" } else {  "--" };

        let mut corpus = String::new();

        for edge in edges.into_iter() {
            let line = String::from(format!("\t{} {} {}\n",edge.0.to_string(),separator, edge.1.to_string()));
            corpus.push_str(line.as_str());
        }

        let content = String::from(
            format!(
                "{} {{\n{} }}",
                g_type,
                corpus
            )
        );
        println!("{:?}", content);
        create_file(content);

    }
}