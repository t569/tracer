use crate::core::Vec3;


pub fn write_color(pixel_color: &Vec3<f32>, samples_per_pixel: u32) -> String {
    // Scale the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f32;
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();

    // Convert to 8-bit color values
    let r = (r * 255.999) as u8;
    let g = (g * 255.999) as u8;
    let b = (b * 255.999) as u8;

    format!("{} {} {}\n", r, g, b)
}