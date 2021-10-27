extern crate image;
use std::collections::HashMap;

use rayon::prelude::*;

use image::imageops;
use image::DynamicImage;
use image::ImageBuffer;
use imageops::FilterType;

use crate::file_reader::{ImageDistributionJsonFileWithClasses, InputJsonFileType, Layer};
use crate::weighted_image_chooser::ImageMapping;

fn store_all_images_from_layer(
    hm: &mut HashMap<String, DynamicImage>,
    layer: &Layer,
    root_image_directory: &String,
) {
    for distribution in layer.distribution.iter() {
        let path = format!(
            "./{}/{}/{}.png",
            root_image_directory, layer.folder_name, distribution.value
        );
        if !hm.contains_key(&path) {
            println!(
                "Opening feature image and storing it into hashmap: {} ...",
                path
            );
            if let Ok(feature_image) = image::open(&path) {
                hm.insert(path, feature_image);
            };
        }
    }
}

fn create_image_feature_path(
    root_image_directory: String,
    folder_name: String,
    feature_img_name: String,
) -> String {
    let feature_img_png = format!("{}.png", feature_img_name);
    let path = format!(
        "./{}/{}/{}",
        root_image_directory, folder_name, feature_img_png
    );
    path
}

fn apply_img_file_to_buffer(
    path: String,
    hashmap: &HashMap<String, DynamicImage>,
    buffer: &mut image::RgbaImage,
    img_width: u32,
    img_height: u32,
) {
    match hashmap.get(&path) {
        Some(dynamic_image) => {
            let resized_image = dynamic_image.resize(img_width, img_height, FilterType::Lanczos3);
            imageops::overlay(buffer, &resized_image, 0, 0);
        }
        None => {
            println!("path not found...");
            return ();
        }
    };
}

fn get_class_layers(
    class_name: String,
    img_json_file: &ImageDistributionJsonFileWithClasses,
) -> &Vec<Layer> {
    return &img_json_file
        .classes
        .iter()
        .find(|class| return class_name == class.class_name)
        .unwrap()
        .layers;
}

fn save_buffer_to_file(file_name: String, buffer: &mut image::RgbaImage) {
    let save_path = format!("results/images/{}", file_name);

    println!("Saving image: {:?} ...", save_path);

    buffer.save(save_path).unwrap();
}

pub fn make_nfts(
    root_image_directory: String,
    img_json_file_type: &InputJsonFileType,
    images_map: &Vec<ImageMapping>,
    img_width: u32,
    img_height: u32,
) {
    match img_json_file_type {
        InputJsonFileType::ImageDistributionJsonFile(img_json_file) => {
            let layers = &img_json_file.layers;
            let mut buffer_hashmap: HashMap<String, DynamicImage> = HashMap::new();

            // load the image layers into a hashmap
            for folder in layers.iter() {
                store_all_images_from_layer(&mut buffer_hashmap, folder, &root_image_directory);
            }

            // create the images
            images_map.par_iter().for_each(|image| {
                let mut imgbuf: image::RgbaImage = ImageBuffer::new(img_width, img_height);
                let file_name = image.file_name.clone();
                for (i, layer) in layers.iter().enumerate() {
                    let path = create_image_feature_path(
                        root_image_directory.clone(),
                        layer.folder_name.clone(),
                        image.features_list[i].clone(),
                    );
                    println!("applying {} to image {}", path, file_name);
                    apply_img_file_to_buffer(
                        path,
                        &buffer_hashmap,
                        &mut imgbuf,
                        img_width,
                        img_height,
                    );
                }

                // save the nft to an image
                save_buffer_to_file(file_name, &mut imgbuf);
            });
        }
        InputJsonFileType::ImageDistributionJsonFileWithClasses(img_json_file) => {
            let classes = &img_json_file.classes;
            let mut buffer_hashmap: HashMap<String, DynamicImage> = HashMap::new();

            for class in classes {
                let layers = &class.layers;

                // load the image layers into a hashmap
                for folder in layers.iter() {
                    store_all_images_from_layer(&mut buffer_hashmap, folder, &root_image_directory);
                }
            }

            // create the images
            images_map.par_iter().for_each(|image| {
                let mut imgbuf: image::RgbaImage = ImageBuffer::new(img_width, img_height);
                let file_name = image.file_name.clone();
                let class_name = image.class_name.clone();

                // get layers for a given class name
                let layers: &Vec<Layer> = get_class_layers(class_name, &img_json_file);

                for (i, layer) in layers.iter().enumerate() {
                    let path = create_image_feature_path(
                        root_image_directory.clone(),
                        layer.folder_name.clone(),
                        image.features_list[i].clone(),
                    );
                    println!("applying {} to image {}", path, file_name);
                    apply_img_file_to_buffer(
                        path,
                        &buffer_hashmap,
                        &mut imgbuf,
                        img_width,
                        img_height,
                    );
                }

                // save the nft to an image
                save_buffer_to_file(file_name, &mut imgbuf);
            });
        }
    }
}
