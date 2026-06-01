use std::fmt::Debug;

use crate::color::{
    Color,
    ColorType,
};

#[allow(dead_code)]
#[derive(Debug)]
pub(super) enum Palette {
    Custom,
    Test,
}

#[derive(Debug)]
pub struct PaletteData {
    pub primary_foreground: [u8; 3],
    pub primary_background: [u8; 3],

    pub black: [u8; 3],
    pub red: [u8; 3],
    pub green: [u8; 3],
    pub yellow: [u8; 3],
    pub blue: [u8; 3],
    pub magenta: [u8; 3],
    pub cyan: [u8; 3],
    pub white: [u8; 3],

    pub bright_black: [u8; 3],
    pub bright_red: [u8; 3],
    pub bright_green: [u8; 3],
    pub bright_yellow: [u8; 3],
    pub bright_blue: [u8; 3],
    pub bright_magenta: [u8; 3],
    pub bright_cyan: [u8; 3],
    pub bright_white: [u8; 3],
}

impl Palette {
    fn palette(&self) -> PaletteData {
        match self {
            Palette::Custom => palette_custom(),
            Palette::Test => palette_test(),
        }
    }

    pub fn get_color(&self, color: ColorType) -> [u8; 3] {
        let palette = self.palette();

        match color {
            ColorType::PrimaryForeground => palette.primary_foreground,
            ColorType::PrimaryBackground => palette.primary_background,
            ColorType::Rgb(rgb) => [rgb.0, rgb.1, rgb.2],

            ColorType::Normal(color) => match color {
                Color::Black => palette.black,
                Color::Red => palette.red,
                Color::Green => palette.green,
                Color::Yellow => palette.yellow,
                Color::Blue => palette.blue,
                Color::Magenta => palette.magenta,
                Color::Cyan => palette.cyan,
                Color::White => palette.white,
            },

            ColorType::Bright(color) => match color {
                Color::Black => palette.bright_black,
                Color::Red => palette.bright_red,
                Color::Green => palette.bright_green,
                Color::Yellow => palette.bright_yellow,
                Color::Blue => palette.bright_blue,
                Color::Magenta => palette.bright_magenta,
                Color::Cyan => palette.bright_cyan,
                Color::White => palette.bright_white,
            },
        }
    }

    pub fn get_faint_color(&self, color: ColorType) -> [u8; 3] {
        let color = self.get_color(color);
        let background = self.palette().primary_background;

        // Blending with background
        [
            ((color[0] as u16 + background[0] as u16) / 2) as u8,
            ((color[1] as u16 + background[1] as u16) / 2) as u8,
            ((color[2] as u16 + background[2] as u16) / 2) as u8,
        ]
    }
}

fn palette_custom() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [242, 242, 242],
        primary_background: [22, 22, 22],

        black: [44, 44, 44],
        red: [198, 40, 40],
        green: [85, 139, 46],
        yellow: [249, 168, 37],
        blue: [21, 101, 193],
        magenta: [168, 37, 191],
        cyan: [0, 131, 143],
        white: [255, 255, 255],

        bright_black: [44, 44, 44],
        bright_red: [198, 40, 40],
        bright_green: [85, 139, 46],
        bright_yellow: [249, 168, 37],
        bright_blue: [21, 101, 193],
        bright_magenta: [168, 37, 191],
        bright_cyan: [0, 131, 143],
        bright_white: [255, 255, 255],
    }
}

fn palette_test() -> PaletteData {
    PaletteData {
        // primary_background: "0x161616".parse().unwrap()
        // primary_foreground: "0xf2f2f2".parse().unwrap()
        primary_foreground: [0, 0, 0],
        primary_background: [255, 255, 255],

        black: [0, 0, 0],
        red: [255, 0, 0],
        green: [0, 255, 0],
        yellow: [249, 168, 37],
        blue: [0, 0, 255],
        magenta: [168, 37, 191],
        cyan: [0, 131, 143],
        white: [255, 255, 255],

        bright_black: [44, 44, 44],
        bright_red: [198, 40, 40],
        bright_green: [85, 139, 46],
        bright_yellow: [249, 168, 37],
        bright_blue: [21, 101, 193],
        bright_magenta: [168, 37, 191],
        bright_cyan: [0, 131, 143],
        bright_white: [255, 255, 255],
    }
}
