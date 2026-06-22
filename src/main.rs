use clap::{Args, Parser, Subcommand};
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

    Uncommit(UncommitArgs),
}

#[derive(Args)]
#[group(multiple = false)]
struct UncommitArgs {
    #[arg(long)]
    soft: bool,

    #[arg(long)]
    hard: bool,

    #[arg(long)]
    push: bool,
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

        Commands::Uncommit(args) => {
            let reset_mode = if args.soft {
                "--soft"
            } else if args.hard {
                "--hard"
            } else {
                "--mixed"
            };

            run_git(&["reset", reset_mode, "HEAD~1"]);

            if args.push {
                run_git(&["push", "--force-with-lease"]);
            }
        }
    }
}
