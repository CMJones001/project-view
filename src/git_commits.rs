/// Manage related information about git commits.
///
/// While a crate does exist for managing git commits, however this is currently
/// unstable and we don't require most of the functionality and there doesn't
/// seem to be a consistent way to get the last commit before a given date.
extern crate chrono;

use chrono::Utc;
use std::process::Command;
use std::str;

pub struct CommitInformation {
    hash: String,
    date: chrono::DateTime::<Utc>,
    subject: String,
    commits_afetr: i32,
}

/// Get the stdout from a function and convert this byte string into a utf8
/// string.
fn capture_output_as_string(process_out: std::process::Output) -> String {
    let output_bytes = process_out.stdout;
    let output = String::from_utf8(output_bytes)
        .expect("Invalid byte sequence from process.");

    output
}

/// We expect that the git information is formatted to split the fields with
/// double underscores. This was chosen as to avoid clashes with likely common
/// strings.
fn split_git_info_string(git_string: &str) -> Vec<&str> {
    let split_chars = "__";
    let split = git_string.split(split_chars);

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
    fn test_split_string_on_dbl_underscores() {
        let test_string = "hash__subject_information__date_2019_10_10";

        let split_expected = vec!["hash",
                                  "subject_information",
                                  "date_2019_10_10"];
        let split_actual = split_git_info_string(test_string);

        assert_eq!(split_expected, split_actual)
    }
}
