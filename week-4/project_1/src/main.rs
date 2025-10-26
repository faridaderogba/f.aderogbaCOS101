use std::io;
// This program calculates the roots of a quadratic equation ax^2 + bx + c = 0

fn main() {
    // Input a, b, and c
    let mut input = String::new();
    println!("Enter values of a, b and c (separated by spaces):");

    io::stdin().read_line(&mut input).expect("Failed to read input");

    let values: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Please enter valid numbers"))
        .collect();

    let (a, b, c) = (values[0], values[1], values[2]);

    // Discriminant
    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {:.2} and {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: {:.2}", root);
    } else {
        println!("No real roots (complex roots).");
    }
}
