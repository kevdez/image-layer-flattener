use std::env;

mod file_reader;
mod nifty_maker;
mod weighted_image_chooser;
mod nft_metadata_writer;

use crate::file_reader::read_input_json_file;
// use crate::nifty_maker::make_nfts;
use crate::nft_metadata_writer::make_nft_metadata;
use crate::weighted_image_chooser::generate_images_map;

fn main() {
  let img_json_file = read_input_json_file("image_map.json");

  let images = generate_images_map(&img_json_file);

  println!("images: {:?}", images);

  let args: Vec<String> = env::args().collect();
  println!("{:?} are the args", args);
  if args.len() > 1 {
    println!("the root level directory is: {:?}", args[1]);
    make_nft_metadata(args[1].clone(), &img_json_file, &images);
    // make_nfts(args[1].clone(), &img_json_file, &images);
  } else {
    println!("Please specificy root level directory of the images.");
  }
  // nft_metadata_writer::hello_world().unwrap();
}