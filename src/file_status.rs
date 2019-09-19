/// Manage the status of the data files.
///
/// This module serves two main roles, locating the experimental data files,
/// typically within some sort of complex folder structure and returning
/// information about these files, such as the last modified time.

// use std::path::Path;
use glob::glob;
use std::path;

pub fn list_files(glob_pattern: &String) -> Option<Vec<path::PathBuf>>{
    /// Return a list of files
    let mut file_list = Vec::new();

    // Load the paths into a vector
    for entry in glob(glob_pattern).expect("Failed to read glob pattern") {
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

    #[test]
    fn this_test_will_pass() {
        let test_dir = "../tests/testDirs/**/*.txt";

        assert_eq!(expected_value, value);
    }
}
