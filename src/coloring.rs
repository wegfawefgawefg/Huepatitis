use crate::utils::find_closest_palette_color;
use image::{Rgba, RgbaImage};

pub fn palletify_image(
    image_path: &std::path::PathBuf,
    palette: &[Rgba<u8>],
) -> Result<image::RgbaImage, image::ImageError> {
    let img = image::open(image_path)?.into_rgba8();
    let (width, height) = img.dimensions();

    let mut new_img = RgbaImage::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels() {
        let closest_color = find_closest_palette_color(*pixel, palette);
        new_img.put_pixel(x, y, closest_color);
    }

    Ok(new_img)
}
