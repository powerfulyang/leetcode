// 20. Valid Parentheses
// https://leetcode.com/problems/valid-parentheses/

#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    if s.len() % 2 == 1 {
        return false;
    }

    let mut v = Vec::with_capacity(s.len());

    s.chars().for_each(|c| {
        match c {
            '(' | '[' | '{' => v.push(c),
            _ => match v.pop() {
                Some('(') if c == ')' => (),
                Some('[') if c == ']' => (),
                Some('{') if c == '}' => (),
                _ => {
                    v.push(' '); // add an invalid char to make the final check false
                }
            },
        }
    });

    v.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_valid("()".to_string()), true);
        assert_eq!(is_valid("()[]{}".to_string()), true);
        assert_eq!(is_valid("(]".to_string()), false);
    }
}
