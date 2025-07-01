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

        if let Some(node) = self.get_node_mut(target) {
            node.insert_edge(Edge::new(target, Some(weight)));
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
}
