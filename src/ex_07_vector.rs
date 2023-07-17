pub fn example_vector() {
    // declare vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);

    println!("{:?}", v);

    let v = vec![1, 2, 3];
    println!("{:?}", v);

    // get item in vector
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // below return None
    println!("{:?}", v.get(100));

    // iterate over a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}
