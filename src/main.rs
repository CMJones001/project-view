#[macro_use]
extern crate log;
extern crate env_logger;
extern crate chrono;
mod file_status;
mod git_commits;
use std::{process,path};

fn main() {
    env_logger::init();

    let current = chrono::offset::Utc::now();
    let commit = git_commits::CommitInformation::from(current);

    println!("Commit date {}", commit.date);
    println!("Commit structure {}", commit.subject);

    // // Search for files matching the glob pattern, quit with error otherwise.
    // let file_list = file_status::list_files_in_dir(dir_path, &glob_pattern)
    //     .unwrap_or_else(|| {
    //         println!("No files found");
    //         process::exit(1);
    // });

    // // Print the first file
    // // This panics if a first file is not found, but this should not be possible
    // // as we have just quit on an empty list.
    // let first_file = &file_list[0];
    // println!("First file is {}", first_file.display());
}
