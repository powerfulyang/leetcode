// 676. Implement Magic Dictionary
// https://leetcode.com/problems/implement-magic-dictionary/

use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    chars: HashMap<u8, TrieNode>,
    word: bool,
}

impl TrieNode {
    fn insert(&mut self, s: &str) {
        // 使用 as_bytes() 而不是 chars() ?
        // `as_bytes()` 方法将字符串转换为字节序列，而 `chars()` 方法将字符串转换为字符序列。
        // 因此，如果你的字典树 (`Trie`) 需要支持 Unicode 字符，那么使用 `chars()` 方法可能更合适。
        // 但是，这可能需要改变 `TrieNode` 结构体的定义，将 `HashMap<u8, TrieNode>` 修改为 `HashMap<char, TrieNode>`。
        // 如果你的 `Trie` 只需要处理 ASCII 字符串，那么现在的实现应该就可以满足需求。
        let s = s.as_bytes();
        let mut n = self;

        for b in s.iter().copied() {
            n = n.chars.entry(b).or_default();
        }

        n.word = true;
    }

    // 搜索给定的字词 word 在一个字典树（Trie）中是否存在，并且允许最多 misses 个字符的失配
    // 这个函数适用于实现一些自动纠错或拼写检查的功能
    fn search_with_misses(&self, word: &str, misses: usize) -> bool {
        let w = word.as_bytes();
        // base case. 如果 word 为空，那么只有当当前节点是一个单词节点且 misses 为 0 时，才返回 true
        if w.is_empty() {
            return self.word && misses == 0;
        }

        // Check the happy path first for performance reasons
        if let Some(node) = self.chars.get(&w[0]) {
            if node.search_with_misses(&word[1..], misses) {
                return true;
            }
        }

        if misses > 0 {
            for (&ch, node) in self.chars.iter() {
                if ch == w[0] {
                    continue;
                }

                if node.search_with_misses(&word[1..], misses - 1) {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Default)]
struct MagicDictionary {
    trie: TrieNode,
}

#[allow(dead_code)]
impl MagicDictionary {
    fn new() -> Self {
        Default::default()
    }

    fn build_dict(&mut self, dictionary: Vec<String>) {
        for s in dictionary.iter() {
            self.trie.insert(s.as_str())
        }
    }

    fn search(&self, search_word: String) -> bool {
        self.trie.search_with_misses(&search_word, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut dict = MagicDictionary::new();
        dict.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);
        assert_eq!(dict.search("hello".to_string()), false);
        assert_eq!(dict.search("hhllo".to_string()), true);
        assert_eq!(dict.search("hell".to_string()), false);
    }

    #[test]
    fn test2() {
        let mut dict = MagicDictionary::new();
        dict.build_dict(vec!["hello".to_string(), "hallo".to_string()]);
        assert_eq!(dict.search("hello".to_string()), true);
        assert_eq!(dict.search("hhllo".to_string()), true);
        assert_eq!(dict.search("hell".to_string()), false);
    }
}
