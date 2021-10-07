use serde_json;
use std::fs;

use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDistributionJsonFile {
  pub image_file_name: String,
  pub description: String,
  pub image_count: u32,
  pub layers: Vec<Layer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layer {
  pub folder_name: String,
  pub trait_type: String,
  pub distribution: Vec<Distribution>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Distribution {
  pub value: String,
  pub weight: u32,
}

pub fn read_input_json_file(file_name: &str) -> ImageDistributionJsonFile {
  println!("JSON file name to open is:\n\n\t{}\n", file_name);

  let data = fs::read_to_string(file_name).expect("Unable to read file");
  let deserialized: ImageDistributionJsonFile =
    serde_json::from_str(&data).expect("Can't serialize");
  deserialized
}
