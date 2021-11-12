#![allow(dead_code)]
#![allow(unused_variables)]
use clap::{App, Arg};
use std::env;
use std::fs;
use std::process::exit;
use std::time::Instant;

mod file_reader;
mod nft_metadata_writer;
mod nifty_maker;
mod weighted_image_chooser;

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();
    println!("{:?} are the args", args);
    const INPUT: &str = "json input file";
    const FOLDER: &str = "folder";
    const IMG_EXT: &str = "image extension";
    const IMG_SIZE: &str = "image size";
    let matches = App::new("Nifty Magic Image Maker")
        .version("1.0")
        .author("kevdez <contact@whatsnextforkev.in>")
        .about("Makes NFT images and metadata files")
        .subcommand(App::new("clear").about("clears the results/ directory"))
        .subcommand(
            App::new("run")
                .arg(
                    Arg::with_name(INPUT)
                        .help("Sets the input file to use")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name(FOLDER)
                        .help("Sets the folder to read images from")
                        .required(true)
                        .index(2),
                )
                .arg(
                    Arg::with_name(IMG_EXT)
                        .short("e")
                        .long("extension")
                        .value_name(IMG_EXT)
                        .help("Sets the file extension of the images")
                        .required(false)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name(IMG_SIZE)
                        .short("i")
                        .long("imagesize")
                        .value_name(IMG_SIZE)
                        .help("Sets the output image size, default: 800x800")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .get_matches();

    let input_arg: &str = match matches.subcommand_name() {
        Some(input_args) => input_args,
        None => panic!("\n\nArguments were not passed to the program. :(\n\nTry the 'run' argument in the command line. For example:\n\n./nifty-magic-image-maker run\nor\ncargo run -- run\n\n")
    };
    match input_arg {
        "clear" => {
            print!("clearing the results/ directory...");
            match fs::remove_dir_all("results") {
                Ok(()) => {
                    println!("done!");
                    fs::create_dir("results").unwrap();
                    fs::create_dir("results/nft-metadata").unwrap();
                    fs::create_dir("results/images").unwrap();
                }
                Err(_e) => {
                    println!("\nresults/ not found");
                    exit(0);
                }
            };
        }
        "run" => {
            if let Some(run_matches) = matches.subcommand_matches("run") {
                // Now we have a reference to run's matches, to gather all arguments.
                let json_file_arg: &str = run_matches.value_of(INPUT).unwrap();
                let folder_arg: &str = run_matches.value_of(FOLDER).unwrap();
                let img_extension: &str = run_matches.value_of(IMG_EXT).unwrap_or_else(|| ".png");
                let img_size: &str = run_matches.value_of(IMG_SIZE).unwrap_or_else(|| "800x800");

                let width_height: Vec<&str> = img_size.split("x").collect();
                let img_width: u32 = width_height[0].parse().unwrap();
                let img_height: u32 = width_height[1].parse().unwrap();

                // Create the results directory.
                fs::create_dir("results").unwrap_or(());
                fs::create_dir("results/nft-metadata").unwrap_or(());
                fs::create_dir("results/images").unwrap_or(());

                // read the JSON file
                let img_json_file = file_reader::read_input_json_file(json_file_arg);

                // generate the weights of the images
                let images = weighted_image_chooser::generate_images_map(
                    &img_json_file,
                    img_extension.to_string(),
                );

                // generate the metadata json files
                nft_metadata_writer::make_nft_metadata(&img_json_file, &images);

                // generate the images
                nifty_maker::make_nfts(
                    folder_arg.to_string(),
                    &img_json_file,
                    &images,
                    img_width,
                    img_height,
                );
            }
        }
        _ => {
            println!("commands not recognized");
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed from start to finish is: \n\n\t{:?}", duration);
}
