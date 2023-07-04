#![allow(dead_code)]
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    let mut res = vec![0; temperatures.len()];
    for i in 0..temperatures.len() {
        while !stack.is_empty() && temperatures[*stack.last().unwrap()] < temperatures[i] {
            let idx = stack.pop().unwrap();
            res[idx] = (i - idx) as i32;
        }
        stack.push(i);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        )
    }
}
