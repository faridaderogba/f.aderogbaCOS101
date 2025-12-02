use std::fs::File;
use std::io::Write;

fn main() {
    // Drink categories
    let lager = [
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stout = [
        "Legend",
        "Turbo King",
        "Williams",
    ];

    let non_alcoholic = [
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
    ];

    // Create / overwrite file
    let mut file = File::create("drinks.txt").expect("Cannot create file");

    // Write categories into file
    writeln!(file, "High-Quality Drink Categories\n").unwrap();

    writeln!(file, "Lager:").unwrap();
    for item in &lager {
        writeln!(file, " - {}", item).unwrap();
    }

    writeln!(file, "\nStout:").unwrap();
    for item in &stout {
        writeln!(file, " - {}", item).unwrap();
    }

    writeln!(file, "\nNon-Alcoholic:").unwrap();
    for item in &non_alcoholic {
        writeln!(file, " - {}", item).unwrap();
    }

    println!("File 'drinks.txt' created successfully!");
}