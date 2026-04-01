use std::collections::HashMap;

fn main() {
    // initial_exercise();
    median_and_mode();
}

// fn initial_exercise() {
//     let vec = vec![1, 2, 3, 4, 5];

//     let first_value = &vec[0];
//     let second_value = vec.get(1);

//     println!("The first element in vector is {first_value}");

//     match second_value {
//         Some(n) => println!("The second element in the vector is {n}"),
//         None => println!("There is no second element in the vector"),
//     }
// }

fn median_and_mode() {
    println!("*** Median and mode exercise ***");

    let mut numbers = vec![1, 2, 16, 6, 3, 2];

    numbers.sort();

    let median = get_median(&numbers);
    let mode = get_mode(&numbers);

    println!("The list of numbers: ");
    print(&numbers);
    match median {
        Some(value) => {
            println!("\nThe median of these numbers is: {}", value);
        }
        None => {
            println!("No median");
        }
    }
    match mode {
        Some(value) => {
            println!("The mode of these numbers is: {}", value);
        }
        None => {
            println!("No mode");
        }
    }
}

fn print(list_of_integers: &Vec<i32>) {
    for integer in list_of_integers {
        print!("{} ", integer)
    }
}

fn get_median(numbers: &Vec<i32>) -> Option<f32> {
    if numbers.len() == 0 {
        return None;
    }

    let length = numbers.len();

    if length % 2 == 0 {
        let halved_length = length / 2;
        let average = (numbers[halved_length] + numbers[halved_length - 1]) as f32 / 2.0;

        return Some(average);
    }

    Some(numbers[length / 2] as f32)
}

fn get_mode(numbers: &Vec<i32>) -> Option<i32> {
    let mut number_occurrence: HashMap<i32, i32> = HashMap::new();

    for &number in numbers {
        let occurence = number_occurrence.entry(number).or_insert(0);
        *occurence += 1;
    }

    match number_occurrence.iter().max_by(|x, y| x.1.cmp(y.1)) {
        Some(occurence) => Some(*occurence.0),
        None => None,
    }
}
