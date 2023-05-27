
mod parsers;

use crate::graph::parsers::*;

// pub fn test_graph() -> Vec<Vec<u32>> {
//     vec![
//         vec![0,7,12,0,0,0],
//         vec![0,0,2,9,0,0],
//         vec![0,0,0,0,10,0],
//         vec![0,0,0,0,0,1],
//         vec![0,0,0,4,0,5],
//         vec![0; 6],
//     ]
// }

pub fn create_graph() -> (Vec<Vec<u32>>, usize, usize) {
    let mut buf = String::new();

    input("Input number of vertices: ", &mut buf);
    let sz : usize = parse_input_usz(&mut buf);

    let mut matrix : Vec<Vec<u32>> = Vec::new();
    for i in 0..sz {
        input(&format!("Adjacency array of node {i}:\n"), &mut buf);
        let node : Vec<u32> = parse_input_vec(&mut buf);

        matrix.push(node);
    }

    input("Input target node: ", &mut buf);
    let target : usize = parse_input_usz(&mut buf);

    (matrix, sz, target)
}

pub fn set_distances(sz : usize) -> Vec<u32> {
    let mut distances : Vec<u32> = vec![u32::MAX; sz];    // distances array initalized to all inf
    distances[0] = 0;                                     // source as 0
    distances
}

pub fn get_least(visited : &Vec<usize>, least_distances : &Vec<u32>) -> usize {
    let mut least_tup = least_distances
        .iter()
        .enumerate()
        .collect::<Vec<(usize, &u32)>>();
    
    least_tup.retain(|x| {!visited.contains(&x.0)});            // set difference

    least_tup.iter()
        .min_by(|x, y| x.1.cmp(y.1))
        .unwrap().0
}

pub fn update_least(least_distances : &mut Vec<u32>, distances : &Vec<u32>, current_node : u32) {
    let distance_tup = least_distances
        .iter_mut()
        .zip(distances.iter());

    for (a, b) in distance_tup {
        if *b == 0 {}
        else if *a > current_node + *b {
            *a = current_node + *b;
        }
    }
}

pub fn print_distances(target : usize, least_distances : &Vec<u32>) {
    println!("The least distances to all nodes on the path to {} are:\n", target);
    for (node, dist) in least_distances.iter().enumerate() {
        println!("Node {}: {}", node, dist);
    }
}