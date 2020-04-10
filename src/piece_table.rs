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

#[derive(PartialEq, Debug, Copy, Clone)]
enum NodeType {
    ORIGINAL,
    ADDED,
}

impl PartialEq<Node> for Node {
    fn eq(&self, other: &Node) -> bool {
        self.node_type == other.node_type
            && self.start == other.start
            && self.length == other.length
    }
}

impl PartialEq<PieceTable> for PieceTable {
    fn eq(&self, other: &PieceTable) -> bool {
        self.original == other.original && self.added == other.added && self.nodes == other.nodes
    }
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

    fn insert(&mut self, index: u32, text: String) {
        if text.is_empty() {
            return;
        }

        let add_start_index = &self.added.len() - 1;
        &self.added.push_str(&text);

        let (node_where_it_got_inserted, index_node_where_it_got_inserted) =
            self.get_node_where_it_got_inserted_and_index(index);

        let length_before_insertion_node = index - node_where_it_got_inserted.start;

        let node_before_insertion = Node {
            node_type: node_where_it_got_inserted.node_type,
            length: length_before_insertion_node as usize,
            start: node_where_it_got_inserted.start,
        };
        let new_node = Node {
            node_type: ADDED,
            length: text.len(),
            start: add_start_index as u32,
        };
        let length_after_insertion =
            node_where_it_got_inserted.length - node_before_insertion.length;

        let node_after_insertion = Node {
            node_type: node_where_it_got_inserted.node_type,
            length: length_after_insertion,
            start: node_before_insertion.start + node_before_insertion.length as u32,
        };

        let new_nodes = vec![node_before_insertion, new_node, node_after_insertion];

        &self.nodes.splice(
            index_node_where_it_got_inserted..index_node_where_it_got_inserted + 1,
            new_nodes.into_iter(),
        );
    }

    fn get_node_where_it_got_inserted_and_index(&mut self, index: u32) -> (Node, usize) {
        let node_where_it_got_inserted = self
            .nodes
            .clone()
            .into_iter()
            .find(|node| node.start + node.length as u32 > index)
            .unwrap()
            .clone();

        let index_node_where_it_got_inserted = self
            .nodes
            .iter()
            .position(|node| node == &node_where_it_got_inserted)
            .unwrap();

        return (node_where_it_got_inserted, index_node_where_it_got_inserted);
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

    #[test]
    fn insert_should_insert_text_in_the_content() {
        // Given
        let input = String::from("This is a text");
        let pushed_input = String::from("...");
        let mut piece_table = PieceTable::new(input);
        piece_table = piece_table.push(pushed_input);

        let added_str = String::from("new ");

        // When
        piece_table.insert(9, added_str);

        println!("{:?}", piece_table);
        // Then
        let text = piece_table.get_text();
        assert_eq!(text, String::from("This is a new text..."))
    }

    #[test]
    fn should_find_node_where_it_got_inserted_and_its_index() {
        // GIVEN
        let input = String::from("This is a text");
        let push_str = String::from(".");
        let push_str2 = String::from("..");
        let mut piece_table = PieceTable::new(input);
        piece_table = piece_table.push(push_str);
        piece_table = piece_table.push(push_str2);
        let expected_node = &piece_table.nodes.get(0).unwrap().clone();

        // WHEN
        let (result, result_index) = piece_table.get_node_where_it_got_inserted_and_index(5);

        println!("{:?}", piece_table);
        println!("{:?}", result);
        //THEN
        assert_eq!(0, result_index);
        assert_eq!(expected_node, &result);
    }

    #[test]
    fn should_be_able_to_compare_two_piece_tables() {
        // Given
        let x = PieceTable::new(String::from("test"));
        let y = PieceTable::new(String::from("test"));
        let z = PieceTable::new(String::from("test - false"));

        // When
        let result = x == y;
        let result_1 = x == z;
        let result_2 = x.nodes == y.nodes;

        // Then
        assert_eq!(result, true);
        assert_eq!(result_1, false);
        assert_eq!(result_2, true);
    }

    #[test]
    fn should_be_able_to_compare_two_nodes() {
        // Given
        let x = Node {
            node_type: ORIGINAL,
            length: 2,
            start: 5,
        };
        let y = Node {
            node_type: ORIGINAL,
            length: 2,
            start: 5,
        };
        let z = Node {
            node_type: ORIGINAL,
            length: 2,
            start: 7,
        };

        // When
        let result = x == y;
        let result_1 = x == z;

        // Then
        assert_eq!(result, true);
        assert_eq!(result_1, false);
    }
}
