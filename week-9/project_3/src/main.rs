use std::fs::File;
use std::io::Write;

fn main() {
    // Dataset 1: Names of commissioners
    let commissioners = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbonna","Adewale Jimoh Akanbi","Osazuwa Faith Etieve",];

    // Dataset 2: Ministries
    let ministries = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum",];

    // Dataset 3: Geopolitical Zones
    let zones = vec!["South West","North East","South South","South West","South East",];

    // Display merged table
    println!("{:<5} {:<30} {:<20} {:<15}",
        "S/N", "Commissioner", "Ministry", "Geo-Zone");

    for i in 0..commissioners.len() {
        println!("{:<5} {:<30} {:<20} {:<15}",i + 1,commissioners[i],ministries[i],zones[i]);
    }

    // Save to file
    let mut file = File::create("efcc_merged_records.csv")
        .expect("Could not create file");

    writeln!(file, "S/N,Commissioner,Ministry,Geo-Zone").unwrap();

    for i in 0..commissioners.len() {
        writeln!(file, "{},{},{},{}", i + 1,commissioners[i], ministries[i],zones[i])
        .unwrap();
    }

    println!("\nMerged file saved as efcc_merged_records.csv");
}
