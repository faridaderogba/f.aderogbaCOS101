use std::fs::File;
use std::io::Write;

// Structure to hold student details
struct Student {
    name: String,
    matric: String,
    department: String,
    level: u32,
}

fn main() {
    // Vector of students
    let students = vec![
        Student {name: "Oluchi Mordi".to_string(),matric: "ACC10211111".to_string(),department: "Accounting".to_string(),level: 300,},
        Student {name: "Adams Aliyu".to_string(),matric: "ECO10110101".to_string(),department: "Economics".to_string(),level: 100,},
        Student {name: "Shania Bolade".to_string(),matric: "CSC10328828".to_string(),department: "Computer".to_string(),level: 200,},
        Student {name: "Adekunkle Gold".to_string(),matric: "EEE11020200".to_string(),department: "Electrical".to_string(),level: 200,},
        Student {name: "Blanca Edemoh".to_string(),matric: "MEE10202001".to_string(),department: "Mechanical".to_string(),level: 100,},
    ];

    // Display data
    println!("PAU SMIS Student Records:\n");
    println!("{:<20} {:<15} {:<15} {:<5}","Student Name", "Matric Number", "Department", "Level");

    for s in &students {
        println!("{:<20} {:<15} {:<15} {:<5}",s.name, s.matric, s.department, s.level);}

    // Save to CSV file
    let mut file = File::create("students.csv").expect("Cannot create file");
    writeln!(file, "PAU SMIS").unwrap();
    writeln!(file, "Student Name,Matric Number,Department,Level").unwrap();

    for s in &students {
        writeln!(
            file,
            "{},{},{},{}",
            s.name, s.matric, s.department, s.level
        )
        .unwrap();
    }

    println!("\nFile 'students.csv' saved successfully!");
}