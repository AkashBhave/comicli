mod ascii;
mod opt;

extern crate reqwest;
extern crate serde_json;

use rand::{thread_rng, Rng};
use reqwest::StatusCode;
use serde_json::Value;
use std::error::Error;
use std::io::Write;
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use ascii::*;
use opt::Opt;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal settings for stdout and stderr
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stderr = StandardStream::stderr(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;

    let opt = Opt::from_args();

    let image_opt: Vec<&str> = opt.image.split(":").collect();

    let image_url = get_image_url(&image_opt)?;
    let image_buf = get_image_buf(image_url)?;

    // Load image
    let mut a = Ascii::new(&opt, image_buf)?;
    // Convert image to ASCII
    let output = a.run()?;

    stdout.flush()?;

    // Display the ASCII characters to stdout
    display(&output, &opt, &mut stdout)?;

    Ok(())
}

fn get_image_url(image_opt: &Vec<&str>) -> Result<String, String> {
    match image_opt[0] {
        "xkcd" => {
            // Either use comic with specified ID or random ID
            let url: String;
            if image_opt.len() > 1 {
                url = format!("https://xkcd.com/{}/info.0.json", image_opt[1]);
            } else {
                // Make the request to xkcd
                let res = reqwest::blocking::get("https://xkcd.com/info.0.json")
                    .map_err(|e| format!("HTTP error: {}", e))?;
                // Hacky method of error handling, but errors shouldn't occur here
                let res_val: Value =
                    serde_json::from_str(&res.text().expect("error")).expect("error");
                // Get the most recent comic's ID
                let max_id = res_val["num"].as_u64().unwrap();

                // Generate a random ID
                let mut rng = thread_rng();
                let id: u64 = rng.gen_range(1, max_id);

                url = format!("https://xkcd.com/{}/info.0.json", id.to_string());
            }

            // Make the request to xkcd
            let res = reqwest::blocking::get(&url).map_err(|e| format!("HTTP error: {}", e))?;
            // Check if ID is invalid
            if res.status() == StatusCode::NOT_FOUND {
                return Err(String::from("unknown comic ID"));
            }

            // Hacky method of error handling, but errors shouldn't occur here
            let res_val: Value = serde_json::from_str(&res.text().expect("error")).expect("error");
            let image_str = res_val["img"].as_str().unwrap(); // "img" is a key of JSON response
            Ok(String::from(image_str))
        }
        _ => Err(String::from("unknown source")),
    }
}

fn get_image_buf(image_url: String) -> Result<Vec<u8>, Box<dyn Error>> {
    // Fetch the actual image
    let mut res = reqwest::blocking::get(&image_url)?;

    // Dump the image data to a vector buffer
    let mut res_vec: Vec<u8> = vec![];
    res.copy_to(&mut res_vec)?;
    Ok(res_vec)
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
