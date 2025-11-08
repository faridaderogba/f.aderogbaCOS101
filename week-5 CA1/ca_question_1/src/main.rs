// question 1
use std::io;

fn main() {
    // Temperature in celsius
    let mut c = String::new();
    println!("Enter temperature in Celsius:");
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c: f32 = c.trim().parse().expect("Please enter a valid number");
    println!("Temperature in Celsius = {}", c);

    // Temperature in Fahrenheit
    let f: f32 = (9.0 / 5.0) * c + 32.0;
    println!("Temperature in Fahrenheit = {}", f);

    // Temperature in Kelvin
    let k: f32 = c + 273.15;
    println!("Temperature in Kelvin = {}", k);

    // Temperature range check
    if c > 30.0 {
        println!("Hot temperature");
    } else if c >= 0.0 && c <= 30.0 {
        println!("Normal range");
    } else {
        println!("Cold temperature");
    }
}
