mod ascii;

extern crate reqwest;
extern crate serde_json;

use serde_json::{Result as SerdeResult, Value};
use std::collections::HashMap;
use std::error::Error;
use std::io::{Error as IOError, ErrorKind, Write};
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use ascii::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stderr = StandardStream::stderr(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;

    let opt = Opt::from_args();

    let image_opt: Vec<&str> = opt.image.split(":").collect();

    let image_url = get_image_url(&image_opt)?;
    // get_image(image_url);

    // // Load image
    // let mut a = Ascii::new(&opt)?;
    // // Convert image to ASCII
    // let output = a.run()?;

    // stdout.flush()?;

    // display(&output, &opt, &mut stdout)?;

    Ok(())
}

fn get_image_url(image_opt: &Vec<&str>) -> Result<String, Box<dyn Error>> {
    match image_opt[0] {
        "xkcd" => {
            let url = format!("https://xkcd.com/{}/info.0.json", image_opt[1]);
            let res_text = reqwest::blocking::get(&url)?.text()?;
            let res: Value = serde_json::from_str(&res_text)?;
            Ok(res["img"].to_string())
        }
        _ => Err(Box::new(IOError::new(ErrorKind::Other, "unknown source"))),
    }
}

fn display(
    output: &AsciiOutput,
    opt: &Opt,
    stdout: &mut StandardStream,
) -> Result<(), Box<dyn Error>> {
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
            write!(stdout, "{}", col.0)?;
        }
        writeln!(stdout, "")?;
    }

    Ok(())
}
