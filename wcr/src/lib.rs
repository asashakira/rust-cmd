use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(about, author, version)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE(s)", default_value = "-")]
    files: Vec<String>,

    /// display line count
    #[arg(short, long, help = "print the newline counts")]
    lines: bool,

    /// display word count
    #[arg(short, long, help = "print the word counts")]
    words: bool,

    /// display char count
    #[arg(short('m'), long, help = "print the character counts")]
    chars: bool,

    /// display byte count
    #[arg(short('c'), long, help = "print the byte counts")]
    bytes: bool,
}

struct FileInfo {
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
    filename: Option<String>,
}

pub fn run() -> MyResult<()> {
    let mut args = Args::parse();

    if [args.words, args.bytes, args.chars, args.lines]
        .iter()
        .all(|v| v == &false)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    // count number flags
    let mut n_flags = 0;
    for flag in [args.words, args.bytes, args.chars, args.lines] {
        if flag {
            n_flags += 1;
        }
    }

    // file infos
    let mut fis: Vec<FileInfo> = Vec::new();

    // le totals
    let mut total = FileInfo {
        lines: 0,
        words: 0,
        chars: 0,
        bytes: 0,
        filename: None,
    };

    // max count number
    let mut max_bytes = 0;

    for filename in &args.files {
        // count
        let mut fi = FileInfo {
            lines: 0,
            words: 0,
            chars: 0,
            bytes: 0,
            filename: Some(filename.to_string()),
        };

        // open each file
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut file) => {
                loop {
                    let mut buf = String::new();
                    let bytes_read = file.read_line(&mut buf)?;
                    if bytes_read == 0 {
                        break;
                    }
                    fi.lines += 1;
                    fi.words += buf.split_whitespace().count();
                    fi.chars += buf.chars().count();
                    fi.bytes += bytes_read;
                    buf.clear();
                }
                max_bytes = std::cmp::max(max_bytes, fi.bytes);

                total.lines += fi.lines;
                total.words += fi.words;
                total.chars += fi.chars;
                total.bytes += fi.bytes;
            }
        }
        fis.push(fi);
    }
    // print width of each count
    let number_width = if args.files.len() > 1 {
        get_width(std::cmp::max(max_bytes, total.bytes))
    } else if n_flags > 1 {
        get_width(max_bytes)
    } else {
        1
    };

    // print all file info
    for fi in fis {
        write_counts(&fi, &args, number_width);
    }

    // print total if multiple files
    if args.files.len() > 1 {
        write_counts(&total, &args, number_width);
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn get_width(mut value: usize) -> usize {
    // calculate width
    let mut width = 0;
    loop {
        if value == 0 {
            return width;
        }
        value /= 10;
        width += 1;
    }
}

fn write_counts(fi: &FileInfo, args: &Args, number_width: usize) {
    // don't want to print initial whitespace when nothing printed before
    // FIXME: there has to be a better way to do this
    let mut first = true;

    if args.lines {
        print!("{:number_width$}", fi.lines);
        first = false;
    }
    if args.words {
        if !first {
            print!(" ");
        }
        print!("{:number_width$}", fi.words);
        first = false;
    }
    if args.chars {
        if !first {
            print!(" ");
        }
        print!("{:number_width$}", fi.chars);
        first = false;
    }
    if args.bytes {
        if !first {
            print!(" ");
        }
        print!("{:number_width$}", fi.bytes);
    }
    if let Some(f) = &fi.filename {
        if f != "-" {
            println!(" {f}");
        } else {
            println!();
        }
    } else {
        println!(" total");
    }
}
