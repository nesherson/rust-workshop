use std::io;
fn main() {
    loop {
        let mut temperature_fahrenheit = String::new();
        let mut repeat_conversion = String::new();

        println!("Please enter temperature in Fahrenheit: ");

        io::stdin()
            .read_line(&mut temperature_fahrenheit)
            .expect("Failed to read line.");

        let temperature_fahrenheit: f32 = match temperature_fahrenheit
            .trim()
            .parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let temperature_celsius = (temperature_fahrenheit - 32.0) * 5.0/9.0;

        println!("The temperature in celsius is {}", temperature_celsius);
        println!("Do you want to do conversion again? (Y/N)");

        io::stdin()
            .read_line(&mut repeat_conversion)
            .expect("Failed to read line.");

        let repeat_conversion: char = repeat_conversion
            .to_lowercase()
            .trim()
            .parse()
            .expect("Failed to parse string.");

        if repeat_conversion == 'n' {
            println!("Exiting program.");
            break;
        }
    }
}