/// Manage related information about git commits.
///
/// While a crate does exist for managing git commits, however this is currently
/// unstable and we don't require most of the functionality and there doesn't
/// seem to be a consistent way to get the last commit before a given date.

use chrono::{Utc,FixedOffset};
use std::process::Command;
use std::str;
use std::convert::TryInto;

pub struct CommitInformation {
    pub hash: String,
    pub date: chrono::DateTime::<Utc>,
    pub subject: String,
    pub commits_after: i32,
}

impl CommitInformation {
    /// Get the last commit before a given date.
    pub fn from(date: chrono::DateTime::<Utc>) -> CommitInformation {
        // Get the last commit, with an abbreviated hash, date of the commit and
        // the header of the subject line.
        let pretty_format_arg = format!("--pretty=format:{}", "%cI%n%s");
        let before_date_arg = format!("--before='{}'", date.to_rfc3339());

        let git_rev_call = Command::new("git")
            .arg("rev-list")
            .arg("-1")
            .arg("--pretty=format:%cI%n%s")
            .arg(before_date_arg)
            .arg("--all")
            .arg("--abbrev-commit")
            .output()
            .expect("Failed to execute git rev call");

        let git_string = try_capture_output_as_string(&git_rev_call)
            .expect("No stdout on git rev-list");

        // Format the git string into Vector<str>, breaking on new lines
        let git_parts = split_git_info_string(&git_string);

        // Print the string as debug logging
        for (num, part) in git_parts.iter().enumerate() {
            debug!("git part {} - {}", num, part);
        }

        // Get the commit time with the correct time zone
        let commit_time = chrono::DateTime::parse_from_rfc3339(&git_parts[1])
            .expect("Unable to parse git time stamp");
        debug!("Formatted commit time {}", commit_time.format("%F %T"));

        CommitInformation{
            hash:git_parts[0].to_string(),
            date:chrono::DateTime::<Utc>::from(commit_time),
            subject:git_parts[2].to_string(),
            commits_after:get_number_of_commits_behind(commit_time),
        }
    }

}

/// A more generalised form of the difference between the commit and data
/// files. This may included the number of major or minor revisions.
/// For now we simply count the number of commits after the given date.
fn get_number_of_commits_behind(
    commit_date: chrono::DateTime::<FixedOffset>) -> i32 {
    let after_date_arg = format!("--after='{}'", commit_date.to_rfc3339());
    let git_rev_call = Command::new("git")
        .arg("rev-list")
        .arg(after_date_arg)
        .arg("--all")
        .output()
        .expect("Call to get rev-list failed.");

    let git_string = try_capture_output_as_string(&git_rev_call)
        .expect("No stdout for git rev-list");

    // Each line in a new commit -> count the number of lines.
    let num_commits = git_string.split_terminator("\n").count();

    num_commits.try_into().unwrap()
}

/// Try to convert the output from a bash command into a utf-8 string.
/// If no stdout is captured, write stderr to logs then return None.
fn try_capture_output_as_string(process_out: &std::process::Output)
                                -> Option<String> {
    let stdout_bytes = &process_out.stdout;
    let output = String::from_utf8(stdout_bytes.clone())
        .expect("Invalid byte sequence from command stdout.");

    if output.len() > 0 {
        Some(output)
    } else {
        let stderr_bytes = &process_out.stderr;
        let errout = String::from_utf8(stderr_bytes.clone())
            .expect("Invalid byte sequence from command stderr.");

        error!("Command call failed with stderr:\n{}",errout);
        None
    }
}

/// We expect that the git information is formatted to terminate the fields with
/// new lines.
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

        let output = try_capture_output_as_string(&echo_test)
            .unwrap();
        assert_eq!(output, "test items\n")
    }

    // Test a shell command than only returns stderr should fail. The man
    // command will do this if no arguments are provided
    #[test]
    fn test_bad_command_call() {
        let echo_test = Command::new("man")
            .output()
            .expect("Failed to execute echo");

        let output = try_capture_output_as_string(&echo_test);

        assert_eq!(output, None)
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


    // Creating of a commit before a given date
    #[test]
    fn create_commit_information() {
        let current = chrono::offset::Utc::now();
        let commit = CommitInformation::from(current);
        // assert!(false)
    }
}
