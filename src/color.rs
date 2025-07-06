use crate::{core::Vec3, Interval};


pub fn write_color(pixel_color: &Vec3<f64>) -> String {
    // Scale the color by the number of samples
    // let r = (pixel_color.x() * scale).sqrt();
    // let g = (pixel_color.y() * scale).sqrt();
    // let b = (pixel_color.z() * scale).sqrt();

    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Convert to 8-bit color values

    // make the final color values bound [0, 1] subsquently [0, 255]
    let intensity = Interval::new(0.000, 0.999);
    let r = (intensity.clamp(*r)* 255.999) as u8;
    let g = (intensity.clamp(*g)* 255.999) as u8;
    let b = (intensity.clamp(*b)* 255.999) as u8;
    
    format!("{} {} {}\n", r, g, b)
}