mod file_reader;

fn main() {
  file_reader::read_input_json_file("image_map.json");
}

// extern crate image;
//
// use image::imageops;
// use image::ImageBuffer;

// let bart = image::open("images/el_barto.png").unwrap();
// // let img2 = image::open("images/sky.jpg").unwrap();

// for i in 0..5 {
//   let mut imgbuf: image::RgbaImage = ImageBuffer::new(500, 500);
//   // imageops::overlay(&mut imgbuf, &img2, 0, 0);
//   for j in 0..10 {
//     imageops::overlay(&mut imgbuf, &bart, 23 * i, 21 * j);
//     let file_name = format!("result_image{image_num}.png", image_num = j);
//     imgbuf.save(file_name).unwrap();
//   }
// }

#[cfg(test)]
mod tests {
  #[test]
  fn test_add() {
    assert_eq!(1 + 2, 3);
  }
}
