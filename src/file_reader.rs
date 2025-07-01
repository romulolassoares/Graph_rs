use std::fs::File;
use std::io::{self, BufRead};
//use std::path::Path;
//use crate::edge::Edge;
//use crate::node::Node;
use crate::graph::Graph;

pub fn freader(filename: &str) -> io::Result<(Graph)> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines().filter_map(Result::ok);

    let order: i32 = lines
        .next()
        .expect("Empty File")
        .trim()
        .parse()
        .expect("Invalid Order");

    let mut graph = Graph::new(order);

    for _ in 0..order {
        if let Some(line) = lines.next() {
            let node_part: Vec<&str> = line.trim().split_whitespace().collect();
            let id: i32 = node_part[0].parse().expect("Invalid ID");
            let weight: f32 = node_part[1].parse().expect("Invalid Weight");
            graph.insert_node(id, Some(weight));
        }
    }

    for line in lines {
        let edge_part: Vec<&str> = line.trim().split_whitespace().collect();
        let src: i32 = edge_part[0].parse().expect("Invalid SRC Node");
        let dst: i32 = edge_part[1].parse().expect("Invalid DST Node");
        let weight: f32 = edge_part[2].parse().expect("Invalid Weight");
        graph.insert_edge(src, dst, weight);
    }

    Ok(graph)
}
