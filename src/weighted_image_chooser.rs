use std::collections::HashMap;

use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::{distributions::WeightedIndex, thread_rng};

// mod file_reader;
use crate::file_reader::InputJsonFileType;

#[derive(Debug)]
pub struct ImageMapping {
    pub file_name: String,
    pub image_number: u32,
    pub name: String,
    pub class_name: String,
    pub features_list: Vec<String>,
}

pub struct ClassImage {
    pub class_name: String,
    pub attributes: Vec<String>
}

pub fn generate_images_map(
    img_json_file_option: &InputJsonFileType,
    file_extension: String,
) -> Vec<ImageMapping> {
    match img_json_file_option {
        InputJsonFileType::ImageDistributionJsonFile(img_json_file) => {
            let file_name = &img_json_file.image_file_name;
            let image_count = &img_json_file.image_count;
            let layers = &img_json_file.layers;

            let mut file_names = vec![];
            let mut metadata_names = vec![];
            let mut image_numbers = vec![];
            for i in 1..=*image_count {
                file_names.push(format!(
                    "{prefix}{image_num}{file_ext}",
                    prefix = file_name,
                    image_num = i,
                    file_ext = file_extension
                ));
                image_numbers.push(i);
                metadata_names.push(format!("{} #{}", img_json_file.image_file_name, i));
            }
            // let mut files_with_added_feature: Vec<String> = vec![];
            let mut images: HashMap<usize, Vec<String>> = HashMap::new();

            for layer in layers {
                let mut choices = vec![];
                let weights = layer
                    .distribution
                    .iter()
                    .map(|distribution| {
                        choices.push(distribution.value.clone());
                        distribution.weight
                    })
                    .collect::<Vec<u32>>();
                println!("choices:: {:?}\n weights:: {:?}", choices, weights);
                let mut rng = thread_rng();
                let dist = WeightedIndex::new(&weights).unwrap();

                for i in 1..=*image_count {
                    let chosen_feature = choices[dist.sample(&mut rng)].clone();
                    let file_name_index = (i - 1) as usize;
                    match images.contains_key(&file_name_index) {
                        true => {
                            images
                                .get_mut(&file_name_index)
                                .unwrap()
                                .push(chosen_feature);
                        }
                        false => {
                            images.insert(file_name_index, vec![chosen_feature]);
                        }
                    }
                }
            }
            let mut result = vec![];
            for (i, (_key, val)) in images.iter().enumerate() {
                result.push(ImageMapping {
                    file_name: file_names[i].clone(),
                    image_number: image_numbers[i],
                    name: metadata_names[i].clone(),
                    features_list: val.clone(),
                    class_name: "".to_string()
                });
            }
            return result;
        }
        InputJsonFileType::ImageDistributionJsonFileWithClasses(img_json_file) => {
            let file_name = &img_json_file.image_file_name;
            let total_image_count = &img_json_file.image_count;
            let class_counts = &img_json_file.class_counts;
            let classes = &img_json_file.classes;

            let mut class_to_count: HashMap<String, u32> = HashMap::new();
            for class_count in class_counts {
                class_to_count.insert(class_count.class_name.clone(), class_count.count);
            }

            let mut result = vec![];
            let mut unmixed_images: Vec<ClassImage> = vec![];

            for nft_class in classes {
                let class_name = nft_class.class_name.clone();
                let layers = &nft_class.layers;
                let image_count = class_to_count.get(&class_name).unwrap();

                let mut images: HashMap<usize, Vec<String>> = HashMap::new();

                for layer in layers {
                    let mut choices = vec![];
                    let weights = layer
                        .distribution
                        .iter()
                        .map(|distribution| {
                            choices.push(distribution.value.clone());
                            distribution.weight
                        })
                        .collect::<Vec<u32>>();
                    println!("choices:: {:?}\n weights:: {:?}", choices, weights);
                    let mut rng = thread_rng();
                    let dist = WeightedIndex::new(&weights).unwrap();

                    for i in 1..=*image_count {
                        let chosen_feature = choices[dist.sample(&mut rng)].clone();
                        let file_name_index = (i - 1) as usize;
                        match images.contains_key(&file_name_index) {
                            true => {
                                images
                                    .get_mut(&file_name_index)
                                    .unwrap()
                                    .push(chosen_feature);
                            }
                            false => {
                                images.insert(file_name_index, vec![chosen_feature]);
                            }
                        }
                    }
                }
                for img_val in images.values() {
                    unmixed_images.push(ClassImage{
                        class_name: class_name.clone(),
                        attributes: img_val.clone()
                    });
                }
            }

            let mut rng = thread_rng();
            unmixed_images.shuffle(&mut rng);

            let mut file_names = vec![];
            let mut metadata_names = vec![];
            let mut image_numbers = vec![];
            for i in 1..=*total_image_count {
                file_names.push(format!(
                    "{prefix}{image_num}{file_ext}",
                    prefix = file_name,
                    image_num = i,
                    file_ext = file_extension
                ));
                image_numbers.push(i);
                metadata_names.push(format!("{} #{}", img_json_file.image_file_name, i));
            }

            for (i, val) in unmixed_images.iter().enumerate() {
                result.push(ImageMapping {
                    file_name: file_names[i].clone(),
                    image_number: image_numbers[i],
                    name: metadata_names[i].clone(),
                    features_list: val.attributes.clone(),
                    class_name: val.class_name.clone()
                });
            }
            return result;
        }
    }
}
