use std::collections::HashSet;

pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    // Create a mutable reference to store the max xor
    let mut max_xor = 0;
    // Create a mutable reference to store the mask
    let mut mask = 0;

    // Loop through the bits
    for i in (0..=31).rev() {
        // Set the mask to the current bit
        mask |= 1 << i;
        // Create a hash set of the prefixes
        let prefixes: HashSet<i32> = nums.iter().map(|num| num & mask).collect();
        // Create a mutable reference to store the temp max xor
        let temp_max_xor = max_xor | (1 << i);
        // Loop through the prefixes
        for &prefix in &prefixes {
            // If the prefixes contains the temp max xor
            if prefixes.contains(&(temp_max_xor ^ prefix)) {
                // Set the max xor to the temp max xor
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
        assert_eq!(find_maximum_xor(vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70]), 127);
    }
}