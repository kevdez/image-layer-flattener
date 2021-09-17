// use serde::{Deserialize, Serialize};
// use serde::{Deserialize, Serialize};
// use serde_json;
// use serde_json::{Deserialize, Serialize};
use std::fs;


pub use serde::{Deserialize, Serialize};
pub use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct ImageDistributionJsonFile {
  imageFileName: String,
}

pub fn read_input_json_file(file_name: &str) -> &str {
  println!("file name given is: {}", file_name);

  fs::read_to_string(file_name);
  let data = fs::read_to_string(file_name).expect("Unable to read file");
  println!(" read this fromt eh file: {}", data);
  let deserialized : ImageDistributionJsonFile = serde_json::from_str(&data).unwrap();
  // println!("deserialized: {:?}", deserialized);

  file_name
}
