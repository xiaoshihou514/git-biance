use charming::{
    component::{Axis, DataZoom},
    element::{AxisType, ItemStyle},
    series::Bar,
    Chart, HtmlRenderer,
};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{collections::HashMap, process::Command, str::Lines};
use time::{
    format_description::{self, BorrowedFormatItem},
    OffsetDateTime,
};

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

    static COMMIT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^commit [a-f0-9]{40}$").unwrap());

    let output = String::from_utf8(output.stdout).ok()?;
    let mut iter = output.lines().into_iter();
    let mut commits: Vec<Commit> = vec![];

    while let Some(line) = iter.next() {
        if COMMIT_REGEX.is_match(line) {
            // good, now expect author line
            if let Some(c) = expect_commit(&mut iter) {
                commits.push(c);
            }
        }
    }

    return Some(commits);
}

pub fn expect_commit(iter: &mut Lines) -> Option<Commit> {
    static AUTHOR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Author: (.*?)$").unwrap());
    static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^<(.+@.+)>$").unwrap());
    static DATE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Date:\s*(\d+)$").unwrap());
    static MERGE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^Merge: .*$").unwrap());

    // the amount of ugly unwraps here is exactly what I want
    let next_line = iter.next().unwrap();
    if MERGE_REGEX.is_match(next_line) {
        return None;
    }

    let author_email = next_line;
    let date = iter.next().unwrap();

    let (author, email) = author_email.split_at(author_email.find(" <").unwrap());

    let author_match = AUTHOR_REGEX
        .captures(author)
        .and_then(|x| x.get(1))
        .map(|x| x.to_owned().as_str().to_string())
        .unwrap_or(String::from("unknown"));
    let email_match = EMAIL_REGEX
        .captures(email.trim())
        .and_then(|x| x.get(1))
        .map(|x| x.to_owned().as_str().to_string())
        .unwrap_or(String::from("unknown"));
    let date_match = DATE_REGEX.captures(date).unwrap()[1]
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

pub fn print_commit(commits: Vec<Commit>, author: Option<String>) {
    // count commits
    let mut stats: HashMap<Author, i64> = HashMap::new();
    for c in commits.into_iter() {
        let author = c.author;
        let count = stats.get(&author).map(|i| i.to_owned() + 1).unwrap_or(1);
        stats.insert(author, count);
    }
    let stats_sorted: Vec<(&Author, &i64)> = match author {
        Some(a) => Vec::from(
            stats
                .iter()
                .filter(|x| x.to_owned().0.to_owned().name.eq(&a))
                .collect::<Vec<_>>(),
        ),
        None => {
            let mut stats = Vec::from_iter(stats.iter());
            stats.sort_by(|a1, a2| a2.1.cmp(a1.1));
            stats
        }
    };

    println!("{0: <30} | {1: <30}", "Author", "Commits");
    for (author, commit_count) in stats_sorted {
        println!(
            "{0: <30} | \u{1B}[94m{1: <30}\u{1B}[0m",
            author.name, commit_count
        );
    }
}

pub fn plot_commit(commits: Vec<Commit>, author: Option<String>) {
    static TIME_FORMAT: Lazy<Vec<BorrowedFormatItem<'_>>> =
        Lazy::new(|| format_description::parse("[year]/[month]/[day]T00:00:00").unwrap());
    // don't fully understand this part
    let shown_commits: Box<dyn Iterator<Item = &Commit>> = author
        .map_or(Box::new(commits.iter()), |name| {
            Box::new(commits.iter().filter(move |c| c.author.name.eq(&name)))
        });

    // for real?
    let mut temp: Vec<_> = shown_commits
        .map(|c| (c.time.format(&TIME_FORMAT).unwrap(), 1))
        .collect::<Vec<_>>();
    temp.sort_unstable();
    let data: Vec<_> = temp
        .chunk_by(|x, y| x.0 == y.0)
        .map(|xs| vec![xs[0].0.clone(), xs.len().to_string()])
        .collect();

    let chart = Chart::new()
        .x_axis(Axis::new().type_(AxisType::Time))
        .y_axis(Axis::new().type_(AxisType::Value))
        .series(
            Bar::new()
                .data(data)
                .item_style(ItemStyle::new().color("#0475FF")),
        )
        .data_zoom(DataZoom::new().brush_select(true));

    let mut renderer = HtmlRenderer::new("commits", 1440, 512).theme(charming::theme::Theme::Dark);
    renderer.save(&chart, "commits.html").unwrap();
}
