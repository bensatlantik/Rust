extern crate image;

use image::{ImageBuffer, Rgb};

// Define the image dimensions
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

// Mandelbrot constants
const MAX_ITER: u32 = 255;  // Maximum iterations
const X_MIN: f64 = -2.5;
const X_MAX: f64 = 1.0;
const Y_MIN: f64 = -1.5;
const Y_MAX: f64 = 1.5;

// Function to compute the number of iterations for a point (x, y)
fn mandelbrot(c_re: f64, c_im: f64) -> u32 {
    let mut z_re = 0.0;
    let mut z_im = 0.0;
    let mut iter = 0;

    while z_re * z_re + z_im * z_im <= 4.0 && iter < MAX_ITER {
        let new_re = z_re * z_re - z_im * z_im + c_re;
        let new_im = 2.0 * z_re * z_im + c_im;
        z_re = new_re;
        z_im = new_im;
        iter += 1;
    }

    iter
}

fn main() {
    // Create a new image buffer
    let mut img = ImageBuffer::new(WIDTH, HEIGHT);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // Map pixel position to the complex plane
        let c_re = X_MIN + (x as f64 / WIDTH as f64) * (X_MAX - X_MIN);
        let c_im = Y_MIN + (y as f64 / HEIGHT as f64) * (Y_MAX - Y_MIN);

        // Calculate how many iterations the point belongs to the Mandelbrot set
        let iter = mandelbrot(c_re, c_im);

        // Color the pixel based on the number of iterations
        let color = if iter < MAX_ITER {
            let intensity = (iter as f64 / MAX_ITER as f64 * 255.0) as u8;
            Rgb([intensity, 0, 255 - intensity])  // Blueish color gradient
        } else {
            Rgb([0, 0, 0])  // Black for points in the Mandelbrot set
        };

        *pixel = color;
    }

    // Save the image to a file
    img.save("mandelbrot_fractal.png").unwrap();
    println!("Fractal image saved as 'mandelbrot_fractal.png'.");
}
