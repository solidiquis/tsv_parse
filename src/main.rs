use clap::Parser;
use rayon::prelude::*;
use regex::Regex;
use std::{env, fmt, fs, path::PathBuf, process::ExitCode, thread};
use strip_ansi_escapes::strip as strip_ansi;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let Cli { path, pattern } = Cli::parse();

    if path.extension().map(|ext| ext != "tsv").unwrap_or(true) {
        return Err(Box::new(Error::NotTsv));
    }

    let tsv = fs::read_to_string(path)?;
    let values = tsv.split('\n');

    let num_workers = thread::available_parallelism()?.get();

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_workers)
        .build()?;

    let regex = Regex::new(&pattern)?;

    let matches = pool.install(|| {
        values
            .enumerate()
            .collect::<Vec<(usize, &str)>>()
            .into_par_iter()
            .filter_map(|(lineno, val)| {
                let val = strip_ansi(val).map_or(val.to_string(), |bytes| {
                    String::from_utf8_lossy(&bytes).to_string()
                });

                regex.find(&val).map(|mat| {
                    let match_str = mat.as_str();
                    let highlight = ansi_term::Color::Red.paint(match_str).to_string();
                    let data = val.replace(match_str, &highlight);
                    format!("{}", Output::new(lineno, data))
                })
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    });

    if matches.is_empty() {
        return Err(Box::new(Error::NoMatches));
    }

    if env::var_os("NO_COLOR").is_some() {
        let matches =
            strip_ansi(matches).map(|bytes| String::from_utf8_lossy(&bytes).to_string())?;
        println!("{matches}");
    } else {
        println!("{matches}");
    }

    Ok(())
}

#[derive(Parser)]
#[command(author = "Benjamin Nguyen", version = "0.1.0", about = "Parse a tsv file with provided regex pattern", long_about = None)]
struct Cli {
    /// Path to file
    path: PathBuf,

    /// Regex pattern to search
    pattern: String,
}

struct Output {
    lineno: usize,
    data: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("No matches using the provided pattern")]
    NoMatches,

    #[error("Provided file does not have the tsv extension")]
    NotTsv,
}

impl Output {
    fn new(lineno: usize, data: String) -> Self {
        Output { lineno, data }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lineno = ansi_term::Color::Green.paint(format!("{}", self.lineno));
        write!(f, "lineno {lineno}:\n{}", self.data)
    }
}
