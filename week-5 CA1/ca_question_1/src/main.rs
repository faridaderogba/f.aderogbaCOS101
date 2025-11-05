// question 1
use std::io;
fn main()

{

    //Temperature in celsius
    let mut c = String::new();
    println!("Enter temperature");
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c:f32 = c.trim().parse().expect("Failed to read input");
    println!("Temperature in celsius = {}",c);

    //Temperatute in farenheit
    let mut f = String::new();
    let f:f32 = ((9.0/5.0)*c+32.0);
    println!("Temperature in farenheit = {}", f);

    //Temperatute in Kelvin

    let mut k = String::new();
    let k:f32 = (c+273.15);
    println!("Temperature in Kelvin = {}", k);

    if ("Temperature in celsius") > 30.0 {
        println!("Hot temperature");
    } else if ("Temperature in celsius") >= 0.0 && ("Temperature in celsius") >= 30.0 {
        println!("Normal Range");
    } else if ("Temperature in celsius") < 0.0 {
        println!("Hot temperature");
    }
}