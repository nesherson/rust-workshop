fn main() {
    let width = 40;
    let height = 20;

    println!("Area of rectangle is: {}", calculate_area(width, height));
}

fn calculate_area(width: u32, height: u32) -> u32 {
    width * height
}