use clap::Parser;

/// Rust echo CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Input text
    #[arg(value_name = "TEXT", required = true)]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short = 'n', long = "omit-newline", required = false)]
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
