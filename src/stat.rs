use charming::{
    component::{Axis, DataZoom},
    element::{AxisType, ItemStyle},
    series::Bar,
    Chart, HtmlRenderer,
};
use std::{collections::HashMap, process::Command, str::Lines};

use once_cell::sync::Lazy;
use regex::Regex;
use time::format_description::{self, BorrowedFormatItem};

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
    Some(DetailedCommit {
        commit,
        insertions,
        deletions,
    })
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
        Some(a) => changes
            .iter()
            .filter(|x| x.to_owned().0.to_owned().name.eq(&a))
            .collect::<Vec<_>>(),
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

pub fn plot_stats(stats: Vec<DetailedCommit>, author: Option<String>) {
    static TIME_FORMAT: Lazy<Vec<BorrowedFormatItem<'_>>> =
        Lazy::new(|| format_description::parse("[year]/[month]/[day]T00:00:00").unwrap());
    // don't fully understand this part
    let shown_commits: Box<dyn Iterator<Item = &DetailedCommit>> =
        author.map_or(Box::new(stats.iter()), |name| {
            Box::new(
                stats
                    .iter()
                    .filter(move |dc| dc.commit.author.name.eq(&name)),
            )
        });

    // for real?
    let mut temp: Vec<_> = shown_commits
        .map(|dc| {
            (
                dc.commit.time.format(&TIME_FORMAT).unwrap(),
                dc.insertions,
                dc.deletions,
            )
        })
        .collect::<Vec<_>>();
    temp.sort_unstable();
    let insertions: Vec<_> = temp
        .chunk_by(|x, y| x.0 == y.0)
        .map(|xs| {
            vec![
                xs[0].0.clone(),
                xs.iter().fold(0, |acc, x| acc + x.1).to_string(),
            ]
        })
        .collect();
    let deletions: Vec<_> = temp
        .chunk_by(|x, y| x.0 == y.0)
        .map(|xs| {
            vec![
                xs[0].0.clone(),
                xs.iter().fold(0, |acc, x| acc + x.2).to_string(),
            ]
        })
        .collect();

    let chart = Chart::new()
        .x_axis(Axis::new().type_(AxisType::Time))
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(
            Bar::new()
                .data(insertions)
                .item_style(ItemStyle::new().color("#3D6C46")),
        )
        .series(
            Bar::new()
                .data(deletions)
                .item_style(ItemStyle::new().color("#E2524A")),
        )
        .data_zoom(DataZoom::new().brush_select(true));

    let mut renderer = HtmlRenderer::new("commits", 1440, 512).theme(charming::theme::Theme::Dark);
    renderer.save(&chart, "stats.html").unwrap();
}
