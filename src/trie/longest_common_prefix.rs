fn longest_common_prefix(strings: Vec<String>) -> String {
    match strings.is_empty() {
        true => "".to_string(),
        // skip the first string and fold over the rest
        // fold? https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold
        // fold 类似于 reduce
        // first arg is the initial value of the accumulator
        // second arg is a closure that takes the accumulator and the current value and returns the new accumulator
        // the closure is called for each element in the iterator
        // the final value of the accumulator is returned
        _ => strings.iter().skip(1).fold(strings[0].clone(), |acc, x| {
            acc.chars()
                // zip? https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.zip
                // zip takes an iterator and returns an iterator of tuples
                .zip(x.chars())
                // take_while? https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take_while
                // take_while takes a closure and returns an iterator of elements until the closure returns false
                .take_while(|(x, y)| x == y)
                .map(|(x, _)| x)
                // collect? https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
                // collect takes an iterator and returns a collection
                // here we are collecting the chars into a string
                .collect()
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let strings = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        assert_eq!(longest_common_prefix(strings), "fl".to_string());
    }
}
