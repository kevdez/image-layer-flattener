use std::fs::File;
use std::io::Write;

use crate::file_reader::ImageDistributionJsonFile;
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

pub fn make_nft_metadata(
    img_json_file: &ImageDistributionJsonFile,
    images_map: &Vec<ImageMapping>,
) {
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
            let attr: NftMetadataAttribute = NftMetadataAttribute {
                trait_type: layer.trait_type.clone(),
                value: image.features_list[i].clone(),
            };
            metadata.attributes.push(attr);
        }
        let save_path = format!("results/nft-metadata/nft-metadata{}.json", image_number);
        let json = serde_json::to_string_pretty(&metadata).unwrap();
        // println!("Saving {:?} ...", save_path);
        let mut file = File::create(save_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    });
    println!("done?");
}
