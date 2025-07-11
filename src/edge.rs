#![allow(dead_code)]
#![allow(unused_variables)]
#[derive(Clone)]

pub struct Edge {
    target_id: i32,
    weight: f32,
}

impl Edge {
    pub fn new(target_id: i32, weight: Option<f32>) -> Self {
        Edge {
            target_id,
            weight: weight.unwrap_or(0.0),
        }
    }

    pub fn get_target_id(&self) -> i32 {
        self.target_id
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    pub fn set_weight(&mut self, weight: f32) {
        self.weight = weight;
    }
}