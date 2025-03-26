struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    simple_variable_example();
    tuple_example();
    struct_example();
}

fn simple_variable_example() {
    let width = 40;
    let height = 20;

    println!("***Simple variable example***");
    println!("Area of rectangle is: {}\n", simple_calculate_area(width, height));
}

fn tuple_example() {
    let rect = (40, 20);

    println!("***Tuple example***");
    println!("Area of rectangle is: {}\n", tuple_calculate_area(rect));
}

fn struct_example() {
    let rect = Rectangle {
        width: 40,
        height: 20
    };

    println!("***Struct example***");
    println!("Area of rectangle is: {}\n", struct_calculate_area(&rect));
}

fn simple_calculate_area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_calculate_area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn struct_calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}