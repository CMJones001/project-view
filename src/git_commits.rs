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

        let git_string = capture_output_as_string(&git_rev_call)
            .expect_stdout("Failed shell command to get git commits before given date.");

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

    let git_string = capture_output_as_string(&git_rev_call)
        .expect_stdout("Failed to get all commits after a given date.");

    // Each line in a new commit -> count the number of lines.
    let num_commits = git_string.split_terminator("\n").count();

    num_commits.try_into().unwrap()
}

/// Possible values from running a shell command, stored as strings 
enum CommandReturn {
    Stdout(String),
    Stderr(String),
    None,
}

/// For now we provide methods similar to unwrap and expect, but panic if no
/// stdout is found and prints the stderr to error logs.
impl CommandReturn {
    // Panic if no stdout is found
    fn unwrap_stdout(&self) -> String {
        match &self {
            CommandReturn::Stdout(message) => message.to_string(),
            CommandReturn::Stderr(message) => {
                error!("Command stderr:\n{}", message);
                panic!("Only stderr received from command.");
            }
            CommandReturn::None => {panic!("Command returned no output")}
        }
    }

    fn expect_stdout(&self, err_message: &str) -> String {
        match &self {
            CommandReturn::Stdout(message) => message.to_string(),
            CommandReturn::Stderr(message) => {
                error!("Command stderr:\n{}", message);
                panic!("{}", err_message);
            }
            CommandReturn::None => {panic!("{}", err_message)}
        }
    }
}

/// Try to convert the output from a bash command into a utf-8 string.
/// If no stdout is captured, write stderr to logs then return None.
fn capture_output_as_string(process_out: &std::process::Output)
                                -> CommandReturn {
    let stdout_bytes = &process_out.stdout;
    let output = String::from_utf8(stdout_bytes.clone())
        .expect("Invalid byte sequence from command stdout.");

    if output.len() > 0 {
        CommandReturn::Stdout(output)
    } else {
        let stderr_bytes = &process_out.stderr;
        let errout = String::from_utf8(stderr_bytes.clone())
            .expect("Invalid byte sequence from command stderr.");
        if errout.len() > 0 {
            CommandReturn::Stderr(errout)
        } else {
            CommandReturn::None
        }
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

        let output = capture_output_as_string(&echo_test)
            .unwrap_stdout();
        assert_eq!(output, "test items\n")
    }

    // Test a shell command than only returns stderr should fail. The man
    // command will do this if no arguments are provided
    #[test]
    #[should_panic(expected="Only stderr")]
    fn test_bad_command_call() {
        let echo_test = Command::new("man")
            .output()
            .expect("Failed to execute echo");

        let output = capture_output_as_string(&echo_test).unwrap_stdout();

    }

    // Test a shell command than only returns stderr should fail. The man
    // command will do this if no arguments are provided
    #[test]
    #[should_panic(expected="Custom message")]
    fn test_bad_command_call_with_message() {
        let echo_test = Command::new("man")
            .output()
            .expect("Failed to execute echo");

        let output = capture_output_as_string(&echo_test)
            .expect_stdout("Custom message");

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
