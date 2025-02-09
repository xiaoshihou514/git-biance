mod commit;
mod data;

use std::{collections::HashMap, path::PathBuf};

use clap::Parser;
use commit::get_commits;

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

    /// Show insertions and deletions on single file
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    if args.commits {
        let commits = get_commits().unwrap();
        // count commits
        let mut stats: HashMap<String, i64> = HashMap::new();
        for c in commits.into_iter() {
            let email = c.author.email;
            let count = stats.get(&email).map(|i| i.to_owned() + 1).unwrap_or(1);
            stats.insert(email, count);
        }
        let mut stats_sorted = Vec::from_iter(stats.iter());
        stats_sorted.sort_by(|a1, a2| a2.1.cmp(a1.1));
        println!("{0: <30} | {1: <30}", "Author", "Commits");
        for (author, commit_count) in stats_sorted {
            println!(
                "{0: <30} | \u{1B}[94m{1: <30}\u{1B}[0m",
                author, commit_count
            );
        }
    } else if args.stat {
        todo!()
    } else if let Some(_) = args.file {
        todo!()
    }
}
