mod file_status;
use std::process;

fn main() {
    println!("Hello, world!");
    let glob_pattern = String::from("../tests/testDirs/**/*.cfg");

    // Search for files matching the glob pattern, quit with error otherwise.
    let file_list = file_status::list_files(&glob_pattern).unwrap_or_else(|| {
        println!("No files found");
        process::exit(1);
    });

    // Print the first file
    // This panics if a first file is not found, but this should not be possible
    // as we have just quit on an empty list.
    let first_file = &file_list[0];
    println!("First file is {}", first_file.display());
}
