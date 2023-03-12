#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_create_pcn_directory() {
        let temp_dir = TempDir::new().unwrap();
        let pcn_path = temp_dir.path().join("PCN");
        ensure_directory_exists(&pcn_path);
        assert!(is_directory(&pcn_path));
    }

    #[test]
    fn test_get_pcn_path_on_windows() {
        let user_profile = "C:\\Users\\TestUser";
        env::set_var("USERPROFILE", user_profile);
        assert_eq!(
            get_pcn_path(),
            PathBuf::from(user_profile).join("Documents").join("PCN")
        );
    }

    #[test]
    fn test_get_pcn_path_on_unix() {
        env::remove_var("USERPROFILE");
        assert_eq!(get_pcn_path(), env::current_dir().unwrap().join("PCN"));
    }

    #[test]
    fn test_get_pcn_number() {
        let input = "123\n";
        let expected_output = "123";
        simulate_input(input, || {
            assert_eq!(get_pcn_number(), expected_output);
        });
    }

    #[test]
    fn test_get_affected_part_numbers() {
        let input = "123,456,789\n";
        let expected_output = vec!["123".to_string(), "456".to_string(), "789".to_string()];
        simulate_input(input, || {
            assert_eq!(get_affected_part_numbers(), expected_output);
        });
    }

    #[test]
    fn test_copy_and_rename_file() {
        let temp_dir = TempDir::new().unwrap();
        let src_file_path = temp_dir.path().join("PCN_template.xlsx");
        let dest_dir_path = temp_dir.path().join("123");
        let dest_file_path = dest_dir_path.join("123.xlsx");
        let mut src_file = File::create(&src_file_path).unwrap();
        src_file.write_all(b"test file contents").unwrap();
        copy_and_rename_file(&src_file_path, &dest_file_path).unwrap();
        let dest_file_contents = fs::read_to_string(&dest_file_path).unwrap();
        assert_eq!(dest_file_contents, "test file contents");
    }

    fn simulate_input<T, F: FnOnce() -> T>(input: &str, f: F) -> T {
        let mut stdin = io::stdin();
        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();
        let old_stdin = std::mem::replace(&mut stdin, io::Cursor::new(input));
        let result = f();
        std::mem::replace(&mut stdin, old_stdin);
        result
    }
}

