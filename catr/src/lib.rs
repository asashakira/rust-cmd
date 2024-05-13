use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(about, author, version)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(short('n'), long("number"), help = "number all output lines")]
    number: bool,

    /// Number non-blank lines
    #[arg(
        short('b'),
        long("number-nonblank"),
        help("number nonempty output lines, overrides -n")
    )]
    number_nonblank: bool,
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();
    let mut prev_num = 0;
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                for line in file.lines() {
                    let line = line?;
                    if args.number_nonblank {
                        if line.is_empty() {
                            println!();
                        } else {
                            prev_num += 1;
                            println!("{prev_num:6}\t{line}");
                        }
                    } else if args.number {
                        prev_num += 1;
                        println!("{prev_num:6}\t{line}");
                    } else {
                        println!("{line}");
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
