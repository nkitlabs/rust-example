#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&mut self, user_preference: Option<ShirtColor>) -> ShirtColor {
        let user_preference = user_preference.unwrap_or_else(|| self.most_stocked());

        self.remove_shirt_from_stock(user_preference);

        user_preference
    }

    fn remove_shirt_from_stock(&mut self, shirt_color: ShirtColor) {
        let selected_shirt_index: usize = self
            .shirts
            .iter()
            .position(|&x| x == shirt_color)
            .expect("cannot find index of the giveaway shirt.");

        self.shirts.remove(selected_shirt_index);
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn example_store() {
    let mut store = Inventory {
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Red,
        ],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn example_closure_iterator() {
    // example closure + iterator
    example_store();

    // mutable closure
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // sort_by_key example
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width * r.height);
    println!("{:?}", list);

    // example map and sum in iterator
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    let sum_v1: i32 = v1.iter().sum();
    assert_eq!(sum_v1, 6);
}
