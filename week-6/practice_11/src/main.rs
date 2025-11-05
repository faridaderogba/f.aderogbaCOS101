fn main() {
    // Declare variables
    let a: i32 = 2;
    let b: i32 = 3; 

    let mut result: i32;

    let result = a & b;
    println!("(a & b) => {}", result);

    let result = a | b;
    println!("(a | b) => {}", result);

    let result = a ^ b;
    println!("(a ^ b) => {}", result);

    let result = !b;
    println!("(!b) => {}", result);

    let result = a << b;
    println!("(a << b) => {}", result);

    let result = a >> b;
    println!("(a >> b) => {}", result);
}