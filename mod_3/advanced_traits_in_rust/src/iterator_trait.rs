/// Advanced Traits
/// https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

/// https://doc.rust-lang.org/std/iter/trait.Iterator.html
// pub trait Iterator {
//     /// associtated types are place holders added to your trait
//     type Item;

//     /// methonds use that placeholder
//     fn next(&mut self) -> Option<Self::Item>;
// }

pub struct Counter {}


// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(0) // since its an example reurning 0 as value
//     }
// }

// will throw error for confliction implementations
// impl Iterator for Counter {
//     type Item = u16;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(0) // since its an example reurning 0 as value
//     }
// }

// implementing generics to implement the generic type
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}

impl Iterator<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        Some(0) // since its an example reurning 0 as value
    }
}

impl Iterator<u16> for Counter {
    fn next(&mut self) -> Option<u16> {
        Some(0) // since its an example reurning 0 as value
    }
}

impl Iterator<u128> for Counter {
    fn next(&mut self) -> Option<u128> {
        Some(0) // since its an example reurning 0 as value
    }
}