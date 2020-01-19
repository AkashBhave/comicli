// Adopted from https://github.com/ajmwagar/rascii [MIT]

use image::{DynamicImage, RgbImage};
use std::error::Error;
use std::io::Write;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

// 10 levels of grayscale
const GSCALE_10: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
const GSCALE_70: &str = " .\"`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
const GAMMA: f64 = 2.2;

type AsciiOutput = Vec<Vec<(char, AsciiColor)>>;

#[derive(Debug)]
enum AsciiColor {
    RGB(u8, u8, u8),
    Grayscale(u8),
}

impl AsciiColor {
    fn to_grayscale(&self) -> u8 {
        // Rlin = R^GAMMA,  Glin = G^GAMMA,  Blin = B^GAMMA
        // Y = .2126 * R^GAMMA + .7152 * G^GAMMA + .0722 * B^GAMMA
        // L* = 116 * Y ^ 1/3 - 16

        match self {
            AsciiColor::RGB(r, g, b) => {
                let rlin = (*r as f64).powf(GAMMA);
                let blin = (*b as f64).powf(GAMMA);
                let glin = (*g as f64).powf(GAMMA);

                let y = (0.2126 * rlin) + (0.7152 * glin) + (0.0722 * blin);

                let l = (116.0 * y.powf(1.0 / 3.0) - 16.0) as u8;
                l
            }
            AsciiColor::Grayscale(l) => *l,
        }
    }
}

// Image to ASCII converter
#[derive(StructOpt, Debug)]
#[structopt(name = "comicli")]
struct Opt {
    // Enable colored output
    #[structopt(short = "c", long = "color")]
    color: bool,

    // Enable braille mode
    #[structopt(short = "b", long = "braille")]
    braille: bool,

    #[structopt(short = "w", long = "width", default_value = "80")]
    // Width in characters of the output
    width: u32,

    #[structopt(short = "d", long = "depth", default_value = "70")]
    // Lumince depth to use. (Number of unique characters)
    depth: u8,

    #[structopt(short = "h", long = "height")]
    // Height in characters of the output
    height: Option<u32>,

    #[structopt(long = "bg")]
    // Enable coloring of background chars
    bg: bool,

    // Path of image file to convert
    #[structopt(name = "IMAGE", parse(from_os_str))]
    image: PathBuf,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;

    let opt = Opt::from_args();

    // Load image
    let mut a = Ascii::from_opt(&opt)?;
    // Convert image to ASCII
    let output = a.run()?;

    stdout.flush()?;

    for row in output {
        for col in row {
            if opt.color {
                let (r, g, b) = match col.1 {
                    AsciiColor::RGB(r, g, b) => (r, g, b),
                    _ => (0, 0, 0),
                };

                if opt.bg {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(
                        255 - r,
                        255 - g,
                        255 - b,
                    ))))?;
                    stdout.set_color(ColorSpec::new().set_bg(Some(Color::Rgb(r, g, b))))?;
                } else {
                    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(r, g, b))))?;
                }
            }
            write!(&mut stdout, "{}", col.0)?;
        }
        writeln!(&mut stdout, "")?;
    }

    Ok(())
}

struct Ascii {
    // Image
    pub image: RgbImage,
    // Image dimensions
    pub dim: (u32, u32),
    // AsciiColored output
    pub color: bool,
    pub depth: u8,
    // Enable braille mode
    pub braille: bool,
}

impl Ascii {
    // Convert CLI options to a Ascii instance
    pub fn from_opt(opt: &Opt) -> Result<Self, Box<dyn Error>> {
        let im: DynamicImage = image::open(&Path::new(&opt.image))?;
        let im = im.to_rgb();
        let aspect = im.height() as f64 / im.width() as f64;
        let height = match opt.height {
            Some(height) => height,
            None => (opt.width as f64 * aspect) as u32,
        };

        Ok(Ascii {
            image: im,
            dim: (opt.width, height),
            depth: opt.depth,
            color: opt.color,
            braille: opt.braille,
        })
    }

    // Convert the image to rascii based on the settings provided
    pub fn run(&mut self) -> Result<AsciiOutput, Box<dyn Error>> {
        let mut output: AsciiOutput = Vec::new();
        // Dimensions of image
        let (width, height) = self.image.dimensions();

        // Get tile dimensions
        let tile_w = width / self.dim.0 as u32;
        let tile_h = height / self.dim.1 as u32;

        // Convert image to image chunks based on dimensions.
        for ty in 1..self.dim.1 - 1 {
            let mut row_tiles = Vec::new();

            for tx in 1..self.dim.0 - 1 {
                let mut tile_pixel_data = Vec::with_capacity((tile_w * tile_h) as usize);
                // per tile
                for px in 0..tile_w {
                    for py in 0..tile_h {
                        let pixel_data = self
                            .image
                            .get_pixel(px + (tx * tile_w), py + (ty * tile_h))
                            .data;

                        let color: AsciiColor;
                        if self.color {
                            color = AsciiColor::RGB(pixel_data[0], pixel_data[1], pixel_data[2])
                        } else {
                            let y = AsciiColor::RGB(pixel_data[0], pixel_data[1], pixel_data[2])
                                .to_grayscale();
                            color = AsciiColor::Grayscale(y as u8);
                        }

                        tile_pixel_data.push(color);
                    }
                }

                let avg: AsciiColor;
                let ascii_char: char;
                if self.color {
                    avg = AsciiColor::RGB(
                        (tile_pixel_data.iter().fold(0usize, |sum, x| {
                            sum + match x {
                                AsciiColor::RGB(r, _, _) => *r as usize,
                                _ => 0,
                            }
                        }) / tile_pixel_data.len()) as u8,
                        (tile_pixel_data.iter().fold(0usize, |sum, x| {
                            sum + match x {
                                AsciiColor::RGB(_, g, _) => *g as usize,
                                _ => 0,
                            }
                        }) / tile_pixel_data.len()) as u8,
                        (tile_pixel_data.iter().fold(0usize, |sum, x| {
                            sum + match x {
                                AsciiColor::RGB(_, _, b) => *b as usize,
                                _ => 0,
                            }
                        }) / tile_pixel_data.len()) as u8,
                    );
                    if self.depth > 10 {
                        let index = (avg.to_grayscale() as f64 / 255.0) * 67.0;
                        let chars = GSCALE_70.chars().collect::<Vec<char>>();
                        ascii_char = chars[index as usize];
                    } else {
                        let index = (avg.to_grayscale() as f64 / 255.0) * 9.0;
                        ascii_char = GSCALE_10[index as usize];
                    }
                } else {
                    avg = AsciiColor::Grayscale(
                        (tile_pixel_data.iter().fold(0usize, |sum, x| {
                            sum + match x {
                                AsciiColor::Grayscale(x) => *x as usize,
                                _ => 0,
                            }
                        }) as usize
                            / tile_pixel_data.len()) as u8,
                    );
                    let x = match avg {
                        AsciiColor::Grayscale(x) => x,
                        _ => 0,
                    };
                    if self.depth > 10 {
                        let index = (x as f64 / 255.0) * 67.0;
                        let chars = GSCALE_70.chars().collect::<Vec<char>>();
                        ascii_char = chars[index as usize];
                    } else {
                        let index = (x as f64 / 255.0) * 9.0;
                        ascii_char = GSCALE_10[index as usize];
                    }
                }

                row_tiles.push((ascii_char, avg));
            }

            output.push(row_tiles);
        }
        Ok(output)
    }
}
