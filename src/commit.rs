use regex::Regex;
use std::{process::Command, str::Lines};
use time::OffsetDateTime;

use crate::data::{Author, Commit};

pub fn get_commits() -> Option<Vec<Commit>> {
    let output = Command::new("git")
        .arg("log")
        .arg("--decorate=no")
        .arg("--date=unix")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let commit_regex = Regex::new(r"^commit [a-f0-9]{40}$").ok()?;

    let output = String::from_utf8(output.stdout).ok()?;
    let mut iter = output.lines().into_iter();
    let mut commits: Vec<Commit> = vec![];

    while let Some(line) = iter.next() {
        if commit_regex.is_match(line) {
            // good, now expect author line
            if let Some(c) = expect_commit(&mut iter) {
                commits.push(c);
            }
        }
    }

    return Some(commits);
}

pub fn expect_commit(iter: &mut Lines) -> Option<Commit> {
    let author_regex = Regex::new(r"^Author: (.*?)$").ok()?;
    let email_regex = Regex::new(r"^<(.+@.+)>$").ok()?;
    let date_regex = Regex::new(r"^Date:\s*([0-9]+)$").ok()?;
    let merge_regex = Regex::new(r"^Merge: .*$").ok()?;

    // the amount of ugly unwraps here is exactly what I want
    let next_line = iter.next().unwrap();
    if merge_regex.is_match(next_line) {
        return None;
    }

    let author_email = next_line;
    let date = iter.next().unwrap();

    let (author, email) = author_email.split_at(author_email.find(" <").unwrap());

    let author_match = author_regex.captures(author).unwrap()[1].to_string();
    let email_match = email_regex.captures(email.trim()).unwrap()[1].to_string();
    let date_match = date_regex.captures(date).unwrap()[1]
        .to_string()
        .parse::<i64>()
        .unwrap();

    return Some(Commit {
        author: Author {
            name: author_match,
            email: email_match,
        },
        time: OffsetDateTime::from_unix_timestamp(date_match).unwrap(),
    });
}
