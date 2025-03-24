use std::io;

fn main() {
    let mut num_one = 0;
    let mut num_two = 1;
    let mut limit = String::new();

    loop {
        println!("Please enter number: ");

        io::stdin()
            .read_line(&mut limit)
            .expect("Failed to read line.");

        let limit: i32 = match limit
            .trim()
            .parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        if limit == 0 {
            return;
        }

        if limit == 1 {
            println!("{}", num_one);
            return;
        }

        let mut index = 2;

        print!("{} {} ", num_one, num_two);

        while index < limit {
            let temp = num_two;
            num_two = num_one + num_two;
            num_one = temp;

            print!("{} ", num_two);

            index = index + 1;
        }

        break;
    }


}