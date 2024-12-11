mod adjacency_list;
mod bfs;
mod graph_analysis;

use std::io::BufRead;
use std::fs::File;

use adjacency_list::{create_adjacency_list, print_adjacency_list};
use bfs::{printallbfs, computeallbfs, onebfs};
use graph_analysis::{avgdistance, furthest, distribution};


fn main() {
    let graph = reader("/Users/coleouyang/Downloads/twitch/ENGB/musae_ENGB_edges.csv");
    let adjlist = create_adjacency_list(graph);
    print_adjacency_list(&adjlist);

    let node = 15;
    let lastnode = 100;
    onebfs(node, lastnode, &adjlist);
    
    let distances = computeallbfs(&adjlist);
    let avgdistances = avgdistance(&distances);
    println!("The average distance is {}", avgdistances);
    printallbfs(&distances);

    let furthestnodes = furthest(&distances);
    println!("Nodes with the maximum distance are ");
    for (node1, node2, furthestdistance) in furthestnodes {
        println!("node {}, node {} with distance {}", node1, node2, furthestdistance);
    }
    
    for target_degree in 1..=7 {
        // Calculate and print the percentage of nodes at the current distance
        let percentage = distribution(&distances, target_degree);
        println!("Percentage of nodes at distance {}: {:.2}%", target_degree, percentage);
    }


    fn reader(path: &str) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        let file = File::open(path).expect("Could not open file");
        let buf_reader = std::io::BufReader::new(file).lines();
        
        for (line_num, line) in buf_reader.enumerate() {
            match line {
                Ok(line_str) => {
                    let v: Vec<&str> = line_str.trim().split(',').collect();
                    if v.len() == 2 {
                        let parsed_x = v[0].parse::<usize>();
                        let parsed_y = v[1].parse::<usize>();
                        match (parsed_x, parsed_y) {
                            (Ok(x), Ok(y)) => result.push((x, y)),
                            _ => eprintln!("Warning: Could not parse numbers on line {}: {}", line_num + 1, line_str),
                        }
                    } else {
                        eprintln!("Warning: Invalid line format on line {}: {}", line_num + 1, line_str);
                    }
                }
                Err(e) => eprintln!("Error reading line {}: {}", line_num + 1, e),
            }
        }
        result
    }
}

#[test]
fn furthesttest() {
    let edges = vec![(0, 1), (1, 2), (1, 3), (3, 4), (3, 5), (5, 6),];
    let adjacency_list = create_adjacency_list(edges);
    let all_distances = computeallbfs(&adjacency_list);
    let result = furthest(&all_distances);
    let expected = vec![(0, 6, 4), (2, 6, 4), (6, 0, 4), (6, 2, 4),];

     assert_eq!(result, expected);
}


#[test]
fn avgdistancetest() {
    let edges = vec![(0, 1), (1, 2), (1, 3), (3, 4), (3, 5), (5, 6),];

    let adjacency_list = create_adjacency_list(edges);
    let all_distances = computeallbfs(&adjacency_list);
    let result = avgdistance(&all_distances);
    let expected = 1.877;

    assert!((result - expected).abs() < 0.001, "Expected {}, got {}", expected, result);
    }

