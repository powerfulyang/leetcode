use crate::trie::trie_node::TrieNode;

#[derive(Debug)]
pub struct TrieStruct {
    root_node: TrieNode,
}

#[allow(dead_code)]
impl TrieStruct {
    pub fn create() -> TrieStruct {
        TrieStruct {
            root_node: TrieNode::new_root(),
        }
    }

    pub fn insert(&mut self, string_val: String) {
        let mut current_node = &mut self.root_node;
        let char_list: Vec<char> = string_val.chars().collect();
        let mut last_match = 0;

        for letter_counter in 0..char_list.len() {
            if current_node
                .child_nodes
                .contains_key(&char_list[letter_counter])
            {
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[letter_counter])
                    .unwrap();
            } else {
                last_match = letter_counter;
                break;
            }
            last_match = letter_counter + 1;
        }

        if last_match == char_list.len() {
            current_node.is_final = true;
        } else {
            for new_counter in last_match..char_list.len() {
                current_node.insert_value(char_list[new_counter], false);
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[new_counter])
                    .unwrap();
            }
            current_node.is_final = true;
        }
    }

    pub fn find(&mut self, string_val: String) -> bool {
        let mut current_node = &mut self.root_node;
        let char_list: Vec<char> = string_val.chars().collect();

        for counter in 0..char_list.len() {
            if !current_node.child_nodes.contains_key(&char_list[counter]) {
                return false;
            } else {
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[counter])
                    .unwrap();
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_struct() {
        let mut trie_test = TrieStruct::create();

        // Insert Stuff
        trie_test.insert("Test".to_string());
        trie_test.insert("Tea".to_string());
        trie_test.insert("Background".to_string());
        trie_test.insert("Back".to_string());
        trie_test.insert("Brown".to_string());

        // Find Stuff
        assert_eq!(trie_test.find("Test".to_string()), true);
        assert_eq!(trie_test.find("Tea".to_string()), true);
        assert_eq!(trie_test.find("hello".to_string()), false);
    }
}
