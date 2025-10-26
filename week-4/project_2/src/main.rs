use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    // Input experience (yes or no)
    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut experience).expect("Failed to read input");
    let experience = experience.trim().to_lowercase();

    // Input age
    println!("Enter the age of the employee: ");
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age: u32 = age.trim().parse().expect("Please enter a valid number");

    // Determine incentive
    if experience == "yes" || experience == "y" {
        if age >= 40 {
            println!("The annual incentive is: N1,560,000");
        } else if age >= 30 && age < 40 {
            println!("The annual incentive is: N1,480,000");
        } else if age < 28 {
            println!("The monthly incentive is: N1,300,000");
        } else {
            println!("No specific incentive rule for this age group.");
        }
    } else {
        println!("The incentive is: N100,000");
    }
}
