use std::collections::HashMap;


// pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    
// }

pub fn dfs (connections: &HashMap<i32, Vec<i32>>, prev_node: i32, cur_node: i32) {
   // how do i get the neightbors
    let neighbors = connections.get(&cur_node).unwrap();

    for neighbor in neighbors {
        if neighbor != &prev_node {
            dfs(&connections, cur_node, *neighbor);
        }else{

        }
    }
}

// TODO make this directed
pub fn build_graph (connections: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

    for city in connections {
        graph.entry(city[0])
        .or_insert(vec![])
        .push(city[1]);

        graph.entry(city[1])
        .or_insert(vec![])
        .push(city[0]);
    }

    graph
}

fn main() {
    println!("Hello, world!");
}
