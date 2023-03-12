use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {
    let pcn_path = get_pcn_path();
    ensure_directory_exists(&pcn_path);
    let pcn_number = get_pcn_number();
    let pcn_number_path = pcn_path.join(&pcn_number);
    ensure_directory_exists(&pcn_number_path);
    let affected_part_numbers = get_affected_part_numbers();
    for part_number in &affected_part_numbers {
        let part_number_path = pcn_number_path.join(&part_number);
        ensure_directory_exists(&part_number_path);
        copy_and_rename_file(
            &pcn_path.join("PCN_template.xlsx"),
            &part_number_path.join(&part_number).with_extension("xlsx"),
        )
        .expect("Failed to copy and rename file");
    }
}

fn get_pcn_path() -> PathBuf {
    if is_windows() {
        let documents_path =
            env::var("USERPROFILE").expect("USERPROFILE environment variable not found.");
        PathBuf::from(documents_path).join("Documents").join("PCN")
    } else {
        env::current_dir()
            .expect("Failed to get current working directory")
            .join("PCN")
    }
}

fn ensure_directory_exists(path: &Path) {
    if !is_directory(path) {
        create_directory(path);
    }
}

fn get_pcn_number() -> String {
    print!("Please enter the PCN number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn get_affected_part_numbers() -> Vec<String> {
    print!("Please enter a comma-separated list of affected part numbers: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

fn is_windows() -> bool {
    std::env::consts::OS == "windows"
}

fn is_directory(path: &Path) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

fn create_directory(path: &Path) {
    fs::create_dir(path).expect("Failed to create directory");
}

fn copy_and_rename_file(src: &Path, dest: &Path) -> io::Result<()> {
    let mut contents = fs::read(src)?;
    let parent_dir_name = dest
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let dest_with_name = dest.with_file_name(parent_dir_name).with_extension("xlsx");
    contents.shrink_to_fit();
    fs::write(&dest_with_name, contents)?;
    Ok(())
}

