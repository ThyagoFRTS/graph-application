

fn get_neighbors(arr: &mut Vec<i8>) -> Vec<usize>{
    let mut indexes: Vec<usize> = Vec::new();
    for (i, &val) in arr.iter().enumerate() {
        if val == 1 {
            indexes.push(i);
        }
    }
    indexes
}

fn deep_search_in(vertex: usize, stack: &mut Vec<bool>, graph_line:  &mut Vec<Vec<i8>>, path: &mut Vec<(usize, usize)>,pos_ordering: bool){
    stack[vertex] = true;
    let neighbors = get_neighbors(&mut graph_line[vertex]);
    for neighbor in neighbors {
        if stack[neighbor] == false {
            if pos_ordering {
                println!("{} > {}",vertex + 1, neighbor + 1);
            }
            path.push((vertex, neighbor));
            deep_search_in(neighbor, stack, graph_line, path, pos_ordering);
            
        }
    }
    
    if !pos_ordering {
        println!("> {}", vertex + 1);
    }
}

pub fn deep_search(graph: Vec<Vec<i8>>) -> Vec<(usize, usize)>{
    let mut graph = graph;
    let total_vertex = graph.len();
    let mut stack = vec![false; total_vertex];
    let mut path: Vec<(usize, usize)> = Vec::new();
    for vertex in 0..total_vertex {
        
        if !stack[vertex] {
            deep_search_in(vertex, &mut stack, &mut graph, &mut path, true);
        }
    }
    path
}
/* 
pub fn vist_all_edges(graph: Vec<Vec<i8>>, start: usize) {
    let mut graph = graph;
    let mut stack: Vec<(usize,usize)> = vec![];

}
*/
pub fn lenght_search(graph: Vec<Vec<i8>>, vertex: usize){
    let mut graph = graph;
    let total_vertex = graph.len();
    let mut stack: Vec<usize> = vec![];
    let mut visited_vertex = vec![false; total_vertex];
    
    stack.push(vertex);
    visited_vertex[vertex] = true;

    while !stack.is_empty() {
        let u = stack[0];
        stack.remove(0);
        let neighbors = get_neighbors(&mut graph[u]);
        //println!("{:?}",neighbors);
        
        for neighbor in neighbors {
            if visited_vertex[neighbor] == false {
                visited_vertex[neighbor] = true;

                println!("{} > {}",vertex+1, neighbor+1);
                stack.push(neighbor);
                
            }
        }
        //println!("------",);
    }

}

pub fn topological_ordering(graph: Vec<Vec<i8>>) -> Vec<(usize, usize)>{
    let mut graph = graph;
    let total_vertex = graph.len();
    let mut stack = vec![false; total_vertex];
    let mut path: Vec<(usize, usize)> = Vec::new();
    for vertex in 0..total_vertex {
        
        if !stack[vertex] {
            deep_search_in(vertex, &mut stack, &mut graph, &mut path, false);
        }
    }
    path
}
