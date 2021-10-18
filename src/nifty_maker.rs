extern crate image;
use std::collections::HashMap;

use rayon::prelude::*;

use image::imageops;
use image::DynamicImage;
use image::ImageBuffer;
use imageops::FilterType;

use crate::file_reader::ImageDistributionJsonFile;
use crate::weighted_image_chooser::ImageMapping;

pub fn make_nfts(
    root_image_directory: String,
    img_json_file: &ImageDistributionJsonFile,
    images_map: &Vec<ImageMapping>,
) {
    let layers = &img_json_file.layers;
    let mut buffer_hashmap: HashMap<String, DynamicImage> = HashMap::new();

    // This way goes through every generated image and every feature of each generated image,
    // and checks if it encounters an image that it hasn't loaded into the hashmap yet.
    // If it has not encountered it, it will add it to the hashmap, otherwise it moves on.
    // for image in images_map.iter() {
    //     for (i, layer) in layers.iter().enumerate() {
    //         let feature_img_name = format!("{}.png", image.features_list[i]);
    //         let path = format!(
    //             "./{}/{}/{}",
    //             root_image_directory, layer.folder_name, feature_img_name
    //         );
    //         print!("Checking for feature image if it already exists in hashmap: {} ...", path);
    //         if !buffer_hashmap.contains_key(&path) {
    //             if let Ok(feature_image) = image::open(&path) {
    //                 println!("storing {} in hashmap", path);
    //                 buffer_hashmap.insert(path, feature_image);
    //             };
    //         } else {
    //             println!("already stored.");
    //         }
    //     }
    // }

    // This faster way goes through every folder of each feature, and opens up each image
    // and loads it into the hashmap. There is no time wasted to see if an image is in
    // the hashmap or not.
    for folder in layers.iter() {
        for distribution in folder.distribution.iter() {
            let path = format!(
                "./{}/{}/{}.png",
                root_image_directory, folder.folder_name, distribution.value
            );
            println!(
                "Opening feature image and storing it into hashmap: {} ...",
                path
            );
            if let Ok(feature_image) = image::open(&path) {
                buffer_hashmap.insert(path, feature_image);
            };
        }
    }

    // create the images
    images_map.par_iter().for_each(|image| {
        let mut imgbuf: image::RgbaImage = ImageBuffer::new(850, 850);
        let file_name = image.file_name.clone();
        for (i, layer) in layers.iter().enumerate() {
            let feature_img_name = format!("{}.png", image.features_list[i]);
            let path = format!(
                "./{}/{}/{}",
                root_image_directory, layer.folder_name, feature_img_name
            );
            println!("applying {} to image {}", path, file_name);
            let resized_image;
            #[allow(unused_variables)]
            let loaded_image: &DynamicImage = match buffer_hashmap.get(&path) {
                Some(dynamic_image) => {
                    resized_image = dynamic_image.resize(850, 850, FilterType::Lanczos3);
                    &resized_image
                }
                None => {
                    println!("path not found...");
                    return ();
                }
            };
            imageops::overlay(&mut imgbuf, &resized_image, 0, 0);
        }

        let save_path = format!("results/images/{}", file_name);

        println!("Saving image: {:?} ...", save_path);
        imgbuf.save(save_path).unwrap();
    });
}
