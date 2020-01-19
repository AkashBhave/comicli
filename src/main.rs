mod ascii;

use std::error::Error;
use std::io::Write;
use structopt::StructOpt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use ascii::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;

    let opt = Opt::from_args();

    // Load image
    let mut a = Ascii::from_opt(&opt)?;
    // Convert image to ASCII
    let output = a.run()?;

    stdout.flush()?;

    display(&output, &opt, &mut stdout)?;

    Ok(())
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
