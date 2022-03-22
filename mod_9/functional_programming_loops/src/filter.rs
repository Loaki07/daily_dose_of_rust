pub fn test_filter() {
    let numbers = vec![1, 2, 3, 4];

    let larger_than_two = numbers.into_iter().filter(|&x| x > 2);

    larger_than_two.for_each(|x| println!("{}", x));
}