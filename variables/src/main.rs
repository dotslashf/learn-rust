use std::cmp::Ordering;

fn main() {
    let mut x = 5;
    println!("X is {}", x);
    x = 6;
    println!("X new value is {}", x);

    const HOURS: i32 = 60 * 60;
    println!("Hours in seconds {}", HOURS);

    let y = 1;
    let y = y * 2;

    {
        let y = y * 3;
        println!("Scope y is: {}", y)
    }
    println!("Outside y is: {}", y);

    let spaces: &str = "   ";
    let spaces: usize = spaces.len();
    println!("Spaces length: {}", spaces);
    /*
    not allowed to mutate a variable's type
    let mut mutSpaces = "   ";
    mutSpaces = mutSpaces.len();
    */

    let sum = 5 + 10;
    let subtraction = 92.4 - 18.6;
    let product = 8 * 6;
    let quotient = 55.3 / 2.4;
    let truncated = -6 / 2;
    let remainder = 60 % 5;

    println!(
        "sum {}, subtraction {:.2}, product {}, quotient {:.2}, truncated {}, remainder {}",
        sum, subtraction, product, quotient, truncated, remainder
    );

    let t = true;
    let f: bool = false;
    println!("{} {}", t, f);

    let c = 'a';
    let z: char = 'Y';
    let emoji = '😁';
    println!("{} {} {}", c, z, emoji);

    let tup: (i32, u32, &str) = (-16, 32, "Hello");
    // tuple accessed like array
    // println!("{}, {}, {}", tup.0, tup.1, tup.2);
    println!("{:?}", tup);

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("{:?}", months);

    let arr_number = [' '; 5];
    println!("{:?}", arr_number);

    print_hello();
    pointer_at_position("2");
    function_multiple_parameter(5, 'S');

    let x_block = {
        let y = 5;
        (y + 2) * 10
    };
    println!("X block {}", x_block);

    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4")
    } else if number % 3 == 0 {
        println!("Number is divisible by 3")
    } else if number % 2 == 0 {
        println!("Number is divisible by 2")
    } else {
        println!("Number is not divisible by 4, 3, 2")
    }

    let condition = true;
    let one_line_number = if condition { 5 } else { 0 };
    println!("{}", one_line_number);
}

fn print_hello() {
    println!("Hello World");
}

fn pointer_at_position(position: &str) {
    let arr_pointer = [1, 2, 3, 4, 5];
    println!("Enter array position: ");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line!");

    let index: usize = position.trim().parse().expect("Not a number!");
    match index.cmp(&arr_pointer.len()) {
        Ordering::Greater => panic!("Pointer is too large!"),
        Ordering::Equal => panic!("Pointer is too large!"),
        Ordering::Less => (),
    }
    println!("Array at position {} is {}", index, arr_pointer[index]);
}

fn function_multiple_parameter(value: i32, unit: char) {
    println!("Values is {}{}", value, unit);
}
