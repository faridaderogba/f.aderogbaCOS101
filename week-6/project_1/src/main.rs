use std::io;
fn main() 
{
    println!("Menu:");
    println!("P = Poundo Yam/Edikaiko Soup - ₦3200");
    println!("F = Fried Rice & Chicken - ₦3000");
    println!("A = Amala & Ewedu Soup - ₦2500");
    println!("E = Eba & Egusi Soup - ₦2000");
    println!("W = White Rice & Stew - ₦2500");

    println!("\nEnter the type of food (P, F, A, E, or W):");
    let mut food = String::new();
    io::stdin().read_line(&mut food).expect("Failed to read input");
    let food = food.trim().to_uppercase();

    println!("Enter the quantity:");
    let mut qty = String::new();
    io::stdin().read_line(&mut qty).expect("Failed to read input");
    let qty: i32 = qty.trim().parse().expect("Please enter a number");

    let price = match food.as_str() {
        "P" => 3200,
        "F" => 3000,
        "A" => 2500,
        "E" => 2000,
        "W" => 2500,
        _ => {
            println!("Invalid food selection!");
            return;
        }
    };

    let mut total = price * qty;
    if total > 10_000 {
        total = (total as f64 * 0.95) as i32;
        println!("You get a 5% discount!");
    }

    println!("Total charge: ₦{}", total); 
}