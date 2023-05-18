const N_LETTERS: usize = 2;

#[derive(Default, Debug)]
struct TrieNode {
    children: [usize; N_LETTERS],
}

#[derive(Debug)]
struct Trie {
    nodes: Vec<TrieNode>,
}

#[allow(dead_code)]
impl Trie {
    fn new() -> Self {
        Self {
            nodes: vec![TrieNode::default()],
        }
    }

    fn insert(&mut self, num: i32) {
        let mut bm = 1 << 30;
        let mut i = 0;
        while bm > 0 {
            let c = (num & bm != 0) as usize;
            if self.nodes[i].children[c] == 0 {
                self.nodes[i].children[c] = self.nodes.len();
                self.nodes.push(TrieNode::default())
            }
            i = self.nodes[i].children[c];
            bm >>= 1;
        }
    }

    fn find(&self, num: i32) -> i32 {
        let mut bm = 1 << 30;
        let mut rez = 0;
        let mut i = 0;
        while bm > 0 {
            rez <<= 1;
            let mut c = !(num & bm != 0) as usize;
            if self.nodes[i].children[c] == 0 {
                c ^= 1;
            } else {
                rez |= 1;
            }
            i = self.nodes[i].children[c];
            bm >>= 1;
        }
        rez
    }
}

#[allow(dead_code)]
fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut trie = Trie::new();
    nums.iter().for_each(|num| trie.insert(*num));
    nums.into_iter().map(|num| trie.find(num)).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
        assert_eq!(find_maximum_xor(vec![0]), 0);
        assert_eq!(find_maximum_xor(vec![2, 4]), 6);
        assert_eq!(find_maximum_xor(vec![8, 10, 2]), 10);
        assert_eq!(
            find_maximum_xor(vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70]),
            127
        );
    }
}
