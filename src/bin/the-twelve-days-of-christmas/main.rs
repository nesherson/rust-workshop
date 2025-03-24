fn main() {
    const VERSES: [&str; 12] = ["",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"];

    let mut index = 0;

    while index < VERSES.len() {
        println!("On the {} day of Christmas my true love sent to me", get_name(index));

        if VERSES[index] != "" {
            println!("{},", VERSES[index]);
        }

        println!("And a partridge in a pear tree.\n");

        index = index + 1;
    }
}

fn get_name(index: usize) -> String {
    let mut value = String::new();

    if index == 0 {
        value = "first".to_string();
    }
    else if index == 1 {
        value =  "second".to_string();
    }
    else if index == 2 {
        value =  "third".to_string();
    }
    else if index == 3 {
        value =  "fourth".to_string();
    }
    else if index == 4 {
        value =  "fifth".to_string();
    }
    else if index == 5 {
        value =  "sixth".to_string();
    }
    else if index == 6 {
        value =  "seventh".to_string();
    }
    else if index == 7 {
        value =  "eight".to_string();
    }
    else if index == 8 {
        value =  "ninth".to_string();
    }
    else if index == 9 {
        value =  "tenth".to_string();
    }
    else if index == 10 {
        value =  "eleventh".to_string();
    }
    else if index == 11 {
        value =  "twelfth".to_string();
    }

    return value;
}