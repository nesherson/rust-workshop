use std::io;

fn main() {
    let mut input = String::new();

    println!("***Welcome to simple calculator***");
    println!("Enter expression(e.g. 3+5):");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let operator = get_operator(&input);

    let numbers: Vec<f32> = input
        .trim()
        .split(operator)
        .filter_map(|x| x.parse().ok())
        .collect();

    if numbers.iter().count() < 2 {
        println!("Invalid input!");

        return;
    }

    let first_number = numbers[0];
    let second_number = numbers[1];

    let result: f32 = match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' => first_number * second_number,
        '/' => first_number / second_number,
        _ => 0.0,
    };

    println!("Your input {}", result)
}

fn get_operator(text: &str) -> char {
    if text.trim().contains("+") {
        return '+';
    } else if text.trim().contains("-") {
        return '-';
    } else if text.trim().contains("*") {
        return '*';
    } else if text.trim().contains("/") {
        return '/';
    } else {
        return '+';
    }
}
