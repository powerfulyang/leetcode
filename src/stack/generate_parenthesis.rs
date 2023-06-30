// 22. Generate Parentheses
// https://leetcode.com/problems/generate-parentheses/
// basic idea: backtracking

fn backtracking(res: &mut Vec<String>, s: String, left: i32, right: i32, n: i32) {
    // base case
    if left == n && right == n {
        res.push(s);
        return;
    }
    // recursive case
    if left < n {
        backtracking(res, s.clone() + "(", left + 1, right, n);
    }
    // recursive case
    if right < left {
        backtracking(res, s + ")", left, right + 1, n);
    }
}

#[allow(dead_code)]
fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = Vec::new();
    backtracking(&mut res, String::new(), 0, 0, n);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            vec![
                "((()))".to_string(),
                "(()())".to_string(),
                "(())()".to_string(),
                "()(())".to_string(),
                "()()()".to_string(),
            ],
            generate_parenthesis(3)
        );
    }
}
