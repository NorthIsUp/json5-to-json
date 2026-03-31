use anyhow::{bail, Result};
use clap::Parser;
use json5_to_json::{convert, IndentOption};
use std::path::PathBuf;
use std::{fs, io, process::ExitCode};

#[derive(Parser)]
#[command(about = "Convert JSON5 to plain JSON")]
struct Args {
    /// Input files (use - for stdin, default: stdin)
    files: Vec<PathBuf>,

    /// Sort object keys
    #[arg(short, long)]
    sort: bool,

    /// Do not sort object keys
    #[arg(long = "no-sort", overrides_with = "sort", hide = true)]
    no_sort: bool,

    /// Indentation width: a number, or "auto" to detect from input (default: auto)
    #[arg(short = 'n', long, default_value = "auto")]
    indent: IndentOption,

    /// Write output back to the input file
    #[arg(short, long)]
    inplace: bool,

    /// Check if files would be modified (exit 1 if so)
    #[arg(short, long)]
    check: bool,
}

const STDIN: &str = "-";

fn is_stdin(path: &PathBuf) -> bool {
    path.as_os_str() == STDIN
}

fn run() -> Result<bool> {
    let args = Args::parse();

    let files = if args.files.is_empty() {
        vec![PathBuf::from(STDIN)]
    } else {
        args.files
    };

    let has_stdin = files.iter().any(is_stdin);

    if args.inplace && has_stdin {
        bail!("--inplace cannot be used with stdin");
    }
    if args.check && has_stdin {
        bail!("--check cannot be used with stdin");
    }

    let sort = args.sort && !args.no_sort;
    let mut would_modify = false;

    for file in &files {
        let input = if is_stdin(file) {
            io::read_to_string(io::stdin())?
        } else {
            fs::read_to_string(file)?
        };

        let output = convert(&input, sort, &args.indent)?;
        let output_with_newline = format!("{output}\n");

        if args.check {
            if input != output_with_newline {
                eprintln!("{}", file.display());
                would_modify = true;
            }
        } else if args.inplace {
            fs::write(file, &output_with_newline)?;
        } else {
            print!("{output_with_newline}");
        }
    }

    Ok(would_modify)
}

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::from(1),
        Ok(false) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("json5-to-json: error: {err}");
            ExitCode::FAILURE
        }
    }
}
