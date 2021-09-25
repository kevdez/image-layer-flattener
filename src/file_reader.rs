use serde_json;
use std::fs;

pub use serde::{Deserialize, Serialize};
pub use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageDistributionJsonFile {
  pub image_file_name: String,
  pub layers: Vec<Layer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layer {
  pub name: String,
  pub count: i32,
  pub distribution: Vec<Distribution>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Distribution {
  pub feature_name: String,
  pub weight: u32,
}

pub fn read_input_json_file(file_name: &str) -> ImageDistributionJsonFile {
  println!("JSON file name to open is:\n\n\t{}\n", file_name);

  let data = fs::read_to_string(file_name).expect("Unable to read file");
  let deserialized: ImageDistributionJsonFile =
    serde_json::from_str(&data).expect("Can't serialize");
  deserialized
}
