// 150. Evaluate Reverse Polish Notation
// https://leetcode.com/problems/evaluate-reverse-polish-notation/

#[allow(dead_code)]
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();
    for s in tokens.iter() {
        if let Ok(x) = s.parse::<i32>() {
            stack.push(x);
        } else {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(match s.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                what => panic!("What's this '{}'", what),
            });
        }
    }
    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            9,
            eval_rpn(vec![
                "2".to_string(),
                String::from("1"),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ])
        );
    }
}
