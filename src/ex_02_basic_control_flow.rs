fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn basic_control_flows() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // below will get an error; condition is not a boolean.
    // if number {
    //     println!("number was three");
    // }

    // can declare variable using if-else statement.
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // return result from loop.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // loop label control.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // conditional loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // using Range.
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // loop items in a collection/array.
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // match
    let dice_roll = 9;
    match dice_roll {
        3 => println!("lucky"),
        7 => println!("super lucky"),
        _ => (),
    }

    let x = Some(5);
    println!("{:?}", plus_one(x));

    // match with multiple options.
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // match range.
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // ignore value (Option type).
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("new setting value: {}", setting_value.unwrap_or(0));

    // extra condition.
    let x = 4;
    match x {
        x if x % 2 == 0 => println!("The number {} is even", x),
        x => println!("The number {} is odd", x),
    }
}
