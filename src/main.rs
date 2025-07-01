use crate::file_reader::freader;

mod edge;
mod file_reader;
mod graph;
mod node;

fn main() {
    let filename = "data/graph_5.txt"; // Nome do arquivo com os dados do grafo

    let graph = match freader(filename) {
        Ok(graph) => graph,
        Err(e) => {
            eprintln!("Erro ao ler o arquivo: {}", e);
            return;
        }
    };

    graph.print();
}
