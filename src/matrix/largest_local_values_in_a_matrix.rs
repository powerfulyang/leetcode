use std::cmp::max;

#[allow(dead_code)]
pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // [1] first, we compute maximum values across
    //     3-windows in each row
    let max_rows: Vec<Vec<i32>> = grid
        .iter()
        .map(|v| v.windows(3).map(|w| *w.iter().max().unwrap()).collect())
        .collect();

    // [2] second, we compute maximum values across
    //     3-windows in each column;
    max_rows
        .windows(3)
        .map(|w: &[Vec<i32>]|
            // [3] to scan columns in a 3-row slice, we zip 3 iterators...
            w[0].iter()
                .zip(w[1].iter())
                .zip(w[2].iter())
                // [4] ...and compute maximum across each column
                .map(|((a,b),c)| max(max(*a,*b),*c))
                .collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            largest_local(vec![
                vec![9, 9, 8, 1],
                vec![5, 6, 2, 6],
                vec![8, 2, 6, 4],
                vec![6, 2, 2, 2]
            ]),
            vec![vec![9, 9], vec![8, 6]]
        );

        assert_eq!(
            largest_local(vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 2, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
            ]),
            vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]]
        )
    }
}
