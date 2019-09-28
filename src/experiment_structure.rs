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

    /// Print information about the number and age of files in the Part
    pub fn create_summary(&self) -> String {
        // Spaces used to indent secondary lines
        let indent = "    ";

        let summary = format!(
            "{} contains {} file",
            &self.name,
            &self.file_list.len(),
        );

        let newest_summary = format!(
            "\n{}Newest file modified at {}",
            indent,
            &self.get_newest_file().formatted_time()
        );

        let mut oldest_summary = String::from("");
        if &self.file_list.len() > &1 {
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
