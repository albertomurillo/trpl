use std::io;

fn mutability() {
    let mut x = 5;
    println!("The value of x is {x}");

    x = 6;
    println!("The value of x is {x}");
}

fn constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");
}

fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

fn shadowing_type() {
    let dots = "...";
    println!("The value of dots is: {dots}");

    let dots = dots.len();
    println!("The value of dots is: {dots}");
}

fn data_types() {
    let guess: i32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {guess}");
}

fn floating_point() {
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32
}

fn numeric_operations() {
    // addition
    let _sum = 5 + 10;

    // substraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncateds = -5 / 3; // Results in -1

    // remainder
    let _remainder = 43 % 5;
}

fn boolean() {
    let _t = true;
    let _f: bool = false; // with explicit type annotation
}

fn character() {
    let _c = 'z';
    let _z: char = 'Z'; // with explicit type annotation
    let _heart_eyed_cat = '😻';
}

fn tuple_destructuring_and_indexing() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is {y}");

    let _five_hundred = tup.0;
    let _six_point_fouir = tup.1;
    let _one = tup.2;
}

fn array() {
    let _a = [1, 2, 3, 4, 5];
    let _months = [
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

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];

    let _a = [3, 3, 3, 3, 3];
    let _a = [3; 5];
}

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of element at index {index} is: {element}");
}
