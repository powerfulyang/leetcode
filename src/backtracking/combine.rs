// Problem: Combinations
// url: https://leetcode.com/problems/combinations/
#[allow(dead_code)]
fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut path = Vec::new();
    backtracking(n, k, 1, &mut path, &mut result);
    result
}

// p2: start point
// p3: path
// p4: result
fn backtracking(p0: i32, p1: i32, p2: i32, p3: &mut Vec<i32>, p4: &mut Vec<Vec<i32>>) {
    // base case
    if p1 == 0 {
        p4.push(p3.clone());
        return;
    }
    // recursive case
    for i in p2..=p0 {
        p3.push(i);
        backtracking(p0, p1 - 1, i + 1, p3, p4);
        p3.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combine() {
        let n = 4;
        let k = 2;
        let result = combine(n, k);
        assert_eq!(result, vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 4]]);
    }
}