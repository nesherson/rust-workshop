use std::io;

fn main() {
    let mut first_number = String::new();
    let mut second_number = String::new();
    let mut operation = String::new();

    println!("***Welcome to simple calculator***");
    println!("Enter first number:");

    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");

    println!("Enter second number:");

    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");

    println!("Enter operation:");

    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    let first_number: u32 = first_number.trim().parse().expect("Wrong value");
    let second_number: u32 = second_number.trim().parse().expect("Wrong value");

    let result: u32;

    let operation = operation.trim();

    if operation.trim() == "+" {
        result = first_number + second_number;
    } else if operation.trim() == "-" {
        result = first_number - second_number;
    } else if operation.trim() == "*" {
        result = first_number * second_number;
    } else if operation.trim() == "/" {
        result = first_number / second_number;
    } else {
        result = 0
    }

    println!("Your input {result}")
}
