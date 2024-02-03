use image::Rgba;

pub fn hex_to_rgb(hex_color: &str) -> Result<Rgba<u8>, std::num::ParseIntError> {
    let r = u8::from_str_radix(&hex_color[1..3], 16)?;
    let g = u8::from_str_radix(&hex_color[3..5], 16)?;
    let b = u8::from_str_radix(&hex_color[5..7], 16)?;
    Ok(Rgba([r, g, b, 255])) // Assuming alpha is always 255
}

pub fn find_closest_palette_color(color: Rgba<u8>, palette: &[Rgba<u8>]) -> Rgba<u8> {
    *palette
        .iter()
        .min_by_key(|&palette_color| {
            let dr = palette_color[0] as i32 - color[0] as i32;
            let dg = palette_color[1] as i32 - color[1] as i32;
            let db = palette_color[2] as i32 - color[2] as i32;
            dr * dr + dg * dg + db * db
        })
        .unwrap_or(&Rgba([0, 0, 0, 255]))
}
