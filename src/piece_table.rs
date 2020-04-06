use piece_table::NodeType::{ADDED, ORIGINAL};

#[derive(Debug)]
pub struct PieceTable {
    original: String,
    added: String,
    nodes: Vec<Node>,
}

#[derive(Debug, Copy, Clone)]
pub struct Node {
    node_type: NodeType,
    start: u32,
    length: usize,
}

#[derive(Debug, Copy, Clone)]
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

    fn push(mut self, text: String) -> PieceTable {
        let new_node = Node {
            node_type: ADDED,
            start: self.added.len() as u32,
            length: text.len(),
        };
        self.nodes.push(new_node);
        self.added.push_str(&text);
        self
    }

    fn remove(mut self, start_index: u32, length: usize) -> PieceTable {
        let remove_start_index = start_index as usize;
        let remove_stop_index = remove_start_index + length;
        self.nodes = self
            .nodes
            .into_iter()
            .flat_map(|node| {
                let node_start_index = node.start as usize;
                let node_stop_index = node_start_index + node.length;
                if node_start_index < remove_start_index && node_stop_index > remove_stop_index {
                    let start_diff = remove_stop_index - node_start_index;
                    return vec![
                        Node {
                            node_type: node.node_type,
                            start: node_start_index as u32,
                            length: node.length - start_diff - 1, // not sure about this - 1
                        },
                        Node {
                            node_type: node.node_type,
                            start: remove_stop_index as u32,
                            length: node.length - node_stop_index + remove_start_index + 1, // not sure about this + 1
                        },
                    ];
                } else if node_start_index >= remove_start_index
                    && node_stop_index <= remove_stop_index
                {
                    return vec![];
                } else if node_start_index < remove_stop_index
                    && remove_start_index <= node_start_index
                {
                    let start_diff = remove_stop_index - node_start_index;
                    return vec![Node {
                        node_type: node.node_type,
                        start: node.start + start_diff as u32,
                        length: node.length - start_diff,
                    }];
                } else if node_stop_index > remove_start_index {
                    return vec![Node {
                        node_type: node.node_type,
                        start: node.start,
                        length: node.length - node_stop_index + remove_start_index,
                    }];
                } else {
                    return vec![node];
                }
            })
            .collect();
        self
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
        let piece_table = PieceTable::new(input);

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
        piece_table = piece_table.push(push_str);

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
        piece_table = piece_table.push(push_str);
        piece_table = piece_table.push(push_str2);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, String::from("This is a text..."))
    }

    #[test]
    fn remove_when_index_is_length_of_text_should_remove_characters_at_the_end_of_piece() {
        // Given
        let input = String::from("This is a text...");
        let mut piece_table = PieceTable::new(input);

        // When
        piece_table = piece_table.remove(15, 2);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, String::from("This is a text."));
    }

    #[test]
    fn remove_when_index_is_length_of_text_should_remove_characters_at_the_beginning_of_piece() {
        // Given
        let input = String::from("xxThis is a text.");
        let mut piece_table = PieceTable::new(input);

        // When
        piece_table = piece_table.remove(0, 2);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, String::from("This is a text."));
    }

    #[test]
    fn remove_when_index_is_length_of_text_should_remove_characters_all_of_piece() {
        // Given
        let input = String::from("This is a text.");
        let mut piece_table = PieceTable::new(input);

        // When
        piece_table = piece_table.remove(0, 15);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, String::from(""));
    }

    #[test]
    fn remove_when_index_is_length_of_text_should_remove_characters_within_of_piece() {
        // Given
        let input = String::from("This isxxxx a text.");
        let mut piece_table = PieceTable::new(input);

        // When
        piece_table = piece_table.remove(7, 4);

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
