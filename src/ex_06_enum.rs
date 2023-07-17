#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

pub fn example_enum() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    dbg!(home);
    dbg!(loopback);

    // Some, Option type
    let _some_number = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None;

    println!(
        "value in {}",
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );

    // match with option type
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(six);
    dbg!(none);
}
