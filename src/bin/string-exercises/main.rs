use::std::io;
/*
Reverse a string
*/
fn main() {
   vowel_exercise();
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