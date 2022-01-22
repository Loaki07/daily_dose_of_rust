pub trait Pilot {
    // fn fly(); 
    fn fly(&self);
}

pub trait Wizard {
    // fn fly();
    fn fly(&self);
}

pub struct Human;

impl Human {
    // pub fn fly() {
    pub fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    // fn fly() {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    // fn fly() {
    fn fly(&self) {
        println!("Up!");
    }
}

