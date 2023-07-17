fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_ownership_2(some_string: &String) {
    println!("{}", some_string);
}

fn takes_ownership_and_change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

pub fn example_ownership() {
    // The following will fail because s1 transfer ownership of the data to s2
    // it’s known as a move. (not like the concept of shallow copy from the other languages)
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);

    let s = String::from("hello");

    // following will get an error; value borrowed here after move
    // takes_ownership(s);
    // println!("{s}");

    takes_ownership_2(&s);
    println!("{s}");

    let mut s = String::from("hello");
    takes_ownership_and_change(&mut s);
    println!("{s}");

    // x would move into the function, but i32 is Copy, so it's okay to still use x afterward.
    let x = 5;
    makes_copy(x);
}
