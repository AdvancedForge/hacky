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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Commit { message } => {
            run_git(&["add", "."]);
            run_git(&["commit", "-m", &message]);
            run_git(&["push"]);
        }
    }
}
