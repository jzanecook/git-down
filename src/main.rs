use clap::Parser;
use std::process::Command;

#[derive(Debug)]
struct GitFileStatus {
    x: char,                  // Remote
    y: char,                  // Local
    filename: String,         // Current Filename
    origname: Option<String>, // Original if renamed
}

#[derive(Debug)]
struct GitProcess {
    status_map: Vec<GitFileStatus>,
}

impl GitProcess {
    fn new() -> Self {
        GitProcess {
            status_map: Vec::new(),
        }
    }

    fn check_status(&mut self) {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()
            .expect("failed to check git status");
        let stdout = String::from_utf8(output.stdout).expect("failed to process git status stdout");
        let lines = stdout.lines();
        
        let mut status_map = Vec::<GitFileStatus>::new();
        for line in lines {
            let mut chars = line.chars();
            let x = chars.next().expect("X char not found or errored");
            let y = chars.next().expect("Y char not found or errored");

            // This means that there is no changes on the remote
            let parts: Vec<&str> = line.split_whitespace().collect();
            let file = parts[1];

            let mut file_status = GitFileStatus {
                x,
                y,
                filename: file.to_string(),
                origname: None,
            };

            if line.contains("->") {
                // This means the file was renamed or moved
                let orig = parts[1]; // The orig_file comes first, overwriting the regular filename
                let file = parts[3]; // The file_name should be after the '->'
                file_status.filename = file.to_string();
                file_status.origname = Some(orig.to_string());
            }

            status_map.push(file_status);
        }
        self.status_map = status_map;
    }

    fn get_diff(git_file: &GitFileStatus) -> String {
        let output = Command::new("git")
            .args(["diff", git_file.filename.as_str()])
            .output()
            .expect("failed to git diff");
        String::from_utf8(output.stdout).expect("failed to process git status stdout")
    }

    fn get_diffs(&self) -> Vec<String> {
        let mut diffs: Vec<String> = Vec::new();
        for file in &self.status_map {
            if file.x != '?' && file.y != '?' {
                diffs.push(GitProcess::get_diff(&file))
            }
        }
        diffs
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = "git-down  Copyright (C) 2024  J. Zane Cook")]
struct Args {
    command: Option<String>,
}

fn main() {
    let args = Args::parse();

    // println!("{:?}", args.command);

    let mut git_process = GitProcess::new();

    git_process.check_status();
    let diff_map = git_process.get_diffs();

    println!("{:?}", git_process.status_map);
    println!("{:?}", diff_map);
}
