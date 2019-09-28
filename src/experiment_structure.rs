/// Grouping and organisation of the experiment files
///
/// # Outline
///
/// We expect a directory that holds all of the steps in the pipeline for a single
/// experiment; for instance we may have: 1) data extracted from images; 2)
/// filtering and aggregation of this data 3) plots created from the previous
/// step. The output for each step of the analysis should be organised into
/// sub-directories.
///
/// Each of these steps would correspond to a ExperimentPart, grouped together
/// into a single Experiment.
///
/// Typically a new experiment directory may use different starting data or a
/// different analysis step, for which a separate Experiment struct is created.
///
/// ## Experiment
///
/// Container for all of the steps in the pipeline for one experiment.
/// Many of these may be created to allow
///
/// # Future work
/// Later we expect to load these values from a configuration files but for now
/// the values are hard coded.

use std::path::{PathBuf};
use crate::file_status as fs;

/// This contains a single step on pipeline. This will likely contain a list of
/// expected output files. However, if these don't exist then we should provide
/// a means of creating these files.
///
/// For now this prints summary information about the output files, whether they
/// are up to date and how this compares against the source code.
///
pub struct ExperimentPart {
    name: String,
    pub file_list: Vec<fs::ExperimentFile>,
    n_files: usize,
}

impl ExperimentPart {
    /// Create the object by providing a subDir, glob and name
    pub fn new(name: String, dir: PathBuf, glob_pattern: String)
                   -> ExperimentPart {

        // Get all matching files, returning an empty list if this fails
        // While it would be possible to keep this as an option, it makes it
        // everything else more awkward.
        let mut file_list = fs::list_files_in_dir(dir, &glob_pattern)
            .unwrap_or_default();
        let n_files = file_list.len();

        // Sort on the modification date
        // Newest files are first
        file_list.sort_by(|a, b| b.modified.cmp(&a.modified));
        ExperimentPart { name, file_list, n_files}
    }

    /// Print information about the number and age of files in the Part
    pub fn create_summary(&self) -> String {
        // Exit early if no results found
        if &self.n_files == &0 {
            let summary = format!(
                "No files found in {}.",
                &self.name
            );
            return summary;
        }
        // Spaces used to indent secondary lines
        let indent = "    ";

        let summary = format!(
            "{} contains {} file",
            &self.name,
            &self.n_files,
        );

        let newest_summary = format!(
            "\n{}Newest file modified at {}",
            indent,
            &self.get_newest_file().formatted_time()
        );

        let mut oldest_summary = String::from("");
        if &self.n_files > &1 {
            oldest_summary += &format!(
                "\n{}Oldest file modified at {}",
                indent,
                &self.get_oldest_file().formatted_time()
            );
        };

        summary + &newest_summary + &oldest_summary
    }

    fn get_newest_file(&self) -> &fs::ExperimentFile {
        &self.file_list[0]
    }

    fn get_oldest_file(&self) -> &fs::ExperimentFile {
        &self.file_list.last().unwrap()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use filetime;
    use tempfile;
    use std::fs;
    use chrono::{TimeZone};

    // Create a temporary file with a given name at a given day in Sep 2019.
    fn create_file_at_hour(file_path: &PathBuf, day: u32) {
        fs::File::create(&file_path).expect("Unable to create date test file.");

        // Set the modification time on the file
        let creation_time = chrono::Local.ymd(2019, 9, day).and_hms(12, 00, 00);
        let creation_time_stamp = creation_time.timestamp();
        filetime::set_file_mtime(&file_path,
                                 filetime::FileTime::from_unix_time(creation_time_stamp, 0))
            .expect("Unable to set time stamp on file");
    }

    // Generate an experiment part with a few sorted files
    fn experiment_part_with_sorted_files(glob: &str) -> ExperimentPart {
        let experiment_dir = tempfile::TempDir::new().unwrap();
        let dir_path = &experiment_dir.path();

        let test_days = vec![4, 2, 6, 3, 5, 3, 4];

        // Create the relevant .txt files
        for i in 0..4 {
            let file_path = dir_path.join(format!("dated_file{}.txt", i));
            create_file_at_hour(&file_path, test_days[i]);
        }

        // Create .csv files that should not be included.
        // One of these is the newest file
        for i in 4..6 {
            let file_path = dir_path.join(format!("dated_file{}.csv", i));
            create_file_at_hour(&file_path, test_days[i]);
        }

        // Create a single .cfg file, useful for single length files
        let file_path = dir_path.join("dated_file.cfg");
        create_file_at_hour(&file_path, 4);

        ExperimentPart::new("Test".to_string(),
                            PathBuf::from(dir_path),
                            String::from(glob))
    }

    // Test that we get the correct files gathered by the file list creation
    #[test]
    fn test_file_list_creation() {
        let exp_part_test = experiment_part_with_sorted_files("*.txt");

        assert_eq!(exp_part_test.n_files, 4);
        for file_ in exp_part_test.file_list {
            assert_eq!(file_.path.extension().unwrap(), "txt");
        }
    }

    // Test the sorting of the files
    #[test]
    fn test_newest_file() {
        let exp_part_test = experiment_part_with_sorted_files("*.txt");
        let newest_file = exp_part_test.get_newest_file();

        let newest_file_name_expected = "dated_file2.txt";
        let newest_file_name_actual = newest_file.path.file_name()
            .unwrap();

        assert_eq!(newest_file_name_actual, newest_file_name_expected)
    }

    // Test the sorting of the files
    #[test]
    fn test_oldest_file() {
        let exp_part_test = experiment_part_with_sorted_files("*.txt");
        let oldest_file = exp_part_test.get_oldest_file();

        let oldest_file_name_expected = "dated_file1.txt";
        let oldest_file_name_actual = oldest_file.path.file_name()
            .unwrap();

        assert_eq!(oldest_file_name_actual, oldest_file_name_expected)
    }

    // Test the summary when no files are found
    #[test]
    fn test_no_files_found_summary() {
        let exp_part_empty = experiment_part_with_sorted_files("*.rs");

        let summary_actual = exp_part_empty.create_summary();
        let summary_expected = "No files found in Test.";

        assert_eq!(summary_expected, summary_actual)
    }

    #[test]
    fn test_single_summary() {
        let exp_part_empty = experiment_part_with_sorted_files("*.cfg");

        let summary_actual = exp_part_empty.create_summary();
        let summary_expected = "Test contains 1 file";

        assert!(summary_actual.starts_with(summary_expected));
    }

    #[test]
    fn test_multiple_summary() {
        let exp_part_empty = experiment_part_with_sorted_files("*.txt");

        let summary_actual = exp_part_empty.create_summary();
        let summary_expected = "Test contains 4 file";

        assert!(summary_actual.starts_with(summary_expected));
    }
}
