//This function creates an adjacency list to use for BFS from the inputed graph
pub fn create_adjacency_list(graph: Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let length = graph
        .iter()
        .fold(0, |max_val, &(node, edge)| max_val.max(node).max(edge)) + 1; // Determine the length

    let mut adj = (0..length).map(|_| Vec::new()).collect::<Vec<_>>(); // Initialize adjacency list

    graph.into_iter().for_each(|(node, edge)| {
        adj[node].push(edge);
        adj[edge].push(node);
    });

    adj
}

//This function is used the print the adjacency list
pub fn print_adjacency_list(adjacency_lists: &Vec<Vec<usize>>) {
    //To print the entire adjacency list
    for (node, neighbors) in adjacency_lists.iter().enumerate() {
        println!("Node {}: {:?}", node, neighbors);
    }
}
