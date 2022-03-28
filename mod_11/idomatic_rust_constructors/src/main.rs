use idomatic_rust_constructors::{Role, User, Post};

fn main() {
    println!("idomatic rust - constructors");

    let user1 = User::new("testuser123".to_owned()).unwrap_or_default();

    println!("{:#?}", user1);

    let post1 = Post::default();
    println!("{:#?}", post1);

    let post2 = Post::new("testpost123".to_owned());
    println!("{:#?}", post2);
}
