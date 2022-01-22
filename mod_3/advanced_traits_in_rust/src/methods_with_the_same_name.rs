pub trait Pilot {
    // associated functions
    // fn fly(); 

    fn fly(&self);
}

pub trait Wizard {
    // associated functions
    // fn fly();
    fn fly(&self);
}

pub struct Human;

impl Human {
    // associated functions
    // pub fn fly() {

    pub fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    // associated functions
    // fn fly() {

    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    // associated functions
    // fn fly() {
    fn fly(&self) {
        println!("Up!");
    }
}

