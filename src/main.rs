use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter},
    path::PathBuf,
};

use clap::{arg, command, value_parser};
use png::{BitDepth, ColorType, Encoder};

use coloring::palletify_image;
use image::{Rgba, RgbaImage};
use utils::hex_to_rgb;

mod coloring;
mod utils;

fn main() {
    let matches = command!()
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
        .arg(arg!(
            -n --notransparency "Disables transparency in the palette"
        ))
        .get_matches();

    let image_path = matches.get_one::<PathBuf>("image").unwrap();

    let palette_path = matches.get_one::<PathBuf>("palette").unwrap();

    let no_transparency: bool = *matches.get_one::<bool>("notransparency").unwrap_or(&false);

    let binding = PathBuf::from("palletified_image.png");
    let output_path = matches.get_one::<PathBuf>("output").unwrap_or(&binding);

    let palette = load_palette(palette_path, no_transparency).expect("Failed to load palette");

    match palletify_image(image_path, &palette) {
        Ok(image) => {
            if let Err(e) = save_compressed_image(&image, output_path) {
                eprintln!("Failed to save compressed image: {}", e);
            }
        }
        Err(e) => eprintln!("Error during image processing: {}", e),
    }
}

fn save_compressed_image(image: &RgbaImage, output_path: &PathBuf) -> Result<(), std::io::Error> {
    let (width, height) = image.dimensions();

    let file = File::create(output_path)?;
    let w = &mut BufWriter::new(file);

    let mut encoder = Encoder::new(w, width, height);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);
    encoder.set_compression(png::Compression::Default);
    encoder.set_filter(png::FilterType::Sub);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(image.as_ref())?; // as_ref() to borrow the image data as &[u8]

    Ok(())
}

fn load_palette(
    palette_path: &std::path::PathBuf,
    no_transparency: bool,
) -> Result<Vec<Rgba<u8>>, std::io::Error> {
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
    // push transparent black also, unless explicitly disabled
    if !no_transparency {
        palette.push(Rgba([0, 0, 0, 0]));
    }

    Ok(palette)
}
