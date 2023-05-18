use std::collections::HashSet;

// see: https://powerfulyang.com/post/127#find_maximum_xor
#[allow(dead_code)]
fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut max_xor = 0;
    let mut mask = 0;

    for i in (0..=31).rev() {
        mask |= 1 << i;
        let prefixes: HashSet<i32> = nums.iter().map(|num| num & mask).collect();
        let temp_max_xor = max_xor | (1 << i);
        for &prefix in &prefixes {
            if prefixes.contains(&(temp_max_xor ^ prefix)) {
                max_xor = temp_max_xor;
                break;
            }
        }
    }

    max_xor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_maximum_xor() {
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
