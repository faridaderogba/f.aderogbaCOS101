use std::io;

fn main() {
    // Vector of job titles
    let roles = vec![
        "Intern",
        "Graduate Officer",
        "Associate Lawyer",
        "Senior Lawyer",
        "Principal Lawyer",
    ];

    // Matching APS levels for each role
    let aps_levels = vec![
        "APS 1–2",       // Intern
        "APS 3–4",       // Graduate Officer
        "APS 5–8",       // Associate Lawyer
        "APS 9–12",      // Senior Lawyer
        "APS 13–15",     // Principal Lawyer
    ];

    // Matching required experience ranges (start, end)
    let experience_ranges = vec![
        (0, 1),   // Intern
        (1, 3),   // Graduate Officer
        (5, 8),   // Associate Lawyer
        (8, 12),  // Senior Lawyer
        (12, 20), // Principal Lawyer
    ];

    // ===== GET USER INPUT =====
    let mut title = String::new();
    println!("Enter Staff Job Title:");
    io::stdin().read_line(&mut title).unwrap();
    let title = title.trim();

    let mut exp = String::new();
    println!("Enter Years of Experience:");
    io::stdin().read_line(&mut exp).unwrap();
    let exp: u32 = exp.trim().parse().unwrap();

    // ===== CHECK APS LEVEL =====
    let mut found = false;

    for i in 0..roles.len() {
        if roles[i].eq_ignore_ascii_case(title) {
            let (min_exp, max_exp) = experience_ranges[i];

            if exp >= min_exp && exp <= max_exp {
                println!(
                    "\n{} with {} years experience is classified as **{}**.",
                    title, exp, aps_levels[i]
                );
            } else {
                println!(
                    "\n{} is a valid role, but {} years does not fit the APS experience range ({min_exp}-{max_exp}).",
                    title, exp
                );
            }

            found = true;
            break;
        }
    }

    if !found {
        println!("\nJob title not found in system.");
    }
}
