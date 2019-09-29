//! # Data pipeline for rapidly developing code.
//!
//! Academic software spends most of its lifetime in an active development
//! cycle. We therefore introduce this tool to keep track of the users data in
//! comparison to the stage of the source code, allowing us to see if any stages
//! of analysis are out of date due to changes in format or method.
//!
//! For now we simply report on the status of the data, but we wish to extend
//! this to allow automatic running of the relevant scripts.
//!
//! # Data structure
//!
//! Our common dataflow boils down to a large set of initial data that requires
//! multiple stages of analysis. The stages of this analysis are organised into
//! sub-directories of a main experiment directory (not including the initial
//! data).
//!
//! In some cases, there may be multiple experiment directories, this may
//! correspond to different starting data or different approaches in analysis.
//!
//! ## Reproducibilty
//!
//! For reproducibilty each of these Experiment directories should contain a
//! configuration file that contains any of the metadata on the run, such as
//! parameters or methods used in the analysis. Therefore, a user should be able
//! to recreate the analysis using only the initial data and this configuration
//! file.
//!
//! This program aims to automate this final step, not only for the initial
//! author of the analysis during development but any later users of the
//! software.



#![allow(dead_code)]
#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;

mod experiment_structure;
mod file_status;
mod git_commits;

use crate::experiment_structure::ExperimentPart;

/// Show the status of the current directory and age of the data contained.
///
/// For now the expected files are hard coded, but this will be extended into a
/// project configuration file.
fn main() {
    env_logger::init();

    let mut dir_path = "figs/fittingSmall/";
    let mut glob_pattern = "**/*.png".to_string();

    let fitting_terms =
        ExperimentPart::new(
            "Figures".to_string(),
            dir_path,
            glob_pattern
        );

    dir_path = ".";
    glob_pattern = "*.cfg".to_string();
    let config_file = ExperimentPart::new(
        "Config file".to_string(),
        dir_path,
        glob_pattern,
    );

    dir_path = "fourier/";
    glob_pattern = "*.xz".to_string();
    let fourier_terms = ExperimentPart::new(
        "Fourier Terms".to_string(),
        dir_path,
        glob_pattern,
    );

    dir_path = "reason";
    glob_pattern = "*.iz".to_string();
    let missing_terms =
        ExperimentPart::new("Empty dirs".to_string(), dir_path, glob_pattern);

    println!("{}\n", config_file.create_summary());
    println!("{}\n", fourier_terms.create_summary());
    println!("{}\n", missing_terms.create_summary());
    println!("{}", fitting_terms.create_summary());
}
