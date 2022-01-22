mod generic_type_paramenters;
mod iterator_trait;
mod methods_with_the_same_name;
mod super_traits;
mod new_type_pattern;

use generic_type_paramenters::*;
use iterator_trait::*;
use methods_with_the_same_name::*;
use super_traits::*;
use new_type_pattern::*;

fn main() {
    println!("advanced traits rust!");

    // assert_eq!(
    //     Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
    //     Point { x: 3, y: 3 }
    // );

    // assert_eq!(
    //     Millimeters(1000) + (Meters(1)),
    //     Millimeters(2000)
    // );

    // let person = Human;
    // will throw error fro multiple implementations
    // person.fly();

    // you can call the method for multiple implementations
    // by caling it using its trait
    // Pilot::fly(&person);
    // Wizard ::fly(&person);

    // calling the struct by the type
    // Human::fly();
    // <Human as Pilot>::fly();
    // <Human as Wizard>::fly();
 
    // super traits
    // SecondPoint{x: 1, y: 2}.outline_print();

    // new_type_pattern
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
