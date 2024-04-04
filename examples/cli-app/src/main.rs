use anyhow::{Context, Ok, Result};
use clap::Parser;
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

/// Searches for a pattern in a file and displays the lines that contain it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to be read
    path: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct QueryResult {
    pattern: String,
    line_number: u32,
    pointer: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct OutputObject {
    result: Vec<QueryResult>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("Error reading file `{}`", &args.path.display()))?;
    let mut query_result = Vec::<QueryResult>::new();
    let pb = ProgressBar::new(content.lines().count() as u64);

    println!("Reading file {}", args.path.display());
    for (i, line) in content.lines().enumerate() {
        if line.contains(&args.pattern) {
            query_result.push(QueryResult {
                pattern: args.pattern.to_string(),
                line_number: i as u32,
                pointer: format!("{:p}", &line),
                content: line.to_string(),
            });
        }
        pb.inc(1);
    }
    pb.finish_with_message("Done!");

    let output_object = OutputObject {
        result: query_result,
    };
    let json = serde_json::to_string(&output_object)?;
    let bytes = json.as_bytes();
    let mut file = File::create("result.json")?;

    let pb = ProgressBar::new_spinner();

    file.write_all(bytes)?;
    pb.finish_with_message("File exported to `result.json`");

    Ok(())
}
