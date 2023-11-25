use std::{env, path::PathBuf, fs};
use clap::{Arg, Command, ArgGroup};

use crate::compress::compress_image;

pub mod compress;


fn main() {
  let rpc1 = "\x1b[0m\x1b[48;2;92;195;233m\n";
  let rpc2 = "\x1b[0m\x1b[48;2;235;165;177m\n";
  let rpc3 = "\x1b[0m\x1b[48;2;255;255;255m\n";
  let rust_programmer_colors = format!("{}{}{}{}{}\x1b[0m", rpc1, rpc2, rpc3, rpc2, rpc1);
  
  let mathches = Command::new("NekoFeatImg")
    .about(rust_programmer_colors)
    .version("0.0.0")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .author("Catiumeye")
    .subcommand(
      Command::new("compress")
        .about("Compress the image as you wish")
        .arg(
          Arg::new("relative_path")
            .short('p')
            .long("path")
            .help("Relative path to image")
        )
        .arg(
          Arg::new("absolute_path")
            .short('a')
            .long("apath")
            .help("Absolute path to image")
        )
        .group(
          ArgGroup::new("path_group")
            .args(["relative_path", "absolute_path"])
            .required(true)
        )
        .arg(
          Arg::new("exact")
            .short('e')
            .long("exact")
            .required(false)
            .help("Range value 0...100 for compression index")
        )
        .arg(
          Arg::new("output")
            .short('o')
            .long("output")
            .required(false)
            .help("Path to output")
        ),
    )
    .get_matches();

  match mathches.subcommand() {
    Some(("compress", test_match)) => {
      let start_time = std::time::Instant::now();
      let is_relative = test_match.contains_id("relative_path");

      let (file_path, file_name): (PathBuf, String) = if is_relative {
        let current_path = env::current_dir().unwrap();
        let img_path = test_match.get_one::<String>("relative_path").unwrap();
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
        let abs_path_str = test_match.get_one::<String>("absolute_path").unwrap();
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

      let (output, output_from_cmd) = if let Some(value) = test_match.get_one::<String>("output") {
        (PathBuf::from(value), true)
      } else {
        (PathBuf::from("./"), false)
      };

      let img = image::open(&file_path).unwrap();

      let coef = compress::compute_img_size_coef(&img);
      let comp_index = match test_match.get_one::<String>("exact") {
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


      println!("Compressed in {:?}", std::time::Instant::now().duration_since(start_time));
    },
    a => {
      panic!("Nothing: {:?}", a)
    }
  }
}