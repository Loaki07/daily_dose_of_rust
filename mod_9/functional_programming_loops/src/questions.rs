pub fn question_1() {
    // given an array of numbers.
    // replace each number with the
    // product of all the numbers in the array
    // except the number itself without using
    // division

    let mut numbers = vec![1, 2, 3, 4, 5];

    numbers
        .iter_mut()
        .enumerate()
        .map(|(index, value)| *value * 2);

    numbers.iter().for_each(|x| println!("res: {}", x));
}
