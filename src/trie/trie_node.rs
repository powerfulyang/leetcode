use std::collections::HashMap;

#[derive(Debug)]
pub struct TrieNode {
    pub value: Option<char>,
    pub is_final: bool,
    pub child_nodes: HashMap<char, TrieNode>,
}

impl TrieNode {
    // Create new node
    pub fn new(c: char, is_final: bool) -> TrieNode {
        TrieNode {
            value: Option::Some(c),
            is_final,
            child_nodes: HashMap::new(),
        }
    }

    pub fn new_root() -> TrieNode {
        TrieNode {
            value: Option::None,
            is_final: false,
            child_nodes: HashMap::new(),
        }
    }

    pub fn insert_value(&mut self, c: char, is_final: bool) {
        // Insert the new char into the current node
        self.child_nodes.insert(c, TrieNode::new(c, is_final));
    }
}
