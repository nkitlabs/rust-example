use std::fmt::Display;

// below function will be error; missing lifetime specifier
// fn longest_no_lifetime(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// pub fn call_no_lifetime_fn() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest_no_lifetime(string1.as_str(), string2);
//     println!("The longest string is {}", result);
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn _level(&self) -> i32 {
        3
    }

    fn _announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// combination of lifetime and generic type.
fn _longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn example_function_with_lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // below will error; result is different scope from the string2 (most inner scope).
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // lifetime in a struct
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _important_excerpt = ImportantExcerpt {
        part: first_sentence,
    };
}
