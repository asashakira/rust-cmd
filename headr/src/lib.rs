use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(about, author, version)]
struct Args {
    /// Input file(s)
    #[arg(value_name("FILE"), default_value("-"))]
    files: Vec<String>,

    /// Number of lines
    #[arg(
        short = 'n',
        long,
        default_value = "10",
        value_name = "LINES",
        allow_hyphen_values = true,
        help = "print the first NUM lines instead of the first 10; with the leading '-', print all but the last NUM lines of each file",
        value_parser = clap::value_parser!(i64),
    )]
    lines: i64,

    /// Number of bytes
    #[arg(
        short('c'),
        long,
        value_name("BYTES"),
        allow_hyphen_values(true),
        help("print the first NUM bytes of each file; with the leading '-', print all but the last NUM bytes of each file"),
        conflicts_with("lines"),
        value_parser(clap::value_parser!(i64)),
    )]
    bytes: Option<i64>,
}

fn write_header(filename: &str, is_first_file: bool) {
    println!(
        "{}==> {} <==",
        if is_first_file { "" } else { "\n" },
        filename,
    );
}

pub fn run() -> MyResult<()> {
    let args = Args::parse();
    let num_files = args.files.len();
    for (file_index, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut file) => {
                if num_files > 1 {
                    let is_first_file = file_index == 0;
                    write_header(filename, is_first_file);
                }

                if let Some(num_bytes) = args.bytes {
                    let mut buffer = vec![0; num_bytes as usize];
                    let bytes = file.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes]));
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
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
