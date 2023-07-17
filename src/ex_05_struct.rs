struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

pub fn example_struct() {
    let _user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // init shorthand
    let user2: User = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // Struct Update Syntax
    let user3 = User {
        email: String::from("someone2@example.com"),
        ..user2
    };

    println!("{}", user2.email);
    println!("{}", user3.email);

    // struct tuple
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // method and functions
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    dbg!(3 * 20);
}
