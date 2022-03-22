pub fn test_fold() {
    let numbers = vec![1, 2, 3, 4];

    let sum: i32 = numbers.iter().fold(0, |sum, val| sum + val);

    println!("Sum: {}", sum);
}