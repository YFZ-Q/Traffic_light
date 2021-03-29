use traffic_light::TrafficInfo;
use traffic_light::TrafficLight;

fn main() {
    let red = TrafficLight::Red;
    println!("Red light is {} seconds.", TrafficLight::time(red));

    let yellow = TrafficLight::Yellow;
    println!("Yellow light is {} seconds.", TrafficLight::time(yellow));

    let green = TrafficLight::Green;
    println!("Green light is {} seconds.", TrafficLight::time(green));
}