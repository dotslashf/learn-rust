fn main() {
    let mut user = User {
        active: true,
        email: String::from("Fadhlu@fadhlu.com"),
        sign_in_count: 0,
        username: String::from("fadhlu"),
    };
    user.username = String::from("fadhlu_02");

    let new_user = create_user(String::from("johndoe"), String::from("JohnDoe@email.com"));

    let another_user = User {
        email: String::from("JaneDoe@email.com"),
        ..new_user
    };
    println!("{:#?} {:#?}", user, another_user);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let indonesia = Location(0, 0);

    // reason why we use String instead of &str
    // check https://doc.rust-lang.org/book/ch05-01-defining-structs.html#ownership-of-struct-data

    let rect = Rectangle {
        height: 5,
        width: 10,
    };

    println!("Rectangle area is {}", calculate_rectangle_area(&rect));
    println!("Rectangle properties is {:?}", rect);

    let new_rect = Rectangle {
        height: dbg!(10 * 1),
        width: 5,
    };

    dbg!(&new_rect);

    let another_rect = Rectangle {
        height: 4,
        width: 4,
    };
    println!("Another rect area is : {}", another_rect.area());

    println!("Can rect hold new_rect? {}", rect.can_hold(&new_rect));
    println!(
        "Can new_rect hold another_rect? {}",
        new_rect.can_hold(&another_rect)
    );

    let square = Rectangle::square(4);
    println!("Area of square is {}", square.area());
}

fn create_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}

fn calculate_rectangle_area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Location(i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
