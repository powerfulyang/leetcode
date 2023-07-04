#[allow(dead_code)]
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    // [left, right]
    let mut left = 0;
    let mut right = nums.len() - 1;
    // left <= right
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }
}
