use crate::edge::Edge;

pub struct Node {
    id: i32,
    weight: f32,
    edges: Vec<Edge>,
}

impl Node {
    pub fn new(id: i32, weight: Option<f32>) -> Self {
        Node {
            id: id,
            weight: weight.unwrap_or(0.0),
            edges: Vec::new(),
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    pub fn get_edges(&self) -> Vec<Edge> {
        self.edges.clone()
    }

    pub fn insert_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    pub fn remove_edge(&mut self, target: i32) {
        self.edges.retain(|e| e.get_target_id() != target);
    }

    pub fn remove_all_edges(&mut self) {
        self.edges.clear()
    }

    pub fn search_edge(&self, target: i32) -> Option<&Edge> {
        self.edges.iter().find(|e| e.get_target_id() == target)
    }
}

