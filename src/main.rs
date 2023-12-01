use std::os::windows;

use image::{GenericImage, Rgba, RgbaImage};


const SIZE_Z: usize = 128;
const SIZE_X: usize = 128;

struct Raindrop{
    r : f64,
    pos : [f64; 2],
}


fn main() {
    println!("Raindrops");

    let mut window : [[u8; SIZE_X]; SIZE_Z] = [[0;SIZE_X];SIZE_Z];

    window[3][24] = 255;

    let mut raindrops : Vec<Raindrop> = vec!();

    step(&mut raindrops);

}

fn step(raindrops : &mut Vec<Raindrop>) {

    //insert new raindrops
        //lets say 10 or something of random velocity 

    //move // dz/dt = -α*r, dx/dt = f(t)
    

    //merge


    todo!()
}

fn save_window(fp : &str, window : [[u8; SIZE_X]; SIZE_Z]){
    // Create an RGBA image
    let mut img = RgbaImage::new(SIZE_X as u32, SIZE_Z as u32);

    // Populate the image with data from the 2D array
    for (y, row) in window.iter().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            img.put_pixel(x as u32, y as u32, Rgba([pixel, pixel, pixel, 255]));
        }
    }

    // Save the image to a file
    img.save(fp).expect("Failed to save image");

}