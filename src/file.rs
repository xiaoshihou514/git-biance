use std::process::Command;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{commit::expect_commit, data::DetailedCommit, stat::expect_stat};

pub fn get_file_stats(path: String) -> Option<Vec<DetailedCommit>> {
    let output = Command::new("git")
        .arg("log")
        .arg("--decorate=no")
        .arg("--date=unix")
        .arg("--shortstat")
        .arg("--follow")
        .arg(path)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    static COMMIT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^commit [a-f0-9]{40}$").unwrap());

    let output = String::from_utf8(output.stdout).ok()?;
    let mut iter = output.lines();
    let mut commits: Vec<DetailedCommit> = vec![];

    while let Some(line) = iter.next() {
        if COMMIT_REGEX.is_match(line) {
            // good, now expect commit AND stats
            if let Some(x) = expect_commit(&mut iter).and_then(|c| expect_stat(&mut iter, c)) {
                commits.push(x);
            }
        }
    }

    Some(commits)
}
