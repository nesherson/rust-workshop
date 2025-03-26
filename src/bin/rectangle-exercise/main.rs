struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    simple_variable_example();
    tuple_example();
    struct_example();
    struct_method_example();
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
    println!("Area of rectangle is: {}\n", rect.area());
}

fn struct_method_example() {
    let rect1 = Rectangle {
        width: 40,
        height: 40
    };
    let rect2 = Rectangle {
        width: 50,
        height: 50
    };
    let rect3 = Rectangle {
        width: 55,
        height: 60
    };

    println!("***Struct method example example**");
    println!("Rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("Rect3 can hold rect2: {}\n", rect3.can_hold(&rect2));
}

fn simple_calculate_area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_calculate_area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}