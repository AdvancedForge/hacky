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
    #[command(about = "Runs Git add ., commit -m, and push")]
    Commit {
        #[arg(help = "Commit message")]
        message: String,
    },

    #[command(about = "Undo the most recent commit")]
    Uncommit(UncommitArgs),

    #[command(about = "Amend the most recent commit")]
    Amend(AmendArgs),
}

#[derive(Args)]
#[group(multiple = false)]
struct UncommitArgs {
    #[arg(long, help = "Reset commit, but keep changes staged (reset --soft)")]
    soft: bool,

    #[arg(long, help = "Discard all changes from the last commit (reset --hard)")]
    hard: bool,

    #[arg(long, help = "Force-push the rewritten history (push --force-with-lease)")]
    push: bool,
}

#[derive(Args)]
struct AmendArgs {
    #[arg(help = "New commit message")]
    message: Option<String>,

    #[arg(long, help = "Force-push the amended commit (push --force-with-lease)")]
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

        Commands::Amend(args) => {
            run_git(&["add", "."]);

            if let Some(message) = args.message {
                run_git(&["commit", "--amend", "-m", &message]);
            } else {
                run_git(&["commit", "--amend", "--no-edit"]);
            }

            if args.push {
                run_git(&["push", "--force-with-lease"]);
            }
        }
    }
}
