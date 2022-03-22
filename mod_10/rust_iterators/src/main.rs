use std::path::Iter;

mod into_iter;
mod iter_and_iter_mut;

fn main() {
    println!("Rust Iterators!");

    // into_iter::test_into_iter();

    iter_and_iter_mut::test_iter_and_iter_mut();
}
