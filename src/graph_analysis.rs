//Computes the average distance between nodes
pub fn avgdistance(distances: &Vec<Vec<Option<u32>>>) -> f64 {
    let mut total = 0;
    let mut count: usize = 0;
    for distance in distances.iter() {
        for &current in distance.iter() {
            if let Some(current) = current {
                total += current as usize;
                count += 1;
            }
        }
    }
    let average = total as f64 / count as f64;
    return average;
}
//Returns the pairs of nodes that are the furthest away from one another in terms of degrees of seperation
pub fn furthest(all_distances: &Vec<Vec<Option<u32>>>) -> Vec<(usize, usize, u32)> {
    let mut maxd = 0;
    let mut furthest = Vec::new();
    for (nodeone, distances) in all_distances.iter().enumerate() {
        for (nodetwo, &distance) in distances.iter().enumerate() {
            if let Some(d) = distance {
                if d > maxd {
                    maxd = d;
                    furthest.clear(); 
                    furthest.push((nodeone, nodetwo, maxd));
                } else if d == maxd {
                    furthest.push((nodeone, nodetwo, maxd));
                }
            }
        }
    }
    return furthest
}


//Calculates the distribution of nodes between degrees
pub fn distribution(distances: &Vec<Vec<Option<u32>>>, degrees: u32) -> f64 {
    let mut total: u32 = 0;
    let mut target: u32 = 0;
    for (nodeone, node) in distances.iter().enumerate() {
        for (nodetwo, &d) in node.iter().enumerate() {
            if nodeone != nodetwo { //This gets rid of nodes with distance 0 
                if let Some(distance) = d { 
                    total += 1;
                    if distance == degrees {
                        target += 1;
                    }   
                }
            }
        }
    }

    let percent = (target as f64 / total as f64) * 100.0;
    
    return percent;
}