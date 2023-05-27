/* dijkstra's algorithm */

mod graph;

use crate::graph::*;

fn main() {
    // let graph = Graph::test_graph();                         // testing    
    // let sz = 6;
    
    let (graph, sz, target) = create_graph();
    let mut least_distances = set_distances(sz);

    let mut visited : Vec<usize> = vec![0];          

    while !visited.contains(&target) {
        if visited.len() == sz || target >= sz {
            println!("There is no valid path to that node.");
            break;
        }

        let current_idx = visited[visited.len() - 1];
        let current_node = least_distances[current_idx];
        let distances = &graph[current_idx];

        update_least(&mut least_distances, &distances, current_node); 
        visited.push(get_least(&visited, &least_distances));
    }

    print_distances(target, &least_distances);
}

