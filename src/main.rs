mod bfs;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use bfs::bfs;

pub fn read_txt(file_name: &str) -> HashMap<i32, HashSet<i32>> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    // creates an empty hashmap to organize edges
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

    // iterates through each edge
    for line in reader.lines() {
        let edge = line.unwrap();
        let nodes: Vec<i32> = edge.split(" ").map(|n| n.parse().unwrap()).collect();

        if nodes.len() != 2 {
            panic!("Invalid input file format: each line must contain two nodes");
        }

        let u = nodes[0];
        let v = nodes[1];
        graph.entry(u).or_insert(HashSet::new()).insert(v);
        graph.entry(v).or_insert(HashSet::new()).insert(u);    
    }

    graph
}

pub fn read_csv(file_name: &str) -> HashMap<i32, HashSet<i32>> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    // creates an empty hashmap to organize edges
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

    // iterates through each edge
    for line in reader.lines() {
        let edge = line.unwrap();
        let nodes: Vec<i32> = edge.split(",").map(|n| n.parse().unwrap()).collect();

        if nodes.len() != 2 {
            panic!("Invalid input file format: each line must contain two nodes");
        }

        let u = nodes[0];
        let v = nodes[1];
        graph.entry(u).or_insert(HashSet::new()).insert(v);
        graph.entry(v).or_insert(HashSet::new()).insert(u);    
    }

    graph
}

pub fn avg_distance(adj_list: &HashMap<i32, HashSet<i32>>) -> f64 {
    let mut total_distance = 0;
    let mut total_pairs = 0;

    for &u in adj_list.keys() {
        for &v in adj_list.keys() {
            if u < v {
                if let Some(dist) = bfs(u, v, &adj_list) {
                    total_distance += dist;
                    total_pairs += 1;
                }
            }
        }
    }

    total_distance as f64 / total_pairs as f64
}

fn main() {
    let fb_graph: HashMap<i32, HashSet<i32>> = read_txt("facebook.txt");
    let fb_avg_dist: f64 = avg_distance(&fb_graph);
    println!("Average distance between all pairs of nodes from facebook.txt: {}", fb_avg_dist);

    let lastfm_graph: HashMap<i32, HashSet<i32>> = read_csv("lastfm_asia.csv");
    let lastfm_avg_dist: f64 = avg_distance(&lastfm_graph);
    println!("Average distance between all pairs of nodes from lastfm_asia.csv: {}", lastfm_avg_dist);

    let difference: f64 = fb_avg_dist - lastfm_avg_dist;
    println!("Absolute difference between average distances: {}", difference.abs());
}
