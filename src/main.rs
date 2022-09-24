use std::fmt;
use std::ops;
use std::time::{Duration, Instant};
use image::{ImageBuffer};

// Scale the image (default 1920*1080)
const SCALE: u32 = 1;

// Configure width and height of image
const WIDTH: u32 = 1920 * SCALE/2;
const HEIGHT: u32 = 1080 * SCALE/2;

// mixup RGB values (value between 0 and 0.5)
const MIXUP: f64 = 0.2;

//Complex number struct
#[derive(Copy, Clone)]
struct Complex {
    real: f64,
    imag: f64,
}

// Change output for complex numbers
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

// Change + Operator for complex numbers
impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        let mut new: Complex = Complex { real: (0.0), imag: (0.0) };
        new.real = self.real + other.real;
        new.imag = self.imag + other.imag;

        new
    }
}

impl ops::Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        let mut new: Complex = Complex { real: (0.0), imag: (0.0) };
        new.real = self.real - other.real;
        new.imag = self.imag - other.imag;

        new
    }
}

impl ops::Mul<Complex> for Complex {
    
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        let mut new: Complex = Complex { real: (0.0), imag: (0.0) };
        new.real = self.real * other.real - self.imag * other.imag;
        new.imag = self.real * other.imag + self.imag * other.real;

        new
    } 
}

impl ops::Div<Complex> for Complex {
    
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let mut new: Complex = Complex { real: (0.0), imag: (0.0) };
        new.real = self.real/(self.real*self.real+self.imag*self.imag);
        new.imag = -(self.imag/(self.real*self.real+self.imag*self.imag));
        new = new * other;
        
        new
    } 
}

// Implement abs() method for Complex numbers
impl Complex {
    fn abs(&self) -> f64 {
        (self.real*self.real+self.imag*self.imag).sqrt()
    }
}

fn hsv_to_rgb(hsv: (u16, f64, f64)) -> (u8, u8, u8) {
    let c: f64 = hsv.1 * hsv.2;
    let a: f64 = (hsv.0 as f64/60.0)%2.0-1.0;
    let x: f64 = c * (1.0-(a).abs());
    let m: f64 = hsv.2 - c;
    let rgb_x: (f64, f64, f64);
    match hsv.0{
        0..=59 => rgb_x = (c, x, 0.0),
        60..=119 => rgb_x = (x, c, 0.0),
        120..=179 => rgb_x = (0.0, c, x),
        180..=239 => rgb_x = (0.0, x, c),
        240..=299 => rgb_x = (x, 0.0, c),
        300..=359 => rgb_x = (c, 0.0, x),
        _ => panic!("Error: This H value is not possible.")
    }

    let mut fin: (u8, u8, u8) = (((rgb_x.0 + m)*255.0) as u8, ((rgb_x.1+m)*255.0) as u8, ((rgb_x.2+m)*255.0) as u8);
    fin = ((fin.0 as f64*MIXUP + fin.1 as f64*(1.0-MIXUP)) as u8, 
        (fin.1 as f64*MIXUP + fin.2 as f64*(1.0-MIXUP)) as u8, 
        (fin.2 as f64*MIXUP + fin.0 as f64*(1.0-MIXUP)) as u8);

    fin
}

fn rgb_convert(i: u16) -> (u8, u8, u8) {
    hsv_to_rgb((i%360, 1.0, 0.5))
}

fn mandelbrot(x: f64, y: f64) -> (u8, u8, u8) {
    let c0: Complex = Complex { real: (x), imag: (y) };
    let mut c: Complex = Complex { real: (0.0), imag: (0.0) };
    for i in 1..=10000 {
        if c.abs() > 2.0 {
            // println!("{:?}", rgb_convert(i));
            return rgb_convert(i);
        }
        c = c * c + c0;
        // println!("Current i= {}, Current c={}", i, c);
    }
    (0,0,0)
}

fn main() {
    // Stop time
    let now = Instant::now();
    // Create Image Buffer, Counter and prev-variable which holds current percentage
    let mut img = ImageBuffer::new(WIDTH, HEIGHT);
    let mut count: u32 = 0;
    let mut prev: u32 = 0;
    let mut curr: u32;

    // Iterate through all pixels and createa mandelbrot fractal
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // Print percentage if percentage changes
        curr = count*100/(WIDTH*HEIGHT);
        if (x+y)%100 == 0 && prev != curr {
            prev = curr;
            println!("Running: {}%", prev);
            // println!("Time elapsed in s: {}", now.elapsed().as_secs_f64());
        }
        let mandelx: f64 = (x as f64 - (0.75 * WIDTH as f64)) / (WIDTH as f64 / 4.0);
        let mandely: f64 = (y as f64 - (WIDTH as f64 / 4.0)) / (WIDTH as f64 / 4.0);
        // Run mandelbrot
        let rgb: (u8, u8, u8) = mandelbrot(mandelx, mandely);
        //println!("X={}, MandelX={}, Y={}, MandelY{}, RGB={:?}", x, mandelx, y, mandely, rgb);
        // Add Pixel to image
        *pixel = image::Rgb([rgb.0,rgb.1,rgb.2]);
        count = count + 1;
    } 

    // Save image in file
    img.save("mandel.bmp").unwrap();
    println!("Done: 100%");
    println!("Time elapsed total in s: {}", now.elapsed().as_secs_f64());
}