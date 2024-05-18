use anyhow::{anyhow, Result};
use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(about, author, version)]
struct Args {
    /// Input file
    #[arg(value_name = "INPUT", default_value = "-")]
    infile: String,

    /// Output file
    #[arg(value_name = "OUTPUT")]
    outfile: Option<String>,

    /// Show counts
    #[arg(short, long, help = "prefix lines by the number of occurrences")]
    count: bool,
}

pub fn run() -> MyResult<()> {
    let args: Args = Args::parse();
    let mut file = open(&args.infile).map_err(|e| anyhow!("{}: {e}", args.infile))?;
    let mut outfile: Box<dyn Write> = match &args.outfile {
        Some(outname) => Box::new(File::create(outname)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |num: u64, text: &str| -> Result<()> {
        if num > 0 {
            if args.count {
                writeln!(outfile, "{num:7} {text}")?;
            } else {
                writeln!(outfile, "{text}")?;
            }
        };
        Ok(())
    };

    let mut prev_line = String::new();
    let mut count: u64 = 0;
    loop {
        let mut line = String::new();
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != prev_line.trim_end() {
            print(count, prev_line.trim_end())?;
            count = 0;
            prev_line = line;
        }
        count += 1;
    }
    print(count, prev_line.trim_end())?;

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
