extern crate image;

use image::imageops;
use image::ImageBuffer;

use crate::file_reader::ImageDistributionJsonFile;
use crate::weighted_image_chooser::ImageMapping;

#[allow(dead_code)]
pub fn make_nfts(
  root_image_directory: String,
  img_json_file: &ImageDistributionJsonFile,
  images_map: &Vec<ImageMapping>,
) {
  let layers = &img_json_file.layers;

  for image in images_map {
    println!("Generating image: {:?}", image);
    let mut imgbuf: image::RgbaImage = ImageBuffer::new(1501, 1501);
    let file_name = image.file_name.clone();
    for (i, layer) in layers.iter().enumerate() {
      let feature_img_name = format!("{}.png", image.features_list[i]);
      let path = format!(
        "./{}/{}/{}",
        root_image_directory, layer.folder_name, feature_img_name
      );
      println!("Path: {}", path);
      let feature_image = match image::open(path) {
        Ok(img) => img,
        Err(e) => {println!("Error: {}.",e); panic!() }
      };
      imageops::overlay(&mut imgbuf, &feature_image, 0, 0);
      // Make things transparent. This is expensive.
      for x in 0..1501 {
        for y in 0..1501 {
          let pixel = imgbuf.get_pixel_mut(x, y);
          let image::Rgba(data) = *pixel;
          let [r, g, b, _] = data;
          if r == g && g == b && r > 85 && g > 85 && b > 85 {
            *pixel = image::Rgba([0, 0, 0, 0]);
          }
        }
      }
    }

    let save_path = format!("results/{}", file_name);

    println!("Saving {:?} ...", save_path);
    imgbuf.save(save_path).unwrap();
  }
}
