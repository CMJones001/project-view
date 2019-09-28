#![allow(dead_code)]
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

mod file_status;
mod git_commits;
mod experiment_structure;
use std::path::PathBuf;

fn main() {
    env_logger::init();

    let mut dir_path = PathBuf::from("figs/fittingSmall/");
    let mut glob_pattern = "**/*.png".to_string();

    let fitting_terms = experiment_structure::ExperimentPart::new(
        "Figures".to_string(),
        dir_path,
        glob_pattern
    );

    dir_path = PathBuf::from(".");
    glob_pattern = "*.cfg".to_string();
    let config_file = experiment_structure::ExperimentPart::new(
        "Config file".to_string(),
        dir_path,
        glob_pattern
    );

    dir_path = PathBuf::from("fourier/");
    glob_pattern = "*.xz".to_string();
    let fourier_terms = experiment_structure::ExperimentPart::new(
        "Fourier Terms".to_string(),
        dir_path,
        glob_pattern
    );

    dir_path = PathBuf::from("reason");
    glob_pattern = "*.iz".to_string();
    let missing_terms = experiment_structure::ExperimentPart::new(
        "Empty dirs".to_string(),
        dir_path,
        glob_pattern,
    );



    println!("{}\n", config_file.create_summary());
    println!("{}\n", fourier_terms.create_summary());
    println!("{}\n",   missing_terms.create_summary());
    println!("{}",   fitting_terms.create_summary());

}
