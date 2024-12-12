use std::collections::VecDeque;
//Performs BFS for all nodes
pub fn computeallbfs(adj: &Vec<Vec<usize>>) -> Vec<Vec<Option<u32>>> {
    (0..adj.len())
        .map(|node| {
            let mut distance = vec![None; adj.len()];
            distance[node] = Some(0); 
            let mut queue = VecDeque::from([node]);

            while let Some(vertex) = queue.pop_front() {
                adj[vertex].iter().for_each(|&current| {
                    if distance[current].is_none() {
                        distance[current] = distance[vertex].map(|d| d + 1);
                        queue.push_back(current);
                    }
                });
            }

            println!("{}", node); 
            distance
        })
        .collect()
}

//Prints all the BFS for each individual node
pub fn printallbfs(distances: &[Vec<Option<u32>>]) {
    for (current_node, distances) in distances.iter().enumerate() {
        print!("BFS for node {}: ", current_node);
        for (v, dist) in distances.iter().enumerate() {
            print!("{}:{} ", v, dist.unwrap());
        }
        println!();
    }
}

//Performs BFS for one selected node a certain amount of other nodes
//Fast runtime and can target specific nodes of interest
pub fn onebfs(selected: usize, lastnode: usize, adj: &Vec<Vec<usize>>) {
    let mut distance: Vec<Option<u32>> = vec![None; adj.len()];
    distance[selected] = Some(0); 
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(selected);
    while let Some(node) = queue.pop_front() {
        for &current in &adj[node] {
            if distance[current].is_none() {
                distance[current] = Some(distance[node].unwrap() + 1);
                queue.push_back(current);
            }
        }
    }
    print!("Distances from BFS for node {} (from 0 up to node {}): ", selected, lastnode);
    for (node, dist) in distance.iter().enumerate().take(lastnode + 1) { //The internet helped me with this line
        print!("{}:{} ", node, dist.unwrap());
    }
}