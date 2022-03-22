use core::num;
use std::vec;

pub fn question_1() {
    // given an array of numbers.
    // replace each number with the
    // product of all the numbers in the array
    // except the number itself without division
    // https://leetcode.com/problems/product-of-array-except-self/
    // Example 1
    // Input: nums = [1,2,3,4]
    // Output: [24,12,8,6]

    let nums = vec![1, 2, 3, 4];

    let mut result = vec![1; nums.len()];

    (0..nums.len()).fold(1i32, |mut sum, i| {
        result[i] = sum;
        sum *= nums[i];
        sum
    });

    (0..nums.len()).rev().fold(1i32, |mut sum, i| {
        result[i] *= sum;
        sum *= nums[i];
        sum
    });

    result.iter().for_each(|x| println!("res: {}", x));
}
