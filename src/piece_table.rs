use piece_table::NodeType::{ADDED, ORIGINAL};

#[derive(Debug, Clone)]
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
    pub fn new(original: String) -> PieceTable {
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

    pub fn get_text(&self) -> String {
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

    pub fn get_range_lines(&self, start: usize, stop: usize) -> Vec<String> {
        let lines: Vec<String> = self
            .get_text()
            .split('\n')
            .map(String::from)
            .enumerate()
            .filter(|&(i, _)| i >= start && i < stop)
            .map(|(_, e)| e)
            .collect::<Vec<String>>();

        lines
    }

    pub fn get_all_lines(&self) -> Vec<String> {
        let lines: Vec<String> = self.get_text().split('\n').map(String::from).collect();
        lines
    }

    pub fn get_number_of_lines(&self) -> usize {
        self.get_text().matches('\n').count() as usize + 1
    }

    #[allow(dead_code)]
    pub fn push(mut self, text: String) -> PieceTable {
        let previous_node = self.nodes.iter().last().unwrap();
        let should_update_previous_node = previous_node.node_type == ADDED
            && previous_node.start as usize + previous_node.length == self.added.len();

        if should_update_previous_node {
            let new_node = Node {
                node_type: ADDED,
                start: previous_node.start,
                length: previous_node.length + text.len(),
            };
            self.nodes.splice(
                self.nodes.len() - 1..self.nodes.len(),
                vec![new_node].into_iter(),
            );
        } else {
            let new_node = Node {
                node_type: ADDED,
                start: self.added.len() as u32,
                length: text.len(),
            };
            self.nodes.push(new_node);
        }
        self.added.push_str(&text);

        self
    }

    /*
     *
     * DELETE
     *
     */

    pub fn remove(&mut self, start_index: u32, length: usize) {
        let remove_start_index = start_index as usize;
        let remove_stop_index = remove_start_index + length;
        let mut text_index = 0 as usize;
        self.nodes = self
            .nodes
            .iter_mut()
            .flat_map(|node| {
                let node_start_index = text_index;
                let node_stop_index = text_index + node.length;
                let current_text_index = text_index;
                text_index += node.length;
                if PieceTable::is_deletion_within_the_node(
                    node_start_index,
                    node_stop_index,
                    remove_start_index,
                    remove_stop_index,
                ) {
                    let second_node_start = (remove_stop_index - current_text_index) as u32;
                    return vec![
                        Node {
                            node_type: node.node_type,
                            start: node.start,
                            length: remove_start_index - node_start_index,
                        },
                        Node {
                            node_type: node.node_type,
                            start: second_node_start,
                            length: node.length
                                - (second_node_start as usize - node.start as usize),
                        },
                    ];
                } else if PieceTable::is_node_within_deletion(
                    node_start_index,
                    node_stop_index,
                    remove_start_index,
                    remove_stop_index,
                ) {
                    return vec![];
                } else if PieceTable::is_deletion_at_the_beginning_of_node(
                    node_start_index,
                    node_stop_index,
                    remove_start_index,
                    remove_stop_index,
                ) {
                    let start_diff = remove_stop_index - node_start_index;
                    return vec![Node {
                        node_type: node.node_type,
                        start: node.start + start_diff as u32,
                        length: node.length - start_diff,
                    }];
                } else if PieceTable::is_deletion_at_the_end_of_node(
                    node_start_index,
                    node_stop_index,
                    remove_start_index,
                    remove_stop_index,
                ) {
                    return vec![Node {
                        node_type: node.node_type,
                        start: node.start,
                        length: node.length + remove_start_index - node_stop_index,
                    }];
                } else {
                    vec![*node]
                    // return vec![node.clone()];
                }
            })
            .collect();
    }

    fn is_deletion_within_the_node(
        node_start_index: usize,
        node_stop_index: usize,
        remove_start_index: usize,
        remove_stop_index: usize,
    ) -> bool {
        node_start_index < remove_start_index && node_stop_index > remove_stop_index
    }

    fn is_node_within_deletion(
        node_start_index: usize,
        node_stop_index: usize,
        remove_start_index: usize,
        remove_stop_index: usize,
    ) -> bool {
        node_start_index >= remove_start_index && node_stop_index <= remove_stop_index
    }

    fn is_deletion_at_the_beginning_of_node(
        node_start_index: usize,
        _node_stop_index: usize,
        remove_start_index: usize,
        remove_stop_index: usize,
    ) -> bool {
        node_start_index < remove_stop_index && remove_start_index <= node_start_index
    }

    fn is_deletion_at_the_end_of_node(
        node_start_index: usize,
        node_stop_index: usize,
        remove_start_index: usize,
        _cremove_stop_index: usize,
    ) -> bool {
        node_stop_index > remove_start_index && remove_start_index >= node_start_index
    }

    /*
     *
     * INSERTION
     *
     */

    pub fn insert(&mut self, index: u32, text: String) {
        let add_start_index = self.added.len();
        self.added.push_str(&text);

        let (node_where_it_got_inserted, index_node_where_it_got_inserted, text_index) =
            self.get_node_where_it_got_inserted_and_index(index);

        let new_nodes = self.build_new_nodes(
            index,
            text,
            add_start_index,
            node_where_it_got_inserted,
            text_index,
        );

        self.nodes.splice(
            index_node_where_it_got_inserted..index_node_where_it_got_inserted + 1,
            new_nodes.into_iter().filter(|node| node.length != 0),
        );
    }

    fn build_new_nodes(
        &self,
        index: u32,
        text: String,
        added_length: usize,
        node_where_it_got_inserted: Node,
        text_index: usize,
    ) -> Vec<Node> {
        let length_before_insertion_node = index - text_index as u32;
        let node_before_insertion = Node {
            node_type: node_where_it_got_inserted.node_type,
            length: length_before_insertion_node as usize,
            start: node_where_it_got_inserted.start,
        };
        let new_node = Node {
            node_type: ADDED,
            length: text.len(),
            start: added_length as u32,
        };
        let length_after_insertion =
            node_where_it_got_inserted.length - node_before_insertion.length;
        let node_after_insertion = Node {
            node_type: node_where_it_got_inserted.node_type,
            length: length_after_insertion,
            start: node_before_insertion.start + node_before_insertion.length as u32,
        };
        let is_node_at_the_end_added = node_before_insertion.start as usize
            + node_before_insertion.length
            == new_node.start as usize
            && node_before_insertion.node_type == ADDED;

        if is_node_at_the_end_added {
            vec![
                Node {
                    node_type: node_before_insertion.node_type,
                    start: node_before_insertion.start,
                    length: node_before_insertion.length + new_node.length,
                },
                node_after_insertion,
            ]
        } else {
            vec![node_before_insertion, new_node, node_after_insertion]
        }
    }

    fn get_node_where_it_got_inserted_and_index(&self, index: u32) -> (Node, usize, usize) {
        let mut total_offset = 0;
        let mut index_node_where_it_got_inserted = 0;

        let node_where_it_got_inserted = self
            .nodes
            .iter()
            .find(|node| {
                if (total_offset + node.length) as u32 >= index {
                    return true;
                }
                total_offset += node.length;
                index_node_where_it_got_inserted += 1;
                false
            })
            .unwrap();

        (
            *node_where_it_got_inserted,
            index_node_where_it_got_inserted,
            total_offset,
        )
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
        assert_eq!(text, String::from("This is a text..."));
        assert_eq!(
            vec![
                Node {
                    node_type: ORIGINAL,
                    start: 0,
                    length: 14,
                },
                Node {
                    node_type: ADDED,
                    start: 0,
                    length: 3,
                }
            ],
            piece_table.nodes
        );
    }

    #[test]
    fn remove_when_index_is_length_of_text_should_remove_characters_at_the_end_of_node() {
        // Given
        let input = String::from("This is a text...");
        let mut piece_table = PieceTable::new(input);

        // When
        piece_table.remove(15, 2);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, String::from("This is a text."));
    }

    #[test]
    fn remove_when_index_is_length_of_text_should_remove_characters_at_the_beginning_of_node() {
        // Given
        let input = String::from("xxThis is a text.");
        let mut piece_table = PieceTable::new(input);

        // When
        piece_table.remove(0, 2);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, String::from("This is a text."));
    }

    #[test]
    fn remove_when_index_is_length_of_text_should_remove_characters_all_of_node() {
        // Given
        let input = String::from("This is a text.");
        let mut piece_table = PieceTable::new(input);

        // When
        piece_table.remove(0, 15);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, String::from(""));
    }

    #[test]
    fn remove_when_index_is_length_of_text_should_remove_characters_within_of_node() {
        // Given
        let input = String::from("This isxxxx a text.");
        let mut piece_table = PieceTable::new(input);

        // When
        piece_table.remove(7, 4);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, String::from("This is a text."));
    }

    #[test]
    fn remove_when_index_is_length_of_text_should_remove_characters_within_of_second_node() {
        // Given
        let input = String::from("This is a text.");
        let mut piece_table = PieceTable::new(input);
        let input1 = String::from(" This is a xxtext.");
        piece_table = piece_table.push(input1);

        // When
        piece_table.remove(26, 2);

        // Then
        let text = piece_table.get_text();
        assert_eq!(text, String::from("This is a text. This is a text."));
    }

    #[test]
    fn remove_when_index_is_between_two_nodes() {
        // Given
        let input = String::from("This is a text.");
        let mut piece_table = PieceTable::new(input);
        let input1 = String::from(" This is a text.xx");
        piece_table = piece_table.push(input1);
        let input2 = String::from("xx This is a text.");
        piece_table = piece_table.push(input2);

        // When
        piece_table.remove(31, 4);

        // Then
        let text = piece_table.get_text();
        assert_eq!(
            text,
            String::from("This is a text. This is a text. This is a text.")
        );
    }

    #[test]
    fn insert_should_insert_text_in_the_content_first_node() {
        // Given
        let input = String::from("This is a text");
        let pushed_input = String::from("...");
        let mut piece_table = PieceTable::new(input);
        piece_table = piece_table.push(pushed_input);

        let added_str = String::from("new ");

        // When
        piece_table.insert(10, added_str);

        // Then
        // Explanation of what should be in the list of nodes
        // ORIGINAL NODE: start: 0, length: 10
        // ORIGINAL NODE: start: 11, length: 4
        // ADDED NODE: start: 0, length: 3
        // ADDED NODE: start: 3,length: 4
        let text = piece_table.get_text();
        assert_eq!(text, String::from("This is a new text..."))
    }

    #[test]
    fn insert_should_insert_text_in_the_content_of_second_node() {
        // Given
        let input = String::from("This is a text.");
        let pushed_input = String::from(" This is a second piece.");
        let mut piece_table = PieceTable::new(input);
        piece_table = piece_table.push(pushed_input);

        let added_str = String::from("new ");

        // When
        piece_table.insert(26, added_str);

        // Then
        // Explanation of what should be in the list of nodes
        // ORIGINAL NODE: start: 0, length: 15
        // ADDED NODE: start: 0, length: 11
        // ADDED NODE: start: 24,length: 4
        // ADDED NODE: start: 11, length: 13
        let text = piece_table.get_text();
        assert_eq!(
            text,
            String::from("This is a text. This is a new second piece.")
        )
    }

    #[test]
    fn insert_should_insert_text_in_the_middle_of_nodes() {
        // Given
        let input = String::from("This is a text");
        let pushed_input = String::from("...");
        let mut piece_table = PieceTable::new(input);
        piece_table = piece_table.push(pushed_input);

        let added_str = String::from(" for unit tests");

        // When
        piece_table.insert(14, added_str);

        // Then
        let text = piece_table.get_text();
        assert_eq!(3, piece_table.nodes.len());
        assert_eq!(text, String::from("This is a text for unit tests..."))
    }

    #[test]
    fn insert_should_insert_text_several_times_in_several_nodes() {
        // Given
        let input = String::from("This is a text.");
        let pushed_input = String::from(" This is a second piece.");
        let mut piece_table = PieceTable::new(input);
        piece_table = piece_table.push(pushed_input);

        let added_str = String::from("new ");
        let added_str_2 = String::from("another ");
        let added_str_3 = String::from("n");

        // When
        piece_table.insert(26, added_str);
        piece_table.insert(10, added_str_2);
        piece_table.insert(9, added_str_3);

        // Then
        let text = piece_table.get_text();
        assert_eq!(
            text,
            String::from("This is an another text. This is a new second piece.")
        )
    }

    #[test]
    fn insert_should_insert_text_several_times_in_several_nodes_consecutively() {
        // Given
        let input = String::from("This is a text.");
        let pushed_input = String::from(" This is a second piece.");
        let mut piece_table = PieceTable::new(input);
        piece_table = piece_table.push(pushed_input);

        let added_str = String::from("n");
        let added_str_2 = String::from("e");
        let added_str_3 = String::from("w ");

        // When
        piece_table.insert(26, added_str);
        piece_table.insert(27, added_str_2);
        piece_table.insert(28, added_str_3);

        // Then
        let text = piece_table.get_text();
        assert_eq!(
            text,
            String::from("This is a text. This is a new second piece.")
        );
        assert_eq!(4, piece_table.nodes.len());
    }

    #[test]
    fn should_find_node_where_it_got_inserted_and_its_index() {
        // Given
        let input = String::from("This is a text");
        let push_str = String::from(".");
        let push_str2 = String::from("..");
        let mut piece_table = PieceTable::new(input);
        piece_table = piece_table.push(push_str);
        piece_table = piece_table.push(push_str2);

        let expected_node = piece_table.nodes.get(0).unwrap().clone();

        // When
        let (result, result_index, text_index) =
            piece_table.get_node_where_it_got_inserted_and_index(5);

        // Then
        assert_eq!(0, result_index);
        assert_eq!(expected_node, result);
        assert_eq!(0, text_index);
    }

    #[test]
    fn should_find_node_where_it_got_inserted_and_its_index_even_on_added_nodes() {
        // Given
        let input = String::from("This is a text");
        let push_str = String::from(". And this is another sentence");
        let push_str2 = String::from("...");
        let mut piece_table = PieceTable::new(input);
        piece_table = piece_table.push(push_str);
        piece_table = piece_table.push(push_str2);

        let expected_node = piece_table.nodes.get(1).unwrap().clone();

        // When
        let (result, result_index, text_index) =
            piece_table.get_node_where_it_got_inserted_and_index(20);
        // Then
        assert_eq!(1, result_index);
        assert_eq!(expected_node, result);
        assert_eq!(14, text_index);
    }

    #[test]
    fn should_be_able_to_compare_two_node_tables() {
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

    #[test]
    fn get_number_of_lines_should_return_1_for_simple_text() {
        // Given
        let input = String::from("This is a text");

        // When
        let piece_table = PieceTable::new(input);

        // Then
        let number_of_line = piece_table.get_number_of_lines();
        assert_eq!(number_of_line, 1)
    }

    #[test]
    fn get_number_of_lines_should_return_3_for_simple_text() {
        // Given
        let input = String::from("This is a text\n bluh\n blah");

        // When
        let piece_table = PieceTable::new(input);

        // Then
        let number_of_line = piece_table.get_number_of_lines();
        assert_eq!(number_of_line, 3)
    }
}
