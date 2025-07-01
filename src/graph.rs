use crate::node::Node;
use crate::edge::Edge;

pub struct Graph {
    order: i32,
    number_edges: i32,
    nodes: Vec<Node>,
}

impl Graph {
    pub fn new(order: i32) -> Self {
        Graph {
            order,
            number_edges: 0,
            nodes: Vec::new(),
        }
    }

    pub fn get_order(&self) -> i32 { self.order }

    pub fn get_node(&self) -> Option<Node> { None }

    pub fn insert_node(&mut self, id: i32) {}

    pub fn insert_edge(&mut self, id: i32, target: i32, weight: f32) {}

    pub fn remove_node(&mut self, id: i32) {}

    pub fn search_node(&mut self, id: i32) {}
}
