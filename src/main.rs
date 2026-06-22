use clap::{Parser, Subcommand};
use std::process::{exit, Command};

#[derive(Parser)]
#[command(name = "hacky")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Commit {
        message: String,
    },
    Uncommit,
}

fn run_git(args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .status()
        .expect("failed to run git");

    if !status.success() {
        exit(1);
    }
}

fn has_changes() -> bool {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .expect("failed to run git status");

    !output.stdout.is_empty()
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Commit { message } => {
            if !has_changes() {
                println!("No changes to commit");
                return;
            }

            run_git(&["add", "."]);
            run_git(&["commit", "-m", &message]);
            run_git(&["push"]);
        }

        Commands::Uncommit => {
            run_git(&["reset", "HEAD~1"]);
        }
    }
}
