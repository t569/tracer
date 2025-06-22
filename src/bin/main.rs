fn main() {
    
    // image

    let image_height = 256;
    let image_width = 256;

    
    for i in 0..image_height{
        for j in 0..image_width{
            // Generate a pixel value based on its position
            let r = ((i as f32 / image_height as f32) * 255.0) as u8;
            let g = ((j as f32 / image_width as f32) * 255.0) as u8;
            let b = 0.0;
            
            // // Print the pixel value in grayscale format
            print!("{:3} {:3} {:3} ", r, g, b);
            

        }
    }
}
