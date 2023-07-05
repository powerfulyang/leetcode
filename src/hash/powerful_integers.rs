use std::collections::HashSet;

#[allow(dead_code)]
pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
    let mut set = HashSet::new();
    let mut i = 1;
    while i < bound {
        let mut j = 1;
        while i + j <= bound {
            set.insert(i + j);
            if y == 1 {
                break;
            }
            j *= y;
        }
        if x == 1 {
            break;
        }
        i *= x;
    }
    set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sort_array;

    #[test]
    fn test() {
        assert_eq!(
            sort_array(powerful_integers(2, 3, 10)),
            vec![2, 3, 4, 5, 7, 9, 10]
        );
        assert_eq!(
            sort_array(powerful_integers(3, 5, 15)),
            vec![2, 4, 6, 8, 10, 14]
        );
        assert_eq!(sort_array(powerful_integers(2, 1, 10)), vec![2, 3, 5, 9]);
        assert_eq!(sort_array(powerful_integers(1, 2, 10)), vec![2, 3, 5, 9]);
    }
}
