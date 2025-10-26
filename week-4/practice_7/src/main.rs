use std::io;

fn main() {
    println!("Enter a number");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut num: i32 = input.trim().parse().expect("Please to input");

    while num < 10 {
        
        println!("insider loop number value is {}", num);
        num += 1;
    }
    println!("Outside loop number value is {}", num);
}