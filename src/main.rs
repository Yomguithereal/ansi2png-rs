use clap::Parser as _;
use image::RgbImage;
use include_flate::flate;
use rusttype::{
    Font,
    Scale,
};
use std::io::Read;
use vte::Parser;

mod color;
mod escape;
mod opt;
mod palette;
mod printer;

use crate::{
    opt::Opt,
    palette::Palette,
    printer::Settings,
};

#[cfg(feature = "font-style")]
fn load_fonts() -> (Font<'static>, Font<'static>, Font<'static>, Font<'static>) {
    flate!(static FONT: [u8] from
        "fonts/iosevka-term-extended.ttf");

    flate!(static FONT_BOLD: [u8] from
        "fonts/iosevka-term-extendedbold.ttf");

    flate!(static FONT_ITALIC: [u8] from
        "fonts/iosevka-term-extendeditalic.ttf");

    flate!(static FONT_ITALIC_BOLD: [u8] from
        "fonts/iosevka-term-extendedbolditalic.ttf");

    (
        Font::try_from_bytes(&FONT).unwrap(),
        Font::try_from_bytes(&FONT_BOLD).unwrap(),
        Font::try_from_bytes(&FONT_ITALIC).unwrap(),
        Font::try_from_bytes(&FONT_ITALIC_BOLD).unwrap(),
    )
}

#[cfg(not(feature = "font-style"))]
fn load_fonts() -> (Font<'static>, Font<'static>, Font<'static>, Font<'static>) {
    flate!(static FONT: [u8] from
        "fonts/iosevka-term-extended.ttf");

    let font = Font::try_from_bytes(&FONT).unwrap();

    (font.clone(), font.clone(), font.clone(), font)
}

fn main() {
    let opt = Opt::parse();

    let input: Box<dyn Read> = match &opt.input_path {
        Some(p) if !matches!(p.to_str(), Some(s) if s == "=") => {
            Box::new(std::fs::File::open(p).unwrap())
        }
        _ => Box::new(std::io::stdin()),
    };

    let mut input = std::io::BufReader::new(input);

    let (font, font_bold, font_italic, font_italic_bold) = load_fonts();

    let font_height = 50.0;
    let scale = Scale {
        x: font_height,
        y: font_height,
    };

    let palette = Palette::Custom;
    let png_width = opt.png_width;

    let mut statemachine = Parser::new();
    let mut performer = printer::new(Settings {
        font,
        font_bold,
        font_italic,
        font_italic_bold,
        font_height,
        scale,
        palette,
        png_width,
    });

    let mut buf = [0; 2048];

    loop {
        match input.read(&mut buf) {
            Ok(0) => break,

            Ok(n) => {
                for byte in &buf[..n] {
                    statemachine.advance(&mut performer, *byte);
                }
            }

            Err(err) => {
                println!("err: {err}");
                break;
            }
        }
    }

    let image: RgbImage = performer.into();
    image.save(opt.output_path).unwrap();
}
