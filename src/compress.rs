use std::io::Cursor;

use image::{DynamicImage, ImageOutputFormat};

pub enum ImgIndexCompression {
  BEST = 100,
  BETTER = 90,
  MEDIUM = 80,
  WORSE = 70,
  WORST = 50
}

pub fn compute_img_size_coef(img: &DynamicImage) -> f64 {
  let h = img.height();
  let w = img.width();

  let img_px_count = (w*h) as f64;
  let size_megabytes = get_size_megabytes(img.as_bytes());
  
  let coef = img_px_count/size_megabytes;
  coef
}

pub fn compute_img_index_compression(coef: f64) -> ImgIndexCompression {
  match coef {
    _ if coef > 900_000. => ImgIndexCompression::BEST,
    _ if coef > 550_000. => ImgIndexCompression::BETTER,
    _ if coef > 400_000. => ImgIndexCompression::MEDIUM,
    _ if coef > 200_000. => ImgIndexCompression::WORSE,
    _ => ImgIndexCompression::WORST,
  }
}

pub fn get_size_megabytes(bytes: &[u8]) -> f64 {
  bytes.len() as f64 / (1024.0 * 1024.0)
}

pub fn compress_image(image: DynamicImage, quality: Option<u8>) -> Vec<u8> {
  let quality = match quality {
    Some(x) => x,
    None => 80,
  };

  let mut buf = Cursor::new(Vec::<u8>::new());
  
  let _ = image.write_to(&mut buf, ImageOutputFormat::Jpeg(quality));
  let my_buf: Vec<u8> = buf.into_inner();
  my_buf
}