enum TrafficLight {
    Red,
    Yellow,
    Green
}

impl TrafficLight {
    fn duration(&self) -> u8 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 15,
            TrafficLight::Green => 35
        }
    }
}

enum Shape {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
    Square { side: f32 }
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Rectangle { width, height } => width * height,
            Shape::Circle { radius } => (radius * 3.14).powf(2.0),
            Shape::Square { side} => side * side
        }
    }
}

fn main() {
    traffic_light_example();
    shape_example();
}

fn traffic_light_example() {
    println!("***Traffic Light Example***");
    println!("The duration of red light is {} seconds", TrafficLight::Red.duration());
    println!("The duration of yellow light is {} seconds", TrafficLight::Yellow.duration());
    println!("The duration of green light is {} seconds", TrafficLight::Green.duration());
    println!();
}

fn shape_example() {
    println!("***Shape Example***");
    println!("The area of the rectangle(width: 30, height: 40) is {}", Shape::Rectangle{ width: 30.0, height: 40.0 }.area());
    println!("The area of the circle(radius: 50) is {}", Shape::Circle{ radius: 50.0 }.area());
    println!("The area of the square(side: 22) is {}", Shape::Square{ side: 22.0 }.area());
    println!();
}