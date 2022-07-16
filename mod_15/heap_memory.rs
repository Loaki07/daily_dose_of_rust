//  A box is a pointer that
// knows when this pointer falls off the stack to free
// that memory on the heap.
// That is basically the difference between a box pointer and
// and just to borrow pointer.
// When you drop the borrow pointer and and the memory
// will remain wherever it was that you've borrowed.

#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,

    // using this will throw the error recursive has infinite size
    // next: Option<LinkedList<T>>,

    // So the fix for this is to make it a
    // pointer and that way if there is nothing,
    // it's only going to take up the size of a
    // data pointer.
    // Which is you 64 on most computers nowadays,
    // and so that is one of the primary values of
    // a box.
    next: Option<Box<LinkedList<T>>>,
}

impl <T:std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    let mut linked_list = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 2,
            next: None,
        })),
    };
    println!("Hello, {:?}", linked_list);

    if let Some(ref mut v) = linked_list.next {
        v.add_up(10);
    }

    println!("Hello, {:?}", linked_list);

    let mut v: Vec<String> = Vec::new(); // default has a capacity of 4
    let mut v: Vec<String> = Vec::with_capacity(100); // will be a slower since it has to capture memory for 100
    v.push("hello".to_string());
    v.push("bye".to_string());

    for i in 0..105 {
        v.push(i.to_string());
    }

    println!("v.len() = {:?}, v.capacity = {:?}", v.len(), v.capacity());


    // str and String types
    let s = "  hello   "; // immutable at stack
    let mut s = "  hello   ".to_string();  // mutable at heap
    let p = s.trim();
    let p = p.to_string();

    s.push_str("bye");

    println!("p == {}", p);

    let fstr = "help me find home";
    let ffstr = string_find_f(fstr);
    println!("ffstr = {}", ffstr);
}

// fn string_find_f(s: &str) -> &str {
fn string_find_f<'a>(s: &'a str) -> &'a str {
    let n = 0;
    for (n, x) in s.char_indices() {
        if x == 'f' {
            return &s[n..];
        }
    }
    s
}

// the lifetime is static,
// that means that it will never change as long as
// the program exists,
fn choose_str(n: i32) -> &'static str {
    match n {
        0 => "hello",
        1 => "bye",
        _ => "other"
    }
}
