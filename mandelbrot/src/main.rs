extern crate image;
extern crate num;

use image::png::PNGEncoder;
use image::ColorType;
use num::Complex;
use std::env;
use std::fs::File;
use std::time;

const BREAKOUT: f64 = 4.0;
const FILENAME: &str = "mandelbrot.png";
const NUM_LOOPS: usize = 256;

fn main() {
    match parse_args() {
        Some(params) => {
            println!("Running..");

            let start = time::Instant::now();

            let bounds = (params.w, params.h);
            let upper_left = Complex {
                re: params.re_1,
                im: params.im_1,
            };
            let lower_right = Complex {
                re: params.re_2,
                im: params.im_2,
            };

            let pixels = render(bounds, upper_left, lower_right);

            let output = File::create(FILENAME).unwrap();
            let encoder = PNGEncoder::new(output);
            encoder
                .encode(
                    &pixels,
                    bounds.0 as u32,
                    bounds.1 as u32,
                    ColorType::Gray(8),
                )
                .unwrap();
            println!("Done in {:?}. Find output in {}", start.elapsed(), FILENAME);
        }
        None => (),
    }
}

struct Params {
    re_1: f64,
    im_1: f64,
    re_2: f64,
    im_2: f64,
    w: usize,
    h: usize,
}

fn parse_args() -> Option<Params> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        7 => Some(Params {
            re_1: args[1].parse().unwrap(),
            im_1: args[2].parse().unwrap(),
            re_2: args[3].parse().unwrap(),
            im_2: args[4].parse().unwrap(),
            w: args[5].parse().unwrap(),
            h: args[6].parse().unwrap(),
        }),
        1 => {
            println!("Using defaults. Run with \"--help\" for more options");
            Some(Params {
                re_1: -1.0,
                im_1: 1.0,
                re_2: 1.0,
                im_2: -1.0,
                w: 1000,
                h: 1000,
            })
        }
        _ => {
            println!(
                "Program requires 6 args: \
                 left_x upper_y right_x lower_y pixels_w pixels_h\nDefault run is \
                 -1.0 1.0 1.0 -1.0 1000 1000"
            );
            None
        }
    }
}

fn get_complex_zero() -> Complex<f64> {
    Complex { re: 0.0, im: 0.0 }
}

fn render(bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) -> Vec<u8> {
    let mut pixels = Vec::new();
    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            // pixels[row * bounds.0 + col]
            pixels.push(match escape(point) {
                Some(count) => 255 - count as u8,
                None => 0,
            })
        }
    }
    pixels
}

fn escape(c: Complex<f64>) -> Option<usize> {
    let mut z = get_complex_zero();
    for i in 0..NUM_LOOPS {
        z = z * z + c;
        if z.norm_sqr() > BREAKOUT {
            return Some(i);
        }
    }
    None
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (25, 75),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: -0.5, im: -0.5 }
    );

    assert_eq!(
        pixel_to_point(
            (100, 100),
            (50, 50),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex { re: 0.0, im: 0.0 }
    );
    assert_eq!(
        pixel_to_point(
            (100, 100),
            (50, 50),
            Complex { re: 0.0, im: 1.0 },
            Complex { re: 1.0, im: 0.0 }
        ),
        Complex { re: 0.5, im: 0.5 }
    );
    assert_eq!(
        pixel_to_point(
            (50, 50),
            (50, 50),
            Complex { re: 0.0, im: 1.0 },
            Complex { re: 1.0, im: 0.0 }
        ),
        Complex { re: 1.0, im: 0.0 }
    );
}
