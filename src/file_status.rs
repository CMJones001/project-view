/// Manage the status of the data files.
///
/// This module serves two main roles, locating the experimental data files,
/// typically within some sort of complex folder structure and returning
/// information about these files, such as the last modified time.
extern crate chrono;

use glob::glob;
use std::path::{PathBuf,Path};
use chrono::Local;
use chrono::Datelike;


/// Return a list of files matching a simple regex in a given directory,
/// typically matching some sort of data file. We only return paths to actual
/// files, directories are not included.
pub fn list_files_in_dir(dir_path: PathBuf, glob_pattern: &str)
                         -> Option<Vec<ExperimentFile>>{
    let mut file_list = Vec::new();
    let mut possible_file;

    // Combine the directory and glob pattern into a pathlike string
    let regex_path = dir_path.join(glob_pattern);

    for entry in glob(regex_path.to_str()?).expect("Failed to read glob pattern") {
        // We must account for glob errors
        possible_file = entry.unwrap();
        // Only link to files, not dirs
        if possible_file.is_file() {
            file_list.push(ExperimentFile::new(possible_file));
        }
    }

    let n_elements = file_list.len();
    if n_elements == 0 {
        None
    } else {
        Some(file_list)
    }
}

/// The experiment dirs are typically defined by the files they contain.
/// Here we create a list of parent directories and then return only the unique
/// values.
pub fn get_unique_parent_dirs(file_list: Vec<ExperimentFile>) ->
    Option<Vec<PathBuf>> {
    let mut parent_dirs = vec![];
    let mut parent_dir;

    // Fail if no dirs are provided
    if file_list.len() == 0 {return None;}

    for file_ in file_list {
        // Get the first parent directory by removing the file name
        parent_dir = PathBuf::from(file_.path.parent()?);

        // Include only the unique vals
        if !parent_dirs.contains(&parent_dir) {
            parent_dirs.push(parent_dir);
        }
    }

    Some(parent_dirs)
}

/// Container for each experiment file, this should contain a list to a valid
/// file and useful metadata of the file.
pub struct ExperimentFile {
    pub path: PathBuf,
    pub modified: chrono::DateTime::<Local>,
}

impl ExperimentFile {
    /// Create the object by providing a file name
    pub fn new(path: PathBuf) -> ExperimentFile {
        if !path.is_file() {
            panic!("ExperimentFile has been provided path to non-existent file");
        }

        // Calculate the last modified time
        let metadata = path.metadata().expect("failed to get metadata");
        let modified_system = metadata.modified().expect("Unable to get file creation time");
        let modified = chrono::DateTime::<Local>::from(modified_system);

        ExperimentFile{ path: path, modified: modified}
    }

    /// Return nicely formatted date time string
    pub fn formatted_time(&self) -> std::string::String {
        let format_string = "%c";
        // std::string::ToString(self.modified.format(format_string))
        self.modified.format(format_string).to_string()
    }
    // Add an ordering to the struct
}


#[cfg(test)]
mod tests {
    use super::*;
    use filetime;
    use tempfile;
    use std::fs;
    /// Return a path to a set of example data directories within the tests
    /// directory.
    fn get_mock_dir() -> PathBuf {
        // Give relative to this source code file
        let mut test_project_dirs = PathBuf::from(file!());

        // Get the full path, remove up to project root and decend down into the
        // tests dir.
        test_project_dirs = test_project_dirs.canonicalize().unwrap();
        test_project_dirs.pop();
        test_project_dirs.pop();
        test_project_dirs.push("tests/testDirs");

        test_project_dirs
    }

    /// Create a temporary directory to hold a mock experiment directory
    /// This is cleared after the tests are run
    #[test]
    fn test_get_mock_dir() {
        // Create the directory and store the path
        let experiment_dir = tempfile::TempDir::new().unwrap();
        let dir_path = &experiment_dir.path(); 

        // Where we expect a sub directory to be created
        let expected_path = dir_path.join("first_dir");

        // Try creating this directory
        let sub_dir = fs::create_dir(dir_path.join("first_dir"))
            .expect("Unable to create sub directory in test dir");

        assert!(expected_path.exists());
    }

