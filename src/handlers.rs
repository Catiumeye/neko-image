use std::{env, fs, path::PathBuf};

use arboard::Clipboard;
use clap::ArgMatches;
use image::{GenericImageView, Pixel};

use crate::compress::{self, compress_image};



pub fn compress(case: &ArgMatches) {

    let is_relative = case.contains_id("relative_path");

    let (file_path, file_name): (PathBuf, String) = if is_relative {
        let current_path = env::current_dir().unwrap();
        let img_path = case.get_one::<String>("relative_path").unwrap();
        let full_path = current_path.join(img_path);
        let file_metadata = fs::metadata(&full_path);
        let original_file_name;
        match file_metadata {
            Ok(ok_val) => {
                if !ok_val.is_file() {
                    panic!("{:?}", format!("{:?} is not file!", full_path))
                }
                original_file_name = full_path.file_name().unwrap().to_string_lossy().to_string();
            }
            Err(err) => {
                panic!("File doesn't exists, {err}, full {:?}", full_path)
            }
        };

        (full_path, original_file_name)
    } else {
        let abs_path_str = case.get_one::<String>("absolute_path").unwrap();
        let full_path = PathBuf::from(abs_path_str);
        let file_metadata = fs::metadata(&full_path);
        let original_file_name;
        match file_metadata {
            Ok(ok_val) => {
                if !ok_val.is_file() {
                    panic!("{:?}", format!("{:?} is not file!", full_path))
                }
                original_file_name = full_path.file_name().unwrap().to_string_lossy().to_string();
            }
            Err(err) => {
                panic!("File doesn't exists, {err}, full {:?}", full_path)
            }
        };
      
        (full_path, original_file_name)
    };

    let (output, output_from_cmd) = if let Some(value) = case.get_one::<String>("output") {
        (PathBuf::from(value), true)
    } else {
        (PathBuf::from("./"), false)
    };

    let img = image::open(&file_path).unwrap();

    let coef = compress::compute_img_size_coef(&img);
    let comp_index = match case.get_one::<String>("exact") {
        Some(raw_value) => {
            let val: u8 = raw_value.parse().unwrap();
            if val.gt(&100) {
                panic!("The compression ratio must be in the range 0...100.");
            }
            val.clone()
        }
        None => {
            let idx = compress::compute_img_index_compression(coef) as u8;
            idx
        }
    };
    let processed_img_bytes = compress_image(img, Some(comp_index));

    let write_path = if output_from_cmd {output} else {PathBuf::from(format!("./compressed_{}", file_name))};
    fs::write(write_path, processed_img_bytes).unwrap();

}

pub fn make_ascii(case: &ArgMatches) {
    let image_path = case.get_one::<PathBuf>("path").unwrap();
    // let image_output_path_raw = case.get_one::<String>("output");
    // let output_path = if let Some(out_path) = image_output_path_raw {
    //     let out_path_buf_raw = PathBuf::from(out_path);
    //     println!("!!!!{:?}",out_path_buf_raw.extension());
    // } else {

    // };
    // println!("{:?}", output_path);

    let gamma = case.get_one::<String>("gamma").unwrap().parse::<f32>().unwrap();
    let target_height = case.get_one::<u32>("height").unwrap().to_owned();
    let target_width = case.get_one::<u32>("width").unwrap().to_owned();

    let image = image::open(image_path).unwrap();
    let width_ratio: f32 = image.width() as f32 / target_width as f32;
    let height_ratio: f32 = image.height() as f32 / target_height as f32;

    let mut ascii_art = String::with_capacity((target_width * target_height) as usize);

    for y in 0..target_height {
        for x in 0..target_width {
            let start_x = (x as f32 * width_ratio) as u32;
            let start_y = (y as f32 * height_ratio) as u32;

            let mut total_r = 0;
            let mut total_g = 0;
            let mut total_b = 0;

            for dy in 0..height_ratio as u32 {
                for dx in 0..width_ratio as u32 {
                    let pixel = image.get_pixel(start_x + dx, start_y + dy);
                    let channels = pixel.channels();
                    total_r += channels[0] as u32;
                    total_g += channels[1] as u32;
                    total_b += channels[2] as u32;
                }
            }

            let count = (width_ratio * height_ratio) as u32;

            let (avg_r, avg_g, avg_b) = if count != 0 {
                let r = (total_r / count) as u8;
                let g = (total_g / count) as u8;
                let b = (total_b / count) as u8;
                (r,g,b)
            } else {
                (0u8, 0u8, 0u8)
            };

            let base_luminance = (0.2126 * avg_r as f32 + 0.7152 * avg_g as f32 + 0.0722 * avg_b as f32) as u8;
            let luminance = ((base_luminance as f32 / 255.0).powf(gamma) * 255.0) as u8;

            let character = match luminance {
                0..=31 => '#',
                32..=63 => '@',
                64..=95 => '8',
                96..=127 => '&',
                128..=159 => 'o',
                160..=191 => ':',
                192..=223 => '*',
                224..=255 => '.',
            };

            ascii_art.push(character);
        }
        ascii_art.push('\n');
    }
    println!("{:?} {:?}", env::current_exe(), env::current_dir());
    Clipboard::new().unwrap().set_text(&ascii_art).unwrap();
    fs::write("./output.txt", ascii_art).unwrap();
    // println!("{ascii_art}");
}