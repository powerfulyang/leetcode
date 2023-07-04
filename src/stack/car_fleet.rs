#[allow(dead_code)]
pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut cars: Vec<(i32, f64)> = position
        .into_iter()
        .zip(speed.into_iter())
        .map(|(p, s)| (p, (target - p) as f64 / s as f64))
        .collect();
    cars.sort_by(|a, b| b.0.cmp(&a.0));
    let mut ans = 0;
    let mut cur = 0.0;
    for (_, time) in cars {
        if time > cur {
            ans += 1;
            cur = time;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3)
    }
}
