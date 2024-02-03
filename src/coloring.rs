use crate::utils::find_closest_palette_color;
use image::{Rgba, RgbaImage};
use rayon::prelude::*;

pub fn palletify_image(
    image_path: &std::path::PathBuf,
    palette: &[Rgba<u8>],
    output_path: &std::path::PathBuf,
) -> Result<(), image::ImageError> {
    let img = image::open(image_path)?.into_rgba8();
    let (width, height) = img.dimensions();

    let mut new_img = RgbaImage::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels() {
        let closest_color = find_closest_palette_color(*pixel, palette);
        new_img.put_pixel(x, y, closest_color);
    }

    new_img.save(output_path)?;
    Ok(())
}
