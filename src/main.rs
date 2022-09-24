use std::fmt;
use std::ops;
use image::{ImageBuffer};

const SCALE: u32 = 4;

const WIDTH: u32 = 1920 * SCALE/2;
const HEIGHT: u32 = 1080 * SCALE/2;

//Complex number struct
#[derive(Copy, Clone)]
struct Complex {
    real: f64,
    imag: f64,
}

// Chane output for complex numbers
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

impl Complex {
    fn abs(&self) -> f64 {
        (self.real*self.real+self.imag*self.imag).sqrt()
    }
}

fn main() {

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

        (((rgb_x.0 + m)*255.0) as u8, ((rgb_x.1+m)*255.0) as u8, ((rgb_x.2+m)*255.0) as u8)
    }

    fn rgb_convert(i: u16) -> (u8, u8, u8) {
        //println!("i = {}, Input to hsv_to_rgb = {:?}", i, ((i as f64/255.0) as u16, 1.0, 0.5));
        hsv_to_rgb(((i as f64/255.0) as u16, 1.0, 0.5))
    }

    fn mandelbrot(x: f64, y: f64) -> (u8, u8, u8) {
        let c0: Complex = Complex { real: (x), imag: (y) };
        let mut c: Complex = Complex { real: (0.0), imag: (0.0) };
        for i in 1..=1000 {
            // println!("{}", i);
            if c.abs() > 20.0 {
                // println!("{:?}", rgb_convert(i));
                return rgb_convert(i);
            }
            c = c * c + c0;
            // println!("Current c={}", c);
        }
        (0,0,0)
    }

    let mut img = ImageBuffer::new(WIDTH, HEIGHT);
    let mut count: u32 = 0;
    let mut prev: u32 = 0;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        if (x+y)%100 == 0 && prev != (count*100/(WIDTH*HEIGHT)) {
            prev = count*100/(WIDTH*HEIGHT);
            println!("Done: {}%", prev);

        }
        let mandelx: f64 = (x as f64 - (0.75 * WIDTH as f64)) / (WIDTH as f64 / 4.0);
        let mandely: f64 = (y as f64 - (WIDTH as f64 / 4.0)) / (WIDTH as f64 / 4.0);
        let rgb: (u8, u8, u8) = mandelbrot(mandelx, mandely);
        *pixel = image::Rgb([rgb.0,rgb.1,rgb.2]);
        count = count + 1;
    }

    img.save("test.bmp").unwrap();

}