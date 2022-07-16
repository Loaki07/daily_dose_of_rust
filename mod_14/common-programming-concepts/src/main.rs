#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

// you cannot implement just copy with out the clone
#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn main() {
    let mut x = 34;
    let y = x; // value gets copied here y = 34
    x += 5;
    println!("y = {}, x = {}", y, x);

    let mut p = Person {
        name: "Matt".to_string(),
        age: 35,
    };

    let p2 = p.clone(); // copies the value before the update
    p.name.push_str("Stoodley");

    println!("p = {:#?}, p2 = {:#?}", p, p2);

    let mut pnt = Point::new(3, 4);
    let p2 = pnt; // will hold the copy of pnt before the update
    pnt.x += 3;

    println!("pnt = {:?}, p2 = {:?}", pnt, p2);
}
