use std::{process::Command, str::Lines};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    commit::expect_commit,
    data::{Commit, DetailedCommit},
};

pub fn get_stats() -> Option<Vec<DetailedCommit>> {
    let output = Command::new("git")
        .arg("log")
        .arg("--decorate=no")
        .arg("--date=unix")
        .arg("--shortstat")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    static COMMIT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^commit [a-f0-9]{40}$").unwrap());

    let output = String::from_utf8(output.stdout).ok()?;
    let mut iter = output.lines().into_iter();
    let mut commits: Vec<DetailedCommit> = vec![];

    while let Some(line) = iter.next() {
        if COMMIT_REGEX.is_match(line) {
            // good, now expect commit AND stats
            if let Some(x) = expect_commit(&mut iter).and_then(|c| expect_stat(&mut iter, c)) {
                commits.push(x);
            }
        }
    }

    return Some(commits);
}

pub fn expect_stat(iter: &mut Lines, commit: Commit) -> Option<DetailedCommit> {
    static STAT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^ \d+ files? changed").unwrap());
    static INSERT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) insertions\(\+\)").unwrap());
    static DELETE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\d+) deletions\(-\)").unwrap());
    let mut line = iter.next().unwrap();

    while !STAT_REGEX.is_match(line) {
        line = iter.next().unwrap();
    }

    let insertions = INSERT_REGEX.captures(line).map_or(0, |mm| {
        mm.get(1).map_or(0, |m| m.as_str().parse::<i64>().unwrap())
    });
    let deletions = DELETE_REGEX.captures(line).map_or(0, |mm| {
        mm.get(1).map_or(0, |m| m.as_str().parse::<i64>().unwrap())
    });
    return Some(DetailedCommit {
        commit,
        insertions,
        deletions,
    });
}
