use std::io;

// Function to read input as f64
fn read_input(prompt: &str) -> f64 {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed");
    input.trim().parse::<f64>().expect("Invalid number")
}

// Formulas
fn area_trapezium() {
    let h = read_input("Enter height:");
    let b1 = read_input("Enter base1:");
    let b2 = read_input("Enter base2:");
    let area = h / 2.0 * (b1 + b2);
    println!("Area of Trapezium = {}", area);
}

fn area_rhombus() {
    let d1 = read_input("Enter diagonal 1:");
    let d2 = read_input("Enter diagonal 2:");
    let area = 0.5 * d1 * d2;
    println!("Area of Rhombus = {}", area);
}

fn area_parallelogram() {
    let base = read_input("Enter base:");
    let altitude = read_input("Enter altitude:");
    let area = base * altitude;
    println!("Area of Parallelogram = {}", area);
}

fn area_cube() {
    let side = read_input("Enter length of the side:");
    let area = 6.0 * side * side;
    println!("Area of Cube = {}", area);
}

fn volume_cylinder() {
    let r = read_input("Enter radius:");
    let h = read_input("Enter height:");
    let volume = std::f64::consts::PI * r * r * h;
    println!("Volume of Cylinder = {}", volume);
}

fn main() {
    println!("Select a calculation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let choice = read_input("Enter option (1–5):") as i32;

    match choice {
        1 => area_trapezium(),
        2 => area_rhombus(),
        3 => area_parallelogram(),
        4 => area_cube(),
        5 => volume_cylinder(),
        _ => println!("Invalid choice"),
    }
}
