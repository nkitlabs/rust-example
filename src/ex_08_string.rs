pub fn example_string() {
    // push character or string
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    println!("s1 is {s1}");

    s1.push('l');
    println!("s1 is {s1}");

    // concatenate string.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);
    println!("{}", s2);
    println!("{}", s3);

    // below will error; ownership
    // println!("{}", s1);

    // below will fail; String doesn't support indexing.
    // let s1 = String::from("hello");
    // let h = s1[0];

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");

    // iterating over a string
    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
