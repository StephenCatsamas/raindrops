use std::ops::{Index,Add, Sub, Mul, Neg, Deref, DerefMut};

use image::{GenericImage, Rgba, RgbaImage};


const SIZE_Z: usize = 128;
const SIZE_X: usize = 128;
const a     : f64   = 0.1;



#[derive(Debug, PartialEq, Clone, Copy)]
struct Point{
    p : [f64;2]
}

macro_rules! point {
    ($x:expr, $y:expr) => {
        Point { p: [$x, $y] }
    };
}

impl Point {
    fn norm(self) -> f64 {
        self.p.iter().fold(0.0f64, |s, v| s + v*v).sqrt()
    }
}


impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            p: [self*other.p[0], self*other.p[1]],
        }
    }
} 

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        -1.0f64 * self
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            p: [self.p[0] + other.p[0], self.p[1] + other.p[1]],
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        self + -other
    }
}

impl Index<usize> for Point {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.p[index]
    }
}

struct Raindrop{
    r : f64,
    pos : Point,
}

fn main() {
    println!("Raindrops");

    let mut window : [[u8; SIZE_X]; SIZE_Z] = [[0;SIZE_X];SIZE_Z];

    window[3][24] = 255;

    let mut raindrops : Vec<Raindrop> = vec!();

    step(&mut raindrops, 0.0f64);

}

fn step(raindrops : &mut Vec<Raindrop>, f : f64) {

    //insert new raindrops
        //lets say 10 or something of random velocity 

    //move // dz/dt = -α*r, dx/dt = f(t)
    raindrops.iter_mut()
             .for_each(
             |drop| 
                drop.pos = drop.pos + point![f, -a*drop.r]
             );
    raindrops.retain(|drop| drop.pos[0] > -1.0);


    //merge
    for (i, &x) in raindrops.iter().enumerate() {
        for &y in raindrops.iter().skip(i + 1) {
            
        }
    }

    todo!()
   
}

fn collision(drop_a : &Raindrop, drop_b : &Raindrop) -> bool{
    (drop_a.pos - drop_b.pos).norm() <= drop_a.r + drop_b.r
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