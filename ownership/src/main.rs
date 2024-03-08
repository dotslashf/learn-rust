fn main() {
    let s = String::from("Hello");

    let mut s_mutable = String::from("Hello");
    s_mutable.push_str(" World!");
    println!("{}, {}", s, s_mutable);
    /*
    this line code is resulting error
    reassigning value to s1 is making it being removed from memory, or being moved to s2

    let s1 = String::from("Copy me");
    let s2 = s1;

    println!("{}", s1);
     */

    // this is valid, but takes more memory
    let s1 = String::from("Copy me");
    let s2 = s1.clone();
    println!("s1 {}. s2 {}", s1, s2);

    /*
    if we use int reassigning value does work,
    because integers have known size at compiled time
     */
    let i1 = 5;
    let i2 = i1 * 2;
    println!("i1 {}, i2 {}", i1, i2);

    let s_taken = String::from("Taken!");
    takes_ownership(s_taken);

    /*
    s_taken is not available because its being taken
    by takes_ownership, and the value of it being drop
     */
    // println!("{}", s_taken);

    let x_taken = 5;
    copy_integer(x_taken);
    println!("Here the proof of x_taken still here {}", x_taken);

    let s_given = gives_ownership();
    let s_give = String::from("I give you this"); // not available because its given to s_takes_and_give
    let s_takes_and_give = takes_and_gives_back(s_give);

    println!("{}, {}", s_given, s_takes_and_give);

    let (string_result, length_of_string) = calculate_length(s_given);
    println!("{}, {}", string_result, length_of_string);

    // string result is still exist, because it was being referenced using &
    let length_from_reference = calculate_length_references(&string_result);
    println!(
        "String result still exist: {}, {}",
        string_result, length_from_reference
    );

    /*
     * can't modify value something is not mutable
     * especially when you're borrowing it using &
     */
    let mut hello = String::from("Hello");
    println!("Before change: {}", hello);
    change(&mut hello);
    println!("After change: {}", hello);

    let new_hello = &mut hello;
    /*
    this is invalid, because we can only borrow mut variable once
    let new_hello_two = &mut hello;

    after borrowing from immutable variable
    you can't borrow it as mutable
    example:
    let not_mutable_hello = &hello;
    let new_hello = &mut hello;
     */
    println!("{}", new_hello);

    // this is possible as we used w1 and w2 in println!
    // r1 and r2 is not used anymore
    let mut world = String::from("World");
    let w1 = &world;
    let w2 = &world;

    println!("{} {}", w1, w2);

    let w3 = &mut world;
    println!("{}", w3);
    println!("{}", undangle());
}

fn takes_ownership(s: String) {
    println!("This variable is taken {}", s)
}

fn copy_integer(i: i32) {
    println!("This is not taken, and still available {}", i)
}

fn gives_ownership() -> String {
    let this_string = String::from("For you");
    this_string
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_references(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) -> () {
    s.push_str(" Added!")
}

/*
not a valid function, because we returning a borrowed value
fn dangle() -> &String {
    let s = String::from("Dangle");

    &s
} */

fn undangle() -> String {
    let s = String::from("Undangle!");
    s
}
