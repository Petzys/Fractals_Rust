use std::fmt;
use std::ops;
use std::time::{Instant};
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use image::{ImageBuffer, RgbImage};

/******* CONFIG *******/

// Scale the image (default 1920*1080, value: 1)
const SCALE: u32 = 2;

// Configure width and height of image
const WIDTH: u32 = 1920 * SCALE.pow(2);
const HEIGHT: u32 = 1080 * SCALE.pow(2);

// mixup RGB values (value between 0 and 0.5), default 1.0
const MIXUP: f64 = 0.3;

// Number of Threads, use 2^x
const THREADS: u32 = 32;
const SPLIT_AFTER: u32 = WIDTH/THREADS;

/******* MAIN *******/

// Main function
fn main() {
    // Stop time
    let now = Instant::now();
    // Create Image Buffer, Counter and prev-variable which holds current percentage
    let mut img: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    // Create sender and receiver for thread channel
    let (sender, receiver): (Sender<ImagePart>, Receiver<ImagePart>) = channel();
    
    // split tasks for threads if picture can be equally split
    if WIDTH%THREADS == 0 {
        println!("Starting Threads...");
        for i in 0..THREADS {
            // clone sender for each thread
            let sender: Sender<ImagePart> = sender.clone();
            println!("Spawning Thread no. [{:2}]", i);
            thread::spawn(move || {
                thread_tasker(i, sender);
            });
        } 

        // receive picture parts from threads and combine them to whole picture
        for _ in 0..THREADS {
            let image_part: ImagePart = receiver.recv().unwrap();
            for (pos, pixel) in image_part.rgb.iter().enumerate() {
                let x = pos as u32%SPLIT_AFTER+image_part.id*SPLIT_AFTER;
                let y = pos as u32/SPLIT_AFTER;
                img.put_pixel(x, y, image::Rgb([pixel.0,pixel.1,pixel.2]));
            }
        }
    }

    /* OLD, SINGLE THREADED VERSION (uncomment to run)
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
        // println!("X={}, MandelX={}, Y={}, MandelY{}, RGB={:?}", x, mandelx, y, mandely, rgb);
        // Add Pixel to image
        *pixel = image::Rgb([rgb.0,rgb.1,rgb.2]);
        count = count + 1;
    } */
    
    // Save image in file
    img.save("mandel.bmp").unwrap();
    println!("Done: 100%");
    println!("Time elapsed total in s: {}", now.elapsed().as_secs_f64());
}

/******* DATA TYPES *******/

//Complex number struct
#[derive(Copy, Clone)]
struct Complex {
    real: f64,
    imag: f64,
}
// Pixel and location in image
#[derive(Clone)]
struct ImagePart {
    id: u32,
    rgb: Vec<(u8, u8, u8)>,
}

/******* OVERRIDE METHODS FOR STRUCTS *******/

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

// Change - Operator for complex numbers
impl ops::Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        let mut new: Complex = Complex { real: (0.0), imag: (0.0) };
        new.real = self.real - other.real;
        new.imag = self.imag - other.imag;

        new
    }
}

// Change * Operator for complex numbers
impl ops::Mul<Complex> for Complex {
    
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        let mut new: Complex = Complex { real: (0.0), imag: (0.0) };
        new.real = self.real * other.real - self.imag * other.imag;
        new.imag = self.real * other.imag + self.imag * other.real;

        new
    } 
}

// Change / Operator for complex numbers
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

    fn pow (self, exp: i32) -> Complex {
        let mut new: Complex = self.clone();
        for _ in 0..exp.abs()-1 {
            new = new*self;
        }

        if exp < 0 {
            new = Complex {real:(1.0), imag:(0.0)}/new;
        }

        new
    }
}

/******* MANDELBROT METHODS *******/

// Convert HSV to RGB values
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

// Converter to use fix S&V
fn rgb_convert(i: u16) -> (u8, u8, u8) {
    hsv_to_rgb((i%360, 1.0, 0.5))
}

// Mandelbrot function
fn mandelbrot(x: f64, y: f64) -> (u8, u8, u8) {
    let c0: Complex = Complex { real: (x), imag: (y) };
    let mut c: Complex = Complex { real: (0.0), imag: (0.0) };
    for i in 1..=10000 {
        if c.abs() > 2.0 {
            // println!("{:?}", rgb_convert(i));
            return rgb_convert(i);
        }
        c = c.pow(2) + c0;
        // println!("Current i= {}, Current c={}", i, c);
    }
    (0,0,0)
}

/******* MULTITHREADER *******/

// Creates tasks for threads
fn thread_tasker(id: u32, sender: Sender<ImagePart>) {
    let mut pix_vector: Vec<(u8, u8, u8)> = vec![];
    let mut count: u32 = 0;
    let mut prev: u32 = 0;
    let mut curr: u32;
    for y in 0..HEIGHT{
        for x in id*SPLIT_AFTER..(id+1)*SPLIT_AFTER{
            // Print percentage if percentage changes
            curr = count*100/(SPLIT_AFTER*HEIGHT);
            if (x+y)%100 == 0 && prev+5 == curr {
                prev = curr;
                println!("Thread [{:2}], Running: {:2}%", id, prev);
                // println!("Time elapsed in s: {}", now.elapsed().as_secs_f64());
            }
            let mandelx: f64 = (x as f64 - (0.75 * WIDTH as f64)) / (WIDTH as f64 / 4.0);
            let mandely: f64 = (y as f64 - (WIDTH as f64 / 4.0)) / (WIDTH as f64 / 4.0);
            // Run mandelbrot
            let rgb: (u8, u8, u8) = mandelbrot(mandelx, mandely);
    
            pix_vector.push(rgb);
            count = count+1;
        }
    } 
    
    sender.send(ImagePart { id: (id), rgb: (pix_vector) }).unwrap();
}