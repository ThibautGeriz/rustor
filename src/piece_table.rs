use piece_table::NodeType::ORIGINAL;

#[derive(Debug)]
pub struct PieceTable {
    original: String,
    added: String,
    nodes: Vec<Node>,
}

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    start: u32,
    length: usize,
}

#[derive(Debug)]
enum NodeType {
    ORIGINAL,
    ADDED,
}

impl PieceTable {
    fn new(original: String) -> PieceTable {
        let original_length = original.len();
        let original_node = Node {
            node_type: ORIGINAL,
            start: 0,
            length: original_length,
        };
        PieceTable {
            original,
            added: String::new(),
            nodes: vec![original_node],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
