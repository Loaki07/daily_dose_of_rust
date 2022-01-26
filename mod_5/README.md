# Topics Covered

1. Ownership Model

// ----- Ownership rules -----
// 1. Each value in Rust has a variable that's called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// ----- The Rules of References -----
// 1. At any given time, you can have either one mutable reference
// or any number of immutable references.
// 
// 2. References must always be valid.