use std::fs::File;
use std::io::Write;

use crate::file_reader::ImageDistributionJsonFile;
use crate::weighted_image_chooser::ImageMapping;
use serde::{Deserialize, Serialize};
// use serde_json::Result;

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
    root_image_directory: String,
    img_json_file: &ImageDistributionJsonFile,
    images_map: &Vec<ImageMapping>,
) {
    let layers = &img_json_file.layers;

    for (img_index, image) in images_map.iter().enumerate() {
        println!("Generating image: {:?}", image);
        let file_name = image.file_name.clone();
        let nft_name = image.name.clone();
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

        let save_path = format!("results/nft-metadata/nft-metadata{}.json", img_index+1);
        let json = create_metadata_json(metadata);
        // println!("Saving {:?} ...", save_path);
        let mut file = File::create(save_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

pub fn create_metadata_json(data: NftMetadata) -> String {
    let j = serde_json::to_string_pretty(&data).unwrap();
    // println!("the json is: {}", j);

    return j;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_test() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn converts_nft_metadata_to_json_correctly() {
        let data: NftMetadata = NftMetadata {
            name: "calaverita numero 777".to_owned(),
            description: "calaveritas NFTs".to_owned(),
            image: "".to_owned(),
            attributes: vec![NftMetadataAttribute {
                trait_type: "Background".to_owned(),
                value: "rojo".to_owned(),
            }],
        };
        assert_eq!(create_metadata_json(data), "{\n  \"name\": \"calaverita numero 777\",\n  \"description\": \"calaveritas NFTs\",\n  \"image\": \"\",\n  \"attributes\": [\n    {\n      \"trait_type\": \"Background\",\n      \"value\": \"rojo\"\n    }\n  ]\n}");
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
