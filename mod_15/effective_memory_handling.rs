#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }

    // pub fn print(self, &self, &mut self, mut self) {}
    // self will own the value
    // and &self will borrow self
    // mut self and &mut self will borrow mutabily 
    // that means you can change it or mut self
    // they take all of the object and own it
    // unless you specifically return the original type

    pub fn greet(&self) -> String {
        format!("Hi my name is {}", self.name)
    }

    // to update the attributes of self 
    pub fn age_up(&mut self, n: i32) {
        self.age += n
    }

    pub fn dropme(self) {}
}

fn main() {
    let mut p = Person::new("matt".to_string(), 35);
    p.age_up(3);
    let s = p.greet();
    println!("{}", s);
    println!("{:?}", p);

    let a = get_age(&p);
    // if you try to do age_up with takes &mut self
    // will now get an error saying that 
    // "cannot borrow 'p' as mutable because it is also borrowed as immutable"
    // hence you cannot have two pointers ready to change the variable at the
    // same time to prevent data races
    // this happens as long as a exists but it was used after the usage of a 
    // it will work just fine
    // p.age_up(5);
    println!("a = {}", a);


    // p.dropme();

    let s = p.greet();
    println!("{}", s);
}

pub fn get_age(s: &Person) -> &i32 {
    &s.age
}