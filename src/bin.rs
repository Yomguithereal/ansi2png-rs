use clap::Parser;
use std::{
    io::Read,
    path::PathBuf,
};

use ansi2png::Ansi2PngSettings;

/// Convert ansi output to pngs
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Opt {
    /// Path to the input file. File should contain utf8 text that uses ANSI
    /// escape codes.
    input_path: Option<PathBuf>,

    /// Path to output file. Will always write a png regardless of file
    /// extenstion.
    #[structopt(short, long)]
    output_path: PathBuf,

    /// Maximum width of the png
    #[structopt(short, long)]
    png_width: Option<u32>,

    /// Horizontal padding, in pixels
    #[structopt(long, default_value = "20")]
    horizontal_padding: u32,

    /// Vertical padding, in pixels
    #[structopt(long, default_value = "20")]
    vertical_padding: u32,
}

fn main() {
    let opt = Opt::parse();

    let settings = Ansi2PngSettings {
        png_width: opt.png_width,
        horizontal_padding: opt.horizontal_padding,
        vertical_padding: opt.vertical_padding,
    };

    let input: Box<dyn Read> = match &opt.input_path {
        Some(p) if !matches!(p.to_str(), Some(s) if s == "=") => {
            Box::new(std::fs::File::open(p).unwrap())
        }
        _ => Box::new(std::io::stdin()),
    };

    let input = std::io::BufReader::new(input);

    settings.to_png(input).save(opt.output_path).unwrap();
}
