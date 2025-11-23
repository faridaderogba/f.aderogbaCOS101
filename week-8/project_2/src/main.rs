use std::io;

struct Developer {
    name: String,
    years: u32,
}

fn main() {
    let mut developers: Vec<Developer> = Vec::new();

    // Collect number of applicants
    let mut input = String::new();
    println!("Enter number of developers:");
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for _ in 0..n {
        // Get name
        let mut name = String::new();
        println!("Enter developer name:");
        io::stdin().read_line(&mut name).unwrap();

        // Get years of experience
        let mut years = String::new();
        println!("Enter years of experience:");
        io::stdin().read_line(&mut years).unwrap();
        let years: u32 = years.trim().parse().unwrap();

        developers.push(Developer {
            name: name.trim().to_string(),
            years,
        });
    }

    // Find the developer with highest experience
    let mut highest = &developers[0];
    for dev in &developers {
        if dev.years > highest.years {
            highest = dev;
        }
    }

    println!(
        "\nThe developer with the highest experience is: {} ({} years)",
        highest.name, highest.years
    );
}
