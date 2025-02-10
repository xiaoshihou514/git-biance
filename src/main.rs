mod commit;
mod data;
mod file;
mod stat;

use crate::commit::{get_commits, print_commit, print_commit_data};
use crate::file::get_file_stats;
use crate::stat::{get_stats, print_stats, print_stats_data};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    /// Specify certain author
    author: Option<String>,

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
    #[arg(short, long, value_delimiter = ' ', num_args = 1.., value_name = "FILE")]
    file: Vec<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    if args.commits {
        let commits = get_commits().expect("git log parse failed");
        if args.gnuplot {
            print_commit_data(commits, args.author);
        } else {
            print_commit(commits, args.author);
        }
    } else if args.stat {
        let stats = get_stats().expect("git log parse failed");
        if args.gnuplot {
            print_stats_data(stats, args.author);
        } else {
            print_stats(stats, args.author);
        }
    } else if !args.file.is_empty() {
        let paths: Vec<String> = args
            .file
            .iter()
            .map(|x| x.to_owned().into_os_string().into_string().unwrap())
            .collect();
        let mut stats = vec![];
        for file in paths {
            let mut single = get_file_stats(file).expect("git log parse failed");
            stats.append(&mut single);
        }

        if args.gnuplot {
            print_stats_data(stats, args.author);
        } else {
            print_stats(stats, args.author);
        }
    }
}
