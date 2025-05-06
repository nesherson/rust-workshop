fn main() {
    initial_exercise();
}

fn initial_exercise() {
    let vec = vec![1, 2, 3, 4, 5];

    let first_value = &vec[0];
    let second_value = vec.get(1);

    println!("The first element in vector is {first_value}");

    match second_value {
        Some(n) => println!("The second element in the vector is {n}"),
        None => println!("There is no second element in the vector"),
    }
}