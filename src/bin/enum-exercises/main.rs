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

fn main() {
    println!("The duration of red light is {} seconds", TrafficLight::Red.duration());
    println!("The duration of yellow light is {} seconds", TrafficLight::Yellow.duration());
    println!("The duration of green light is {} seconds", TrafficLight::Green.duration());
}