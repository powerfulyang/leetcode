use rand::Rng;

#[allow(dead_code)]
fn quick_sort_recursive<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);
    quick_sort_recursive(&mut arr[..pivot]);
    quick_sort_recursive(&mut arr[pivot + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot = rand::thread_rng().gen_range(0..arr.len());
    arr.swap(pivot, arr.len() - 1);
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut arr = vec![1, 3, 2, 5, 4];
        quick_sort_recursive(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
