/// Grouping and organisation of the experiment files
///
/// # ExperimentPart
///
/// This contains a single step on pipeline, for instance a input directory,
/// script to run and the expected output files, but also a means of printing
/// the information about the creation date of the information
///
/// # Experiment
///
/// Root structure for the run, containing the configuration file and a vector
/// of ``ExperimentPart``s.
///
/// Later we expect to load these values from a configuration files but for now
/// the values are hard coded.

use std::path::{PathBuf,Path};
use crate::file_status as fs;

/// Container for the single step of the pipeline
///
/// # arg
pub struct ExperimentPart {
    name: String,
    pub file_list: Vec<fs::ExperimentFile>,
}

impl ExperimentPart {
    /// Create the object by providing a subDir, glob and name
    pub fn new(name: String, dir: PathBuf, glob_pattern: String)
                   -> ExperimentPart {

        let mut file_list = fs::list_files_in_dir(dir, &glob_pattern)
            .expect("No files found in experiment part.");

        file_list.sort_by(|a, b| b.modified.cmp(&a.modified));
        ExperimentPart { name, file_list}
    }

    pub fn create_summary(&self) -> String {
        // Spaces used to indent secondary lines
        let indent = "    ";

        let mut summary = format!("{} contains {} files",
            &self.name,
            &self.file_list.len(),
        );

        let newest_summary = format!("\n{}Newest file modified at {}",
                                  indent,
                                  &self.get_newest_file().formatted_time()
        );

        let oldest_summary = format!("\n{}Oldest file modified at {}",
                                  indent,
                                  &self.get_oldest_file().formatted_time()
        );

        summary + &newest_summary + &oldest_summary
    }

    fn get_newest_file(&self) -> &fs::ExperimentFile {
        &self.file_list[0]
    }

    fn get_oldest_file(&self) -> &fs::ExperimentFile {
        &self.file_list.last().unwrap()
    }

}

