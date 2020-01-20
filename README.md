# ComiCLI

A webcomic to ASCII art program written in Rust.
Also a fun way to practice the language.

## Usage

```
comicli [FLAGS] [OPTIONS] <image>
```

For example, viewing XKCD comic 500 with color would be:
```sh
comicli -c xkcd:500
```

### Flags
```
    --bg
-b, --braille
-c, --color
    --help       Prints help information
-V, --version    Prints version information
```

### Options
```
-d, --depth <depth>       [default: 70]
-h, --height <height>
-w, --width <width>       [default: 80]
```

### Arguments
```
<image>         <comic source>:<comic ID>
```

## Structure

* */src/main.rs*: Runs the actual program
* */src/opt.rs*: Contains information about command line usage
* */src/ascii.rs*: Handles converting between and image and an ASCII output, adopted from [@ajmwagar/rascii](https://github.com/ajmwagar/rascii)

## Supported Sources

Currently, ComiCLI only supports pulling images from xkcd. Support for more sources might be added in the future.

## Development

0. Ensure you have Rust and Cargo installed
1. Clone this repository
2. Edit files as necessary
3. Run `cargo run` with parameters to test
4. Run `cargo build --release` to build for production
