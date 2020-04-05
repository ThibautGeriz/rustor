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

    fn get_text(&self) -> &String {
        let mut text = self.original.clone();
        text.push_str(&self.added);
        return &text;
    }

    fn push(&mut self, text: String) {
        self.added = text;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_init_with_text() {
        // Given
        let input = String::from("This is a text");

        // When
        let piece_table = PieceTable::new(input);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, &String::from("This is a text"))
    }

    #[test]
    fn push_should_add_text_at_end_of_line() {
        // Given
        let input = String::from("This is a text");
        let piece_table = PieceTable::new(input);
        let push_str = String::from(".");

        // When
        piece_table.push(push_str);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, &String::from("This is a text."))
    }
}
