extern crate image;
use std::collections::HashMap;

use image::DynamicImage;
use rayon::prelude::*;

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
    let mut buffer_hashmap: HashMap<String, DynamicImage> = HashMap::new();
    // construct the hashmap of files that we'll work with
    for image in images_map.iter() {
        for (i, layer) in layers.iter().enumerate() {
            let feature_img_name = format!("{}.png", image.features_list[i]);
            let path = format!(
                "./{}/{}/{}",
                root_image_directory, layer.folder_name, feature_img_name
            );
            print!("Checking for feature image: {} ...", path);
            if !buffer_hashmap.contains_key(&path) {
                if let Ok(feature_image) = image::open(&path) {
                    println!("storing {} in hashmap", path);
                    buffer_hashmap.insert(path, feature_image);
                };
            } else {
                println!("already stored.");
            }
        }
    }
    // create the images
    images_map.par_iter().for_each(|image| {
        let mut imgbuf: image::RgbaImage = ImageBuffer::new(1500, 1500);
        let file_name = image.file_name.clone();
        for (i, layer) in layers.iter().enumerate() {
            let feature_img_name = format!("{}.png", image.features_list[i]);
            let path = format!(
                "./{}/{}/{}",
                root_image_directory, layer.folder_name, feature_img_name
            );
            println!("applying {} to image {}", path, file_name);
            // if buffer_hashmap.contains_key(&path) {
            let loaded_image: &DynamicImage = buffer_hashmap.get(&path).unwrap();
            imageops::overlay(&mut imgbuf, loaded_image, 0, 0);
            // } else {
            //     let feature_image = match image::open(path) {
            //         Ok(img) => img,
            //         Err(e) => {
            //             println!("Error: {}.", e);
            //             panic!()
            //         }
            //     };
            //     let loaded_image: &mut DynamicImage =
            //         buffer_hashmap.entry(path).or_insert(feature_image);
            //     imageops::overlay(&mut imgbuf, loaded_image, 0, 0);
            // }
        }

        let save_path = format!("results/images/{}", file_name);

        println!("Saving image: {:?} ...", save_path);
        imgbuf.save(save_path).unwrap();
    });
}
