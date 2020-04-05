use piece_table::NodeType::{ADDED, ORIGINAL};

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

    fn get_text(&self) -> String {
        let mut text = String::from("");
        for node in &self.nodes {
            let start = node.start as usize;
            let stop = start + node.length;

            match node.node_type {
                ADDED => text.push_str(&self.added[start..stop]),
                ORIGINAL => text.push_str(&self.original[start..stop]),
            }
        }

        text
    }

    fn push(&mut self, text: String) {
        let new_node = Node {
            node_type: ADDED,
            start: self.added.len() as u32,
            length: text.len(),
        };
        self.nodes.push(new_node);
        self.added.push_str(&text);
    }

    fn remove(&mut self, start_index: u32, length: usize) {
        let new_node = Node {
            node_type: ADDED,
            start: start_index + 1,
            length: 2,
        };
        self.nodes.push(new_node);
    }

    //    fn insert(&mut self, index: u32, text: String) {
    //        self.added.push_str(&text);
    //        let new_node = Node {
    //            node_type: ADDED,
    //            start: self.added.len() as u32,
    //            length: text.len()
    //        };
    //        let before_insertion_node = Node {
    //            node_type: ORIGINAL,
    //            start: 0,
    //            length: (index + 1) as usize
    //        };
    //        let after_insertion_node = Node {
    //            node_type: ADDED,
    //            start: 0,
    //            length: (index + 1) as usize
    //        };
    //        self.nodes.push(new_node);
    //    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_init_with_text() {
        // Given
        let input = String::from("This is a text");

        // When
        let mut piece_table = PieceTable::new(input);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, String::from("This is a text"))
    }

    #[test]
    fn push_should_add_text_at_end_of_line() {
        // Given
        let input = String::from("This is a text");
        let mut piece_table = PieceTable::new(input);
        let push_str = String::from(".");

        // When
        piece_table.push(push_str);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, String::from("This is a text."))
    }

    #[test]
    fn push_should_add_text_at_end_of_line_2() {
        // Given
        let input = String::from("This is a text");
        let mut piece_table = PieceTable::new(input);
        let push_str = String::from(".");
        let push_str2 = String::from("..");

        // When
        piece_table.push(push_str);
        piece_table.push(push_str2);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, String::from("This is a text..."))
    }

    #[test]
    fn remove_when_index_is_length_of_text_should_remove_characters_at_the_end() {
        // Given
        let input = String::from("This is a text...");
        let mut piece_table = PieceTable::new(input);

        // When
        piece_table.remove(14, 2);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, String::from("This is a text."));
    }

    //    #[test]
    //    fn insert_should_add_text_in_the_content() {
    //        // Given
    //        let input = String::from("This is a text");
    //        let mut piece_table = PieceTable::new(input);
    //        let push_str = String::from("new ");
    //
    //        // When
    //        piece_table.insert(10, push_str);
    //
    //        // Then
    //        let text = piece_table.get_text();
    //        assert_eq!(text, String::from("This is a new text"))
    //    }
}
