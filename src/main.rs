extern crate image;

use image::imageops;
#[allow(unused_imports)]
use imageproc::drawing::{
  draw_cross_mut, draw_filled_circle_mut, draw_filled_rect_mut, draw_hollow_circle_mut,
  draw_hollow_rect_mut, draw_line_segment_mut,
};
#[allow(unused_imports)]
use imageproc::rect::Rect;

use image::{DynamicImage, GenericImageView, ImageBuffer, RgbImage, Rgba};

#[allow(unused_mut)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
fn main() {
  let bart = image::open("images/el_barto.png").unwrap();
  let img2 = image::open("images/sky.jpg").unwrap();

  
  for i in 0..5 {
    let mut imgbuf: image::RgbaImage = ImageBuffer::new(500, 500);
    // imageops::overlay(&mut imgbuf, &img2, 0, 0);
    for j in 0..10 { 
      imageops::overlay(&mut imgbuf, &bart, 23 * i, 21 * j);
      let fileName = format!("result_image{image_num}.png", image_num = j);
      imgbuf.save(fileName).unwrap();
    }
    // imageops::overlay(&mut imgbuf, &bart, 0, 0);
  }
  // imgbuf.save("result.png").unwrap();
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_add() {
    assert_eq!(1 + 2, 3);
  }
}
