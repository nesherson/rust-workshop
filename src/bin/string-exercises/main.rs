use::std::io;

fn main() {
    reverse_string();
    vowel_exercise();
}

fn reverse_string() {
    let mut input = String::new();
    let mut reversed_input = String::new();

    println!("Please enter a word: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed_string = input.trim();
    let input_length =  trimmed_string
        .chars()
        .count();
    let mut index = 0;

    while index < input_length {
        reversed_input.push(trimmed_string
            .chars()
            .nth(input_length - index - 1).unwrap());

        index += 1;
    }

    println!("Reversed input: {}", reversed_input);
}

fn vowel_exercise() {
    let mut input = String::new();
    let mut vowel_count = 0;

    println!("Please enter a word: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    for (_, c) in input.chars().enumerate() {
        if is_vowel(c) {
            vowel_count = vowel_count + 1;
        }
    }

    println!("Number of vowels: {}", vowel_count);
}

fn is_vowel(c: char) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    VOWELS.contains(&c)
}