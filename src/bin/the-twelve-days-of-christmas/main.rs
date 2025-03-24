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
    let mut value = "";

    if index == 0 {
        value = "first";
    }
    else if index == 1 {
        value =  "second";
    }
    else if index == 2 {
        value =  "third";
    }
    else if index == 3 {
        value =  "fourth";
    }
    else if index == 4 {
        value =  "fifth";
    }
    else if index == 5 {
        value =  "sixth";
    }
    else if index == 6 {
        value =  "seventh";
    }
    else if index == 7 {
        value =  "eight";
    }
    else if index == 8 {
        value =  "ninth";
    }
    else if index == 9 {
        value =  "tenth";
    }
    else if index == 10 {
        value =  "eleventh";
    }
    else if index == 11 {
        value =  "twelfth";
    }

    return value.to_string();
}