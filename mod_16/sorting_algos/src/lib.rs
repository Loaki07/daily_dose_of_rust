mod bubble_sort;
use bubble_sort::*;


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];

        bubble_sort(&mut v);
        assert_eq!(v, [1, 3, 4, 6, 8, 11, 13]);
    }
}
