#![allow(dead_code)]
#![allow(unused_variables)]

use crate::node::Node;
use crate::edge::Edge;
use std::fs::File;
use std::io::Write;

pub struct Graph {
    order: i32,
    number_edges: i32,
    nodes: Vec<Node>,
    has_node_weight: bool,
    has_edge_weight: bool,
    is_directed: bool,
}

impl Graph {
    pub fn new(order: i32, has_node_weight: bool, has_edge_weight: bool, is_directed: bool) -> Self {
        Graph {
            order,
            number_edges: 0,
            nodes: Vec::new(),
            has_node_weight,
            has_edge_weight,
            is_directed,
        }
    }

    pub fn get_order(&self) -> i32 { self.order }

    pub fn get_node(&self, id: i32) -> Option<Node> {
        self.nodes.iter().find(|n| n.get_id() == id).cloned()
    }

    fn get_node_mut(&mut self, id: i32) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.get_id() == id)
    }

    pub fn insert_node(&mut self, id: i32, weight: Option<f32>) {
        if self.nodes.iter().all(|n| n.get_id() != id) {
            self.nodes.push(Node::new(id, weight));
        }
    }

    pub fn insert_edge(&mut self, id: i32, target: i32, weight: f32) {
        if !self.search_node(id) {
            self.insert_node(id, None);
        }

        if !self.search_node(target) {
            self.insert_node(target, None);
        }

        if let Some(node) = self.get_node_mut(id) {
            node.insert_edge(Edge::new(target, Some(weight)));
        }

        if !self.is_directed && let Some(node) = self.get_node_mut(target) {
            node.insert_edge(Edge::new(id, Some(weight)));
        }
        self.number_edges += 1
    }

    pub fn remove_node(&mut self, id: i32) {println!("{}", id)}

    pub fn search_node(&mut self, id: i32) -> bool {
        self.nodes.iter().all(|n| n.get_id() != id)
    }

    pub fn print(&self) {
        println!("Graph order: {}, number of edges: {}", self.order, self.number_edges);
        for node in self.nodes.iter() {
            print!("{}", node.get_id());
            for edge in node.get_edges() {
                print!(" - {} ({})", edge.get_target_id(), edge.get_weight())
            }
            println!("")
        }
    }

    pub fn to_dot(&self, filename: &str) -> std::io::Result<()> {
        let mut dot_str = String::new();
        let line_type: &str;

        if self.is_directed {
            line_type = "->"
        } else {
            line_type = "--"
        }

        dot_str = dot_str + "strict graph {\n";

        if self.has_node_weight {
            for node in self.nodes.iter() {
                dot_str = dot_str + "    " + &node.get_id().to_string() + "[weight = " + &node.get_weight().to_string() + "]" + "\n";
            }
        }

        for node in self.nodes.iter() {
            for edge in node.get_edges() {
                dot_str = dot_str + "   " + &node.get_id().to_string();

                dot_str = dot_str + line_type + &edge.get_target_id().to_string();

                if self.has_edge_weight{
                    dot_str = dot_str + "[label=" + &edge.get_weight().to_string() + ", weight=" + &edge.get_weight().to_string() + "]";
                }
                dot_str = dot_str + "\n"
            }
        }

        dot_str = dot_str + "}";

        let mut file = File::create(filename)?;
        file.write_all(dot_str.as_bytes())?;

        Ok(())
    }
}
