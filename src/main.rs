#![allow(dead_code)]
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;

mod file_status;
mod git_commits;
mod experiment_structure;
use std::{process,path};
use std::path::PathBuf;

fn main() {
    env_logger::init();

    let dir_path =  PathBuf::from("figs/fittingSmall/");
    let glob_pattern = "**/*.png".to_string();

    let fourier_terms = experiment_structure::ExperimentPart::new(
        "Figures".to_string(), dir_path, glob_pattern
    );

    println!("{}", fourier_terms.create_summary());

}
