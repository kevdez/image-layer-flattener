use std::fs::File;
use std::io::Write;

use crate::file_reader::{InputJsonFileType, Layer};
use crate::weighted_image_chooser::ImageMapping;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NftMetadataAttribute {
    trait_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NftMetadata {
    name: String,
    description: String,
    image: String,
    attributes: Vec<NftMetadataAttribute>,
}

pub fn make_nft_metadata(img_json_file_type: &InputJsonFileType, images_map: &Vec<ImageMapping>) {
    match img_json_file_type {
        InputJsonFileType::ImageDistributionJsonFile(img_json_file) => {
            let layers = &img_json_file.layers;

            images_map.par_iter().for_each(|image| {
                println!("Generating image: {:?}", image);
                let file_name = &image.file_name.clone();
                let nft_name = image.name.clone();
                let image_number = image.image_number;
                let mut metadata = NftMetadata {
                    name: nft_name.clone(),
                    description: img_json_file.description.clone(),
                    image: file_name.clone(),
                    attributes: vec![],
                };
                for (i, layer) in layers.iter().enumerate() {
                    if !layer.omit_from_metadata {
                        let attr: NftMetadataAttribute = NftMetadataAttribute {
                            trait_type: layer.trait_type.clone(),
                            value: image.features_list[i].clone(),
                        };
                        metadata.attributes.push(attr);
                    }
                }
                let save_path = format!("results/nft-metadata/{}.json", image_number);
                let json = serde_json::to_string_pretty(&metadata).unwrap();
                // println!("Saving {:?} ...", save_path);
                let mut file = File::create(save_path).unwrap();
                file.write_all(json.as_bytes()).unwrap();
            });
        }
        InputJsonFileType::ImageDistributionJsonFileWithClasses(img_json_file) => {
            images_map.par_iter().for_each(|image| {
                let class_name = &image.class_name.clone();
                let layers: &Vec<Layer> = &img_json_file
                    .classes
                    .iter()
                    .find(|nft_class| *nft_class.class_name == class_name.to_string())
                    .unwrap()
                    .layers;
                let nft_class = &img_json_file
                    .classes
                    .iter()
                    .find(|nft_class| *nft_class.class_name == class_name.to_string())
                    .unwrap();
                let layers: &Vec<Layer> = &nft_class.layers;
                let classes_to_include = &nft_class.must_have_class_traits;

                println!("Generating image: {:?}", image);
                let file_name = &image.file_name.clone();
                let nft_name = image.name.clone();
                let image_number = image.image_number;
                let mut metadata = NftMetadata {
                    name: nft_name.clone(),
                    description: img_json_file.description.clone(),
                    image: file_name.clone(),
                    attributes: vec![],
                };
                for (i, layer) in layers.iter().enumerate() {
                    if !layer.omit_from_metadata {
                        let attr: NftMetadataAttribute = NftMetadataAttribute {
                            trait_type: layer.trait_type.clone(),
                            value: image.features_list[i].clone(),
                        };
                        metadata.attributes.push(attr);
                    }
                }
                for class_to_include in classes_to_include.iter() {
                    let attr: NftMetadataAttribute = NftMetadataAttribute {
                        trait_type: class_to_include.trait_type.clone(),
                        value: class_to_include.value.clone(),
                    };
                    metadata.attributes.push(attr);
                }

                let save_path = format!("results/nft-metadata/{}.json", image_number);
                let json = serde_json::to_string_pretty(&metadata).unwrap();
                // println!("Saving {:?} ...", save_path);
                let mut file = File::create(save_path).unwrap();
                file.write_all(json.as_bytes()).unwrap();
            });
            println!("done?");
        }
    }
    println!("done?");
}
