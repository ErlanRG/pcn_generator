use std::collections::HashSet;
use std::fs::{self, create_dir, metadata};
use std::io::{self, stdin, stdout, Write};
use std::path::{Path, PathBuf};

fn create_pcn_case(pcn_path: &Path) -> PathBuf {
    // Ask the user for the PCN name
    println!("Enter the PCN case number: ");
    stdout().flush().expect("Failed to flush stdout.");

    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error reading input: {}", e);
            }
        };

        let pcn_case = input.trim();

        let pcn_case_dir = pcn_path.join(pcn_case);

        if create_dir(&pcn_case_dir).is_ok() {
            println!(
                "Directory created successfully at \"{}\".",
                pcn_case_dir.display()
            );
            return pcn_case_dir;
        } else {
            println!(
                "{} already exist. Failed to create directory.",
                pcn_case_dir.display()
            );
            println!("\nEnter a valid PCN case number: ");
        }
    }
}

fn create_or_find_pcn_directory() -> PathBuf {
    // Get the default documents path
    let documents_path = dirs::document_dir().expect("Could not find documents directory.");

    // Append "PCN" to the path
    let pcn_path = documents_path.join("PCN");

    // Check if the PCN directory exists and create it if necessary
    if metadata(&pcn_path).is_ok() {
        println!("PCN directory found at \"{}\".", pcn_path.display());
    } else {
        print!(
            "PCN directory not found at \"{}\". Create it? (y/n): ",
            pcn_path.display()
        );
        stdout().flush().expect("Failed to flush stdout.");

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("Error reading the input: {}", error);
            }
        }

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => {
                if create_dir(&pcn_path).is_ok() {
                    println!(
                        "PCN directory created successfully at \"{}\".",
                        pcn_path.display()
                    );
                } else {
                    println!("Failed to create PCN directory.");
                    std::process::exit(1);
                }
            }
            _ => {
                println!("Exiting program.");
                std::process::exit(1);
            }
        }
    }

    pcn_path
}

fn create_affected_parts_directory(pcn_case: &Path, pcn_root: &Path) {
    // Ask the user for the affected parts list
    println!("Please enter the affected parts (comma-separated values): ");

    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Error reading the input: {}", error);
        }
    }

    loop {
        let values: Vec<&str> = input.trim().split(',').map(|t| t.trim()).collect();
        let unique_values: HashSet<_> = values.iter().collect();

        if unique_values.len() == values.len() {
            for value in values {
                copy_rename_template(pcn_root, pcn_case, &value).unwrap();
            }

            println!("Files created successfully at \"{}\".", pcn_case.display());
            break;
        } else {
            println!("There are duplicated affected parts. Try again with unique values.");
            input.clear();
            match stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("Error reading the input: {}", error);
                }
            }
        }
    }
}

fn copy_rename_template(root: &Path, destination: &Path, name: &str) -> Result<(), io::Error> {
    let source = root.join("PCN_template.xlsx");
    let mut dest = PathBuf::from(destination);
    dest.push(format!("{}.xlsx", name));

    match fs::copy(&source, &dest) {
        Ok(_) => Ok(()),

        Err(e) => {
            println!("Error copying file: {}", e);
            Err(e)
        }
    }
}

fn press_enter_to_continue() {
    println!("Press enter to exit...");
    let stdin = stdin();
    let _ = stdin.read_line(&mut String::new());
}

fn main() {
    let pcn_root = create_or_find_pcn_directory();
    let pcn_case = create_pcn_case(&pcn_root);
    create_affected_parts_directory(&pcn_case, &pcn_root);
    press_enter_to_continue();
}
