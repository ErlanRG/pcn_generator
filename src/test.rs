#[cfg(test)]

mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_create_pcn_dir() {
        let pcn_name = "test_pcn";
        let dir_path = format!("./{}", pcn_name);

        // Test creating a PCN directory that doesn't already exist
        assert!(create_pcn_dir(pcn_name).is_ok());
        assert!(fs::metadata(&dir_path).unwrap().is_dir());

        // Test creating a PCN directory that already exists
        assert!(create_pcn_dir(pcn_name).is_err());
    }

    #[test]
    fn test_create_subdirs() {
        let pcn_name = "test_pcn";
        let dir_path = format!("./{}", pcn_name);

        // Create the PCN directory
        create_pcn_dir(pcn_name).unwrap();

        // Test creating subdirectories that don't already exist
        let dirs = "dir1,dir2,dir3";
        let expected_paths = vec![
            format!("{}/dir1", dir_path),
            format!("{}/dir2", dir_path),
            format!("{}/dir3", dir_path),
        ];
        assert_eq!(create_subdirs(pcn_name, dirs).unwrap(), expected_paths);
        assert!(fs::metadata(&expected_paths[0]).unwrap().is_dir());
        assert!(fs::metadata(&expected_paths[1]).unwrap().is_dir());
        assert!(fs::metadata(&expected_paths[2]).unwrap().is_dir());

        // Test creating subdirectories that already exist
        let dirs = "dir1,dir2,dir3";
        assert_eq!(create_subdirs(pcn_name, dirs).unwrap(), vec![]);
    }
}

