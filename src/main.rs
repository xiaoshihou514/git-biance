mod commit;
mod data;
mod stat;

use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use commit::get_commits;

use crate::{data::Author, stat::get_stats};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    /// Show total insertions and deletions
    #[arg(short, long)]
    stat: bool,

    /// Show total commits
    #[arg(short, long)]
    commits: bool,

    /// Outputs data to be plotted by gnuplot
    #[arg(short, long)]
    gnuplot: bool,

    /// Specify certain author
    #[arg(short, long, value_name = "AURTHOR")]
    author: Option<String>,

    /// Show insertions and deletions on single file
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    if args.commits {
        let commits = get_commits().expect("git log parsed successfully");
        // count commits
        let mut stats: HashMap<Author, i64> = HashMap::new();
        for c in commits.into_iter() {
            let author = c.author;
            let count = stats.get(&author).map(|i| i.to_owned() + 1).unwrap_or(1);
            stats.insert(author, count);
        }
        let stats_sorted: Vec<(&Author, &i64)> = match args.author {
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
    } else if args.stat {
        let stats = get_stats().expect("git log parsed successfully");
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
        let stats_sorted: Vec<(&Author, &(i64, i64))> = match args.author {
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
    } else if let Some(_) = args.file {
        todo!()
    }
}
