/// Manage related information about git commits.
///
/// While a crate does exist for managing git commits, however this is currently
/// unstable and we don't require most of the functionality and there doesn't
/// seem to be a consistent way to get the last commit before a given date.

use chrono::{Utc};
use std::process::Command;
use std::str;

pub struct CommitInformation {
    hash: String,
    date: chrono::DateTime::<Utc>,
    subject: String,
    commits_after: i32,
}

impl CommitInformation {
    /// Get the last commit before a given date.
    pub fn from(date: chrono::DateTime::<Utc>) -> CommitInformation {
        // Get the last commit, with an abbreviated hash, date of the commit and
        // the header of the subject line.
        let git_rev_call = Command::new("git")
            .arg("rev-list")
            .arg("-1")
            .arg("--pretty=format:%cI%n%s")
            .arg("--all")
            .arg("--abbrev-commit")
            .output()
            .expect("Failed to execute git rev call");

        // Format the git string into Vector<str>, breaking on new lines
        let git_string = &capture_output_as_string(git_rev_call);
        if git_string.len() == 0 {panic!("Git command returned no stdout");}
        let git_parts = split_git_info_string(git_string);

        // Print the string as debug logging
        for (num, part) in git_parts.iter().enumerate() {
            debug!("git part {} - {}", num, part);
        }

        CommitInformation{
            hash:"thou".to_string(),
            date:date,
            subject:"Test".to_string(),
            commits_after:5
        }

    }

    // pub fn get_number_of_commits_behind() {
    // }

    // /// A more generalised form of the difference between the commit and data
    // /// files. This may included the number of major or minor revisions.

    // pub fn distance() {
    // }
}

/// Get the stdout from a function and convert this byte string into a utf8
/// string.
fn capture_output_as_string(process_out: std::process::Output) -> String {
    let output_bytes = process_out.stdout;
    let output = String::from_utf8(output_bytes)
        .expect("Invalid byte sequence from process.");

    output
}

/// Get the stderr from a function and convert this byte string into a utf8
/// string.
fn capture_error_as_string(process_out: std::process::Output) -> String {
    let output_bytes = process_out.stderr;
    let output = String::from_utf8(output_bytes)
        .expect("Invalid byte sequence from process.");

    output
}

/// We expect that the git information is formatted to split the fields with
/// new lines. Note that a newline at the n
fn split_git_info_string(git_string: &str) -> Vec<&str> {
    let split_chars = "\n";
    let split = git_string.split_terminator(split_chars);

    let values = split.collect::<Vec<&str>>();
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    // Test running a shell command and retrieving the output
    #[test]
    fn test_command_call() {
        let echo_test = Command::new("echo")
            .arg("test items")
            .output()
            .expect("Failed to execute echo");

        let output = capture_output_as_string(echo_test);
        assert_eq!(output, "test items\n")
    }

    #[test]
    fn test_split_string_on_newline() {
        let test_string = "hash\nsubject_information\ndate_2019_10_10";

        let split_expected = vec!["hash",
                                  "subject_information",
                                  "date_2019_10_10"];
        let split_actual = split_git_info_string(test_string);

        assert_eq!(split_expected, split_actual)
    }

    #[test]
    // Creating of a commit before a given date
    fn create_commit_information() {
        let current = chrono::offset::Utc::now();
        let commit = CommitInformation::from(current);
        assert!(false)
    }
}
