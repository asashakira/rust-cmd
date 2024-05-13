use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, author, version)]
struct Args {
    #[arg(value_name = "TEXT", help = "Input text")]
    text: Vec<String>,

    #[arg(short = 'n', long, help = "do not output the trailing newline")]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
}
