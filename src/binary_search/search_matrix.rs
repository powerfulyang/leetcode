#![allow(dead_code)]
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let rows = matrix.len();
    let columns = matrix[0].len();

    let mut start = 0;
    let mut end = rows * columns - 1;
    while start <= end {
        let mid = start + (end - start) / 2;
        let mid_value = matrix[mid / columns][mid % columns];
        if mid_value == target {
            return true;
        } else if mid_value < target {
            start = mid + 1;
        } else {
            end = mid - 1;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            ),
            true
        );
    }
}