    // Tests that the file only returns the expected extension
    #[test]
    fn regex_search_correct_extension() {
        // Create the directory and store the path
        let experiment_dir = tempfile::TempDir::new().unwrap();
        let dir_path = &experiment_dir.path(); 

        let test_extension = "**/*.csv";
        let mut sub_file_path;

        // Create 4 csv files
        for i in 0..4 {
            sub_file_path = dir_path.join(format!("experiment{}.csv", i));
            fs::File::create(sub_file_path).unwrap();
        }

        // Create two txt files
        for i in 0..2 {
            sub_file_path = dir_path.join(format!("experiment{}.txt", i));
            fs::File::create(sub_file_path).unwrap();
        }

        let actual_paths = list_files_in_dir(dir_path.to_path_buf(),
                                             &test_extension)
            .expect("No files matched");

        for p in actual_paths {
            assert_eq!("csv", p.path.extension().unwrap())
        }
    }

    // Ensure that we match the expected number of files
    #[test]
    fn regex_search_correct_count() {
        let experiment_dir = tempfile::TempDir::new().unwrap();
        let dir_path = &experiment_dir.path(); 

        let test_extension = "**/*.csv";
        let mut sub_file_path;

        // Create 4 csv files
        for i in 0..4 {
            sub_file_path = dir_path.join(format!("experiment{}.csv", i));
            fs::File::create(sub_file_path).unwrap();
        }

        // Create two txt files
        for i in 0..2 {
            sub_file_path = dir_path.join(format!("experiment{}.txt", i));
            fs::File::create(sub_file_path).unwrap();
        }

        let actual_paths = list_files_in_dir(dir_path.to_path_buf(),
                                             &test_extension)
            .expect("No files matched");

        assert_eq!(4, actual_paths.len());
    }

    // Ensure that we return the expected number of non-empty directories
    #[test]
    fn return_unqiue_vals() {
        let experiment_dir = tempfile::TempDir::new().unwrap();
        let dir_path = &experiment_dir.path(); 

        let test_extension = "**/*.csv";
        let mut sub_dir_path;
        let mut sub_file_path;

        let n_unique_expected = 2;

        for j in 0..n_unique_expected {
            sub_dir_path = dir_path.join(format!("sub_dir_{}", j));
            fs::create_dir(&sub_dir_path);
            
            // Create 4 csv files
            for i in 0..4 {
                sub_file_path = sub_dir_path.join(format!("experiment{}.csv", i));
                fs::File::create(&sub_file_path).unwrap();
            }

            // Create two txt files
            for i in 0..2 {
                sub_file_path = sub_dir_path.join(format!("experiment{}.txt", i));
                fs::File::create(&sub_file_path).unwrap();
            };
        }

        let actual_paths = list_files_in_dir(dir_path.to_path_buf(),
                                             &test_extension)
            .expect("No files matching '.csv' found");

        let unique_dirs = get_unique_parent_dirs(actual_paths).unwrap();

        assert_eq!(unique_dirs.len(), n_unique_expected)

    }

    // Test that we get the creation time of the file with ExperimentFile
    #[test]
    #[ignore]
    fn get_creation_time() {
        let test_file = get_mock_dir().join("first_dir/data.txt");

        let experiment_file = ExperimentFile::new(test_file);
        let modified_time = experiment_file.modified;

        assert_eq!(modified_time.year(), 2019);
        assert_eq!(modified_time.month(), 9);
        assert_eq!(modified_time.day(), 19);
    }

    // Formatting of a date into a human readable format
    #[test]
    #[ignore]
    fn format_date() {
        let expected_formatted_time = "Thu Sep 19 22:16:28 2019";

        let test_file = get_mock_dir().join("first_dir/data.txt");
        let experiment_file = ExperimentFile::new(test_file);
        let actual_formatted_time = experiment_file.formatted_time();

        assert_eq!(actual_formatted_time, expected_formatted_time)
    }
}
