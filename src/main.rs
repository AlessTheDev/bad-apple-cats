use std::{fmt::format, fs, thread};

use bad_apple_cats::{convert_to_glob_pattern, get_image_data};
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};
use rand::{random, seq::IteratorRandom};
type ImageData = Vec<String>;

use rayon::prelude::*;
use std::sync::{Arc, Mutex};

// The cat images width
const CAT_WIDTH: u32 = 50;
// The cat images height
const CAT_HEIGHT: u32 = 50;

fn main() {
    // Get all frame files
    let files: Vec<_> = fs::read_dir("frames")
        .unwrap()
        .map(|file| file.unwrap().path())
        .collect();

    // Parallel rendering
    files.par_iter().for_each(|file| {
        let frame = extract_frame_number(format!("{}", file.file_name().unwrap().to_str().unwrap())); 
        println!("rendering: {}", frame);

        let file_path = file.to_str().unwrap();
        let frame_data = get_image_data(file_path, 4);

        write_image(&frame_data, frame);
        println!("{:?} rendered!", file);
    });
}

/// Extracts the frame number from the file name
fn extract_frame_number(file_name: String) -> usize {
    let frame_str = &file_name[file_name.len() - 8..file_name.len() - 4];
    frame_str.parse().unwrap_or(0)
}

/// Creates the imahe based on the frame data
fn write_image(data: &ImageData, frame_n: usize) {
    let mut img: RgbImage = ImageBuffer::new(
        CAT_WIDTH * (data[0].len()) as u32,
        CAT_HEIGHT * (data.len()) as u32,
    );

    for (y, row) in data.iter().enumerate() {
        for (x, pixel) in row.chars().enumerate() {
            // Get a random cat
            let cat_image = get_cat_img();
            let (width, height) = cat_image.dimensions();

            let offset_x = x as u32 * width;
            let offset_y = y as u32 * height;

            if pixel == '1' {
                // Draw cat
                for source_y in 0..height {
                    for source_x in 0..width {
                        let cat_px = cat_image.get_pixel(source_x, source_y).to_rgb();
                        img.put_pixel(offset_x + source_x, offset_y + source_y, cat_px);
                    }
                }
            } else {
                // Draw black square
                for source_y in 0..height {
                    for source_x in 0..width {
                        img.put_pixel(offset_x + source_x, offset_y + source_y, Rgb([0, 0, 0]));
                    }
                }
            }
        }
    }

    // Create a thread to save the image
    thread::spawn(move || {
        img.save(format!(
            "rendered/{}.png",
            convert_to_glob_pattern(frame_n, 4)
        ))
        .unwrap(); // Save once after processing all pixels
    });
}

/// returns a random cat image
fn get_cat_img() -> DynamicImage {
    let files = fs::read_dir("cat_images").unwrap();

    let file = files.choose(&mut rand::thread_rng()).unwrap().unwrap();
    image::open(file.path()).unwrap()
}
