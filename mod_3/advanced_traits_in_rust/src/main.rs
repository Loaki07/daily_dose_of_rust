mod generic_type_paramenters;
mod iterator_trait;
mod methods_with_the_same_name;

use generic_type_paramenters::*;
use iterator_trait::*;
use methods_with_the_same_name::*;

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
    // which uses associated functions
    // Human::fly();
    // <Human as Pilot>::fly();
    // <Human as Wizard>::fly();
}
