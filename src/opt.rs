use structopt::StructOpt;

// Image to ASCII converter
#[derive(StructOpt, Debug)]
#[structopt(name = "comicli")]
pub struct Opt {
    // Enable colored output
    #[structopt(short = "c", long = "color")]
    pub color: bool,

    // Enable braille mode
    #[structopt(short = "b", long = "braille")]
    pub braille: bool,

    #[structopt(short = "w", long = "width", default_value = "80")]
    // Width in characters of the output
    pub width: u32,

    #[structopt(short = "d", long = "depth", default_value = "70")]
    // Lumince depth to use. (Number of unique characters)
    pub depth: u8,

    #[structopt(short = "h", long = "height")]
    // Height in characters of the output
    pub height: Option<u32>,

    #[structopt(long = "bg")]
    // Enable coloring of background chars
    pub bg: bool,

    // Path of image file to convert
    #[structopt(name = "image")]
    pub image: String,
}
