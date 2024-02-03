use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use coloring::palletify_image;
use image::Rgba;
use utils::hex_to_rgb;

mod coloring;
mod utils;

use clap::{arg, command, value_parser};
use std::path::PathBuf;

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(
            arg!(
                -i --image <IMAGE> "Sets the input image file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                --palette <PALETTE> "Sets the palette file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -o --output <OUTPUT> "Sets the output image file name"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let image_path = matches.get_one::<PathBuf>("image").unwrap();

    let palette_path = matches.get_one::<PathBuf>("palette").unwrap();

    let binding = PathBuf::from("palletified_image.png");
    let output_path = matches.get_one::<PathBuf>("output").unwrap_or(&binding);

    let palette = load_palette(palette_path).expect("Failed to load palette");

    if let Err(e) = palletify_image(image_path, &palette, output_path) {
        eprintln!("Error processing image: {}", e);
    }
}

fn load_palette(palette_path: &std::path::PathBuf) -> Result<Vec<Rgba<u8>>, std::io::Error> {
    let file = File::open(palette_path)?;
    let reader = BufReader::new(file);
    let mut palette = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') {
            if let Ok(color) = hex_to_rgb(&line) {
                palette.push(color);
            }
        }
    }

    Ok(palette)
}
