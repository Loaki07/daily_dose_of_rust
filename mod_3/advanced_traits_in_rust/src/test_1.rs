pub trait Iterator {
    /// associtated types are place holders added to your trait
    type Item;

    /// methonds use that placeholder
    fn next(&mut self) -> Option<Self::Item>;
}

pub strut Number {}

impl Iterator for Number {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0);
    }
}