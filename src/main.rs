mod edge;
mod node;
mod graph;

fn main() {
    let mut graph = graph::Graph::new(30);

    // Insert nodes explicitly (optional, insert_edge auto-inserts nodes if missing)
    graph.insert_node(1, Some(1.0));
    graph.insert_node(2, Some(2.0));
    graph.insert_node(3, Some(3.0));

    // Insert edges (auto-inserts nodes if needed)
    graph.insert_edge(1, 2, 5.0);
    graph.insert_edge(2, 3, 3.5);
    graph.insert_edge(1, 3, 1.2);

    // Print the graph structure
    graph.print();
}
