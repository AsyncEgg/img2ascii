use std::{fs::File, io::{Write}};
use image::{GenericImageView, Luma, imageops};

pub fn resize_image(original_image_path: &str, resized_image_path: &str, scale_percentage: f32) {
    let img = image::open(original_image_path).expect("File not found!");
    
    let (width, height) = img.dimensions();
    let new_width = (width as f32 * (scale_percentage / 100.0)) as u32;
    let new_height = (height as f32 * (scale_percentage / 100.0)) as u32;


    let resized_img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = imageops::resize(&img, new_width, new_height, imageops::Nearest);
    resized_img.save(resized_image_path).expect("Failed to save resized image");
}

pub fn image2ascii(image_path: &str, output_path: &str, scale: &str) {
    let img = image::open(image_path).expect("Failed to open image");

    let grayscale_img = img.to_luma8();     
    let (width, height) = grayscale_img.dimensions();

    let matrix: Vec<Vec<u8>> = (0..height).map(|y| {
        (0..width).map(|x| {
            let pixel: Luma<u8> = *grayscale_img.get_pixel(x, y);
            pixel.0[0]
        }).collect()
    }).collect();

    let mut file = File::create(output_path).unwrap();    
    let mut r = vec![];

    let vec_scale = scale.chars().collect::<Vec<_>>();

    for row in matrix {
        let mut result = String::new();
        for col in row {
            let index = map_to_index(col as usize, vec_scale.len(), 0, 255);
            result.push(vec_scale[index]);
            result.push(' ');
        }
        r.push(result)
    }

    file.write_all((r.join("\n")).as_bytes()).unwrap();
}

fn map_to_index(input_value: usize, array_length: usize, input_range_min: usize, input_range_max: usize) -> usize {
    let scaling_factor = (array_length - 1) as f64 / (input_range_max - input_range_min) as f64;
    let index = ((input_value - input_range_min) as f64 * scaling_factor).round() as usize;
    index
}