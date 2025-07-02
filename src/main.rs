use crate::file_reader::freader;

mod edge;
mod file_reader;
mod graph;
mod node;

fn main() {
    let filename: &str = "data/graph_30.txt"; // Nome do arquivo com os dados do grafo
    let has_node_weight: bool = true;
    let has_edge_weight: bool = false;
    let is_directed: bool = true;

    let graph = match freader(filename, has_node_weight, has_edge_weight, is_directed) {
        Ok(graph) => graph,
        Err(e) => {
            eprintln!("Erro ao ler o arquivo: {}", e);
            return;
        }
    };

    graph.to_dot("data/result.dot").expect("Failed to write DOT file");
//    graph.print();
}
