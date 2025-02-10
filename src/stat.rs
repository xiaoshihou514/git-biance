use std::{collections::HashMap, process::Command, str::Lines};

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    commit::expect_commit,
    data::{Author, Commit, DetailedCommit},
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

pub fn print_stats(stats: Vec<DetailedCommit>, author: Option<String>) {
    // count commits
    let mut changes: HashMap<Author, (i64, i64)> = HashMap::new();
    for c in stats.into_iter() {
        let author = c.commit.author;
        let count = changes
            .get(&author)
            .map(|s| {
                let (i, d) = s;
                (i + c.insertions, d + c.deletions)
            })
            .unwrap_or((c.insertions, c.deletions));
        changes.insert(author, count);
    }
    let stats_sorted: Vec<(&Author, &(i64, i64))> = match author {
        Some(a) => Vec::from(
            changes
                .iter()
                .filter(|x| x.to_owned().0.to_owned().name.eq(&a))
                .collect::<Vec<_>>(),
        ),
        None => {
            let mut stats = Vec::from_iter(changes.iter());
            stats.sort_by(|a1, a2| a2.1.cmp(a1.1));
            stats
        }
    };

    println!(
        "{0: <30} | {1: <12} | {2: <12}",
        "Author", "Insertions", "Deletions"
    );
    for (author, (insertions, deletions)) in stats_sorted {
        println!(
            "{0: <30} | \u{1B}[92m{1: <12}\u{1B}[0m | \u{1B}[31m{2: <12}\u{1B}[0m",
            author.name,
            insertions.to_string() + "+",
            deletions.to_string() + "-",
        );
    }
}

pub fn print_stats_data(stats: Vec<DetailedCommit>, author: Option<String>) {
    todo!();
}
