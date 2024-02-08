use image::GenericImageView;

/// Converts a number to glob pattern (ex 23 -> 0023)
/// num_len the new number length (ex. 5 -> 00000)
pub fn convert_to_glob_pattern(number: usize, num_len: usize) -> String {
    let mut num_string = number.to_string();
    loop {
        if num_string.len() >= num_len{
            return num_string;
        }
        num_string = format!("0{}", num_string);
    };
}

/// Returns a vector of Strings containing row data
/// 1 for white pixels and 0 for black ones
/// the scale determines how many pixels get skipped
pub fn get_image_data(path: &str, scale: u32) -> Vec<String> {
    let image = image::open(path).unwrap();

    let (w, h) = image.dimensions();

    let mut rows: Vec<String> = vec![];
    let mut row_data = String::new();
    for y in 0..h {
        for x in 0..w {
            if y % (scale * 2) == 0 && x % (scale * 2) == 0 {
                // Skip some pixels
                let pixel = image.get_pixel(x, y);
                if pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255 {
                    // It's a white pixel
                    row_data += "1";
                } else {
                    row_data += "0";
                }
            }
        }
        if y % (scale * 2) == 0 {
            // New row
            rows.push(row_data.clone());
            row_data = String::new();
        }
    }

    rows
}
