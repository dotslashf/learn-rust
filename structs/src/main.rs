fn main() {
    let mut user = User {
        active: true,
        email: String::from("Fadhlu@fadhlu.com"),
        sign_in_count: 0,
        username: String::from("fadhlu"),
    };
    user.username = String::from("fadhlu_02");

    let mut new_user = create_user(String::from("johndoe"), String::from("JohnDoe@email.com"));

    let another_user = User {
        email: String::from("JaneDoe@email.com"),
        ..new_user
    };

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let indonesia = Location(0, 0);

    // reason why we use String instead of &str
    // check https://doc.rust-lang.org/book/ch05-01-defining-structs.html#ownership-of-struct-data
}

fn create_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Location(i32, i32);
