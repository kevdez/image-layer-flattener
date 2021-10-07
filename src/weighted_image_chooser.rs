use std::collections::HashMap;

use rand::prelude::*;
use rand::{distributions::WeightedIndex, thread_rng};
#[allow(unused_imports)]
use rayon::iter::IntoParallelIterator;
// mod file_reader;
use crate::file_reader::ImageDistributionJsonFile;

#[derive(Debug)]
pub struct ImageMapping {
    pub file_name: String,
    pub image_number: u32,
    pub name: String,
    pub features_list: Vec<String>,
}

pub fn generate_images_map(img_json_file: &ImageDistributionJsonFile) -> Vec<ImageMapping> {
    let file_name = &img_json_file.image_file_name;
    let image_count = &img_json_file.image_count;
    let layers = &img_json_file.layers;

    let mut file_names = vec![];
    let mut metadata_names = vec![];
    let mut image_numbers = vec![];
    for i in 1..=*image_count {
        file_names.push(format!(
            "{prefix}{image_num}{file_extension}",
            prefix = file_name,
            image_num = i,
            file_extension = ".png"
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
        });
    }
    return result;
}
