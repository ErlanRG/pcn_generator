use std::fs;

fn main() {
    // Ask the user for a PCN name
    println!("Enter a PCN name:");
    let mut pcn_name = String::new();
    std::io::stdin().read_line(&mut pcn_name).unwrap();

    // Remove any whitespace or newline characters from the input
    pcn_name = pcn_name.trim().to_string();

    // Create the PCN directory if it doesn't already exist
    if let Err(_) = fs::create_dir(&pcn_name) {
        println!("Warning: Directory '{}' already exists", pcn_name);
    } else {
        println!("Directory '{}' created successfully", pcn_name);
    }

    // Ask the user for comma-separated directory names
    println!("Enter directory names (comma-separated):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Remove any whitespace or newline characters from the input
    input = input.trim().to_string();

    // Split the input into individual directory names
    let dirs: Vec<&str> = input.split(',').map(|s| s.trim()).collect();

    // Create each directory inside the PCN directory
    for dir in dirs {
        let path = format!("{}/{}", pcn_name, dir);
        if let Err(_) = fs::create_dir(&path) {
            println!("Warning: Directory '{}' already exists", path);
        } else {
            println!("Directory '{}' created successfully", path);
        }
    }
}
