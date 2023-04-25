use crate::trie::trie_node::TrieNode;

#[derive(Debug)]
pub struct TrieStruct {
    root_node: TrieNode,
}

impl TrieStruct {
    // Create a TrieStruct
    pub fn create() -> TrieStruct {
        TrieStruct {
            root_node: TrieNode::new_root(),
        }
    }

    // Insert a string
    pub fn insert(&mut self, string_val: String) {
        // Create a mutable reference to the root node
        let mut current_node = &mut self.root_node;
        // Convert the string to a vector of chars
        let char_list: Vec<char> = string_val.chars().collect();
        // Create a variable to store the last match
        let mut last_match = 0;

        // Loop through the char list
        for letter_counter in 0..char_list.len() {
            // If the current node has a child node with the current char
            if current_node
                .child_nodes
                .contains_key(&char_list[letter_counter])
            {
                // Set the current node to the child node
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[letter_counter])
                    .unwrap();
            } else {
                // If the current node does not have a child node with the current char
                // Set the last match to the current letter counter
                last_match = letter_counter;
                // Break out of the loop
                break;
            }
            // Set the last match to the current letter counter
            last_match = letter_counter + 1;
        }

        // If the last match is the same as the length of the char list
        if last_match == char_list.len() {
            // Set the current node to final
            current_node.is_final = true;
        } else {
            // If the last match is not the same as the length of the char list
            for new_counter in last_match..char_list.len() {
                // Insert the new char into the current node
                current_node.insert_value(char_list[new_counter], false);
                // Set the current node to the new child node
                current_node = current_node
                    .child_nodes
                    .get_mut(&char_list[new_counter])
                    .unwrap();
            }
            // Set the current node to final
            current_node.is_final = true;
        }
    }

    // Find a string
    pub fn find(&mut self, string_val: String) -> bool {
        // Create a mutable reference to the root node
        let mut current_node = &mut self.root_node;
        // Convert the string to a vector of chars
        let char_list: Vec<char> = string_val.chars().collect();

        // Loop through the char list
        for counter in 0..char_list.len() {
            // If the current node does not have a child node with the current char
            if !current_node.child_nodes.contains_key(&char_list[counter]) {
                return false;
            } else {
                // If the current node has a child node with the current char
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
