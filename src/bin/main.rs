use std::fs::File;
use std::io::Write;

fn main() {
    
    // image

    let image_height = 256;
    let image_width = 256;

    // write this all to a file

    // first put all of this into a string
    let mut image_data = String::new();

    image_data.push_str("P3\n");
    image_data.push_str(&format!("{} {}\n", image_width, image_height));
    image_data.push_str("255\n");

    for i in 0..image_height{
        for j in 0..image_width{
            // Generate a pixel value based on its position
            let r = ((i as f32 / image_height as f32) * 255.0) as u8;
            let g = ((j as f32 / image_width as f32) * 255.0) as u8;
            let b = 0.0 as u8; // No blue component
                    
            image_data.push_str(&format!("{} {} {} ", r, g, b));

        }
    }

    write_to_file("renders/image.ppm", &image_data);

}


fn write_to_file(filename: &str, data: &str) {
    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(data.as_bytes()).expect("Unable to write data");


}
