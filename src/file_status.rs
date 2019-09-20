/// Manage the status of the data files.
///
/// This module serves two main roles, locating the experimental data files,
/// typically within some sort of complex folder structure and returning
/// information about these files, such as the last modified time.

// use std::path::Path;
use glob::glob;
use std::path;

/// Return a list of files matching a simple regex, typically matching some sort
/// of data file. This will be extended to split the glob pattern into a short
/// wildcard and a path to a directory of interest.
pub fn list_files_in_dir(dir_path: path::PathBuf, glob_pattern: &String)
                         -> Option<Vec<path::PathBuf>>{
    let mut file_list = Vec::new();

    // Load the paths into a vector
    let regex_path = dir_path.join(glob_pattern);

    for entry in glob(regex_path.to_str().unwrap()).expect("Failed to read glob pattern") {
        file_list.push(entry.unwrap());
    }

    let n_elements = file_list.len();
    if n_elements == 0 {
        None
    } else {
        Some(file_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Return a path to a set of example data directories within the tests
    /// directory.
    fn get_mock_dir() -> path::PathBuf {
        // Give relative to this source code file
        let mut test_project_dirs = path::PathBuf::from(file!());

        // Get the full path, remove up to project root and decend down into the
        // tests dir.
        test_project_dirs = test_project_dirs.canonicalize().unwrap();
        test_project_dirs.pop();
        test_project_dirs.pop();
        test_project_dirs.push("tests/testDirs");

        test_project_dirs
    }

    // Test that we get the correct mock directory by looking for a given folder.
    #[test]
    fn test_get_mock_dir() {
        let test_dir = get_mock_dir();
        let sub_dir = test_dir.join(path::Path::new("first_dir"));

        assert!(sub_dir.exists());
    }

    // Tests that the file only returns the expected extension
    #[test]
    fn regex_search_correct_extension() {
        let test_dir = get_mock_dir();
        let test_extension = String::from("**/*.csv");

        let actual_paths = list_files_in_dir(test_dir, &test_extension)
            .expect("No files matched");

        for p in actual_paths {
            assert_eq!("csv", p.extension().unwrap())
        }
    }

    // Ensure that we match the expected number of files
    #[test]
    fn regex_search_correct_count() {
        let test_dir = get_mock_dir();
        let test_extension = String::from("**/*.csv");

        let actual_paths = list_files_in_dir(test_dir, &test_extension)
            .expect("No files matched");

        assert_eq!(3, actual_paths.len());

    }
}
