use std::collections::HashSet;
use std::fs::{self, create_dir, metadata};
use std::io::{self, stdin, stdout, Write};
use std::path::{Path, PathBuf};

fn create_pcn_case(pcn_path: &Path) -> PathBuf {
    // Ask the user for a PCN name
    print!("Enter the PCN name: ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Could not read user input.");

    let pcn_name = input.trim();

    // Create the PCN directory inside the PCN path
    let pcn_dir = pcn_path.join(pcn_name);

    if create_dir(&pcn_dir).is_ok() {
        println!(
            "Directory created successfully at \"{}\".",
            pcn_dir.display()
        );
    } else {
        println!(
            "{} already exist. Failed to create directory.",
            pcn_dir.display()
        );
        std::process::exit(1);
    }

    pcn_dir
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
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Could not read user input.");

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
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    loop {
        let values: Vec<&str> = input.trim().split(',').collect();
        let unique_values: HashSet<_> = values.iter().collect();

        if unique_values.len() == values.len() {
            for value in values {
                let affected_path = pcn_case.join(value);
                if create_dir(&affected_path).is_ok() {
                    println!(
                        "PCN directory created successfully at \"{}\".",
                        affected_path.display()
                    );

                    copy_rename_template(&pcn_root, &affected_path);
                }
            }
            break;
        } else {
            println!("There are duplicated affected parts. Try again with unique values.");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");
        }
    }
}

fn copy_rename_template(root: &Path, destination: &Path) {
    let source_file_path = root.join("PCN_template.xlsx");

    let source_file = Path::new(&source_file_path);
    let new_file_name = destination.file_name().unwrap().to_str().unwrap();

    let new_file_path = PathBuf::from(destination).join(format!(
        "{}.{}",
        new_file_name,
        source_file.extension().unwrap().to_str().unwrap()
    ));

    if !source_file.exists() {
        println!(
            "Warning: Template file not found at \"{}\".",
            root.display()
        );
        return;
    }

    if let Err(e) = fs::copy(source_file, &new_file_path) {
        panic!("Error copying file: {:?}", e);
    }

    println!("Template copied successfully to {:?}", new_file_path);
}

fn main() {
    let pcn_root = create_or_find_pcn_directory();
    let pcn_case = create_pcn_case(&pcn_root);
    create_affected_parts_directory(&pcn_case, &pcn_root);
}
