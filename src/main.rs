//! codetrace — record how you actually solve a problem.
//!
//! The shell harness in `bin/` implements the working pipeline today. This
//! binary is where it migrates as each subsystem lands; see SPEC.md.

#[allow(dead_code)]
mod event;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ct", version, about = "Record how you actually solve a problem")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Check toolchains, mic, and whisper model
    Doctor,
    /// Scaffold a new problem
    New { slug: String, #[arg(long, default_value = "java")] lang: String },
    /// Start a session (TUI dashboard)
    Rec { problem: String },
    /// Compile + test, emit events
    Run,
    /// Transcribe session audio into speech events
    Transcribe { id: Option<String> },
    /// Regenerate session.md and metrics.json
    Render { id: Option<String> },
    /// Print the last N sessions for review
    Review { #[arg(short, long, default_value_t = 5)] n: usize },
}

fn main() -> Result<()> {
    match Cli::parse().cmd {
        Cmd::Doctor => doctor(),
        other => {
            let name = match other {
                Cmd::New { .. } => "ct new",
                Cmd::Rec { .. } => "ct rec",
                Cmd::Run => "ct run",
                Cmd::Transcribe { .. } => "ct transcribe",
                Cmd::Render { .. } => "ct render",
                Cmd::Review { .. } => "ct review",
                Cmd::Doctor => unreachable!(),
            };
            eprintln!("{name}: not implemented yet — use the shell harness in bin/");
            eprintln!("see README.md 'Quick start (no build required)'");
            std::process::exit(2);
        }
    }
}

fn doctor() -> Result<()> {
    let checks: &[(&str, &str)] = &[
        ("git", "session snapshots"),
        ("jq", "shell harness"),
        ("javac", "java adapter"),
        ("java", "java adapter"),
        ("whisper-cli", "transcription (whisper.cpp)"),
        ("ffmpeg", "video/audio extraction (optional)"),
    ];
    let mut missing = 0;
    for (bin, why) in checks {
        let found = which(bin);
        println!("{} {:<12} {}", if found { "ok  " } else { "MISS" }, bin, why);
        if !found { missing += 1; }
    }
    if missing > 0 {
        println!("\n{missing} missing. The java adapter and transcription are optional\nuntil you use them; git and jq are required by the shell harness.");
    }
    Ok(())
}

fn which(bin: &str) -> bool {
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths).any(|p| p.join(bin).is_file())
        })
        .unwrap_or(false)
}
