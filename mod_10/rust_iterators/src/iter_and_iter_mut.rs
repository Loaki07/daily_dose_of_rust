pub fn test_iter_and_iter_mut() {
    let mut mylist = vec!(1, 2, 3, 4);

    for element in mylist.iter() {
        println!("{}", *element);
    }

    println!("{}", mylist[0]);

    for element in mylist.iter_mut() {
        *element = *element + 1;
        println!("{}", *element);
    }

    println!("{}", mylist[0]);
}