pub fn test_into_iter() {
    let mylist = vec!(1, 2, 3, 4);

    // into_iter consumes the original vec
    for element in mylist.into_iter() {
        println!("{}", element);
    }

    // will throw an error that value used
    // here after move
    // println!("{}", mylist[0]);
}