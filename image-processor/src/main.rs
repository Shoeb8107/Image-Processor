use std::env;
use std::process;
use std::path::Path;
use std::f32::consts::PI;
use egui::Image;
use image::{ImageBuffer, Rgba};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.is_empty() || args.len() > 3 {
        eprintln!("Incorrect use, please try again");
        process::exit(1);
    }
    let query = &args[1];
    let file_path = &args[2];

    print!("Applying {}", query);
    println!(" to {}", file_path);

    let width: i32;
    let height: i32;
    let channels: i32;

    let img = image::open(file_path).unwrap();
    let img = img.to_rgba8();

    let blurred_img = gauss_filter(img);

    blurred_img.save("output1.png").expect("Failed to save image");
    println!("Image processed");


}

fn gauss_filter(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let sigma = 2.0;
    let kernel_size = 5;

    let mut kernel = vec![vec![0.0; kernel_size]; kernel_size];
    let mean = (kernel_size / 2) as f32;
    let mut sum = 0.0;

    for x in 0..kernel_size {
        for y in 0..kernel_size {
            let value = (1.0/(2.0 * PI * sigma * sigma)) * (-((x as f32 - mean).powi(2) + (y as f32 - mean).powi(2)) / (2.0 * sigma * sigma)).exp();
            kernel[x][y] = value;
            sum += value;
        }
    }

    for x in 0..kernel_size {
        for y in 0..kernel_size {
            kernel[x][y] /= sum;
        }
    }

    let (width, height) = img.dimensions();
    let mut output_img = ImageBuffer::new(width, height);
    let k_size = kernel.len() as i32;
    let k_center = k_size / 2;
    
    for x in 0..width {
        for y in 0..height {
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;
            let mut a = 0.0;

            for i in 0..k_size {
                for j in 0..k_size {
                    let ix = (x as i32 + i - k_center).clamp(0, width as i32 - 1) as u32;
                    let iy = (y as i32 + j - k_center).clamp(0, height as i32 - 1) as u32;
                    let pixel = img.get_pixel(ix, iy);

                    r += kernel[i as usize][j as usize] * pixel[0] as f32;
                    g += kernel[i as usize][j as usize] * pixel[1] as f32;
                    b += kernel[i as usize][j as usize] * pixel[2] as f32;
                    a += kernel[i as usize][j as usize] * pixel[3] as f32;
                }
            }

            output_img.put_pixel(x, y, Rgba([r as u8, g as u8, b as u8, a as u8]));
        }
    }
    
    output_img
}
