use std::collections::HashMap;

#[allow(dead_code)]
pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .fold(HashMap::new(), |mut acc, v| {
            *acc.entry(v).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .filter_map(|(k, v)| if v == 1 { Some(k) } else { None })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(sum_of_unique(vec![1, 2, 3, 2]), 4);
    }
}
