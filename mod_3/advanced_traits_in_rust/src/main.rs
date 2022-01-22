mod generic_type_paramenters;

use generic_type_paramenters::*;


fn main() {
    println!("advanced traits rust!");
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}