use std::collections::HashMap;

#[allow(dead_code)]
pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    for word in words1 {
        *map1.entry(word).or_insert(0) += 1;
    }
    for word in words2 {
        *map2.entry(word).or_insert(0) += 1;
    }
    map1.iter()
        .filter(|(w1, c1)| **c1 == 1 && map2.get(*w1) == Some(&1))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            count_words(
                vec![
                    "leetcode".to_string(),
                    "is".to_string(),
                    "amazing".to_string(),
                    "as".to_string(),
                    "is".to_string(),
                ],
                vec![
                    "leetcode".to_string(),
                    "is".to_string(),
                    "amazing".to_string(),
                ]
            ),
            2
        );
    }
}
