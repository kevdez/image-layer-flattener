use serde_json;
use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputJsonFileType {
    ImageDistributionJsonFile(ImageDistributionJsonFile),
    ImageDistributionJsonFileWithClasses(ImageDistributionJsonFileWithClasses),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDistributionJsonFile {
    pub image_file_name: String,
    pub description: String,
    pub image_count: u32,
    pub layers: Vec<Layer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageDistributionJsonFileWithClasses {
    pub image_file_name: String,
    pub description: String,
    pub image_count: u32,
    pub class_counts: Vec<ClassCount>,
    pub classes: Vec<NFTClass>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ClassCount {
    pub class_name: String,
    pub count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TraitType {
    pub trait_type: String,
    pub value: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NFTClass {
    pub class_name: String,
    pub must_have_class_traits: Vec<TraitType>,
    pub layers: Vec<Layer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layer {
    pub folder_name: String,
    pub trait_type: String,
    pub distribution: Vec<Distribution>,
    
    #[serde(default = "default_omit_from_metadata")]
    pub omit_from_metadata: bool,
}

fn default_omit_from_metadata() -> bool {
  false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Distribution {
    pub value: String,
    pub weight: u32,
}

pub fn read_input_json_file(file_name: &str) -> InputJsonFileType {
    println!("JSON file name to open is:\n\n\t{}\n", file_name);

    let data = fs::read_to_string(file_name).expect("Unable to read file");
    let deserialized: InputJsonFileType = serde_json::from_str(&data).expect("Can't deserialize");
    deserialized
}
