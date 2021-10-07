use clap::{App, Arg};
use std::env;
use std::fs;
use std::process::exit;
use std::time::{Duration, Instant};

mod file_reader;
mod nft_metadata_writer;
mod nifty_maker;
mod weighted_image_chooser;

fn main() {
    let start = Instant::now();

    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();
    println!("{:?} are the args", args);
    const INPUT: &str = "JSON_INPUT_FILE";
    const FOLDER: &str = "FOLDER";
    let matches = App::new("Nifty Magic Image Maker")
        .version("1.0")
        .author("Kevin H. <contact@whatsnextforkev.in>")
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
                ),
        )
        .get_matches();

    let input_arg: &str = matches.subcommand_name().unwrap();
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
                // Now we have a reference to run's matches
                let json_file_arg: &str = run_matches.value_of(INPUT).unwrap();
                let folder_arg: &str = run_matches.value_of(FOLDER).unwrap();
                fs::create_dir("results").unwrap_or(());
                fs::create_dir("results/nft-metadata").unwrap_or(());
                fs::create_dir("results/images").unwrap_or(());
                let img_json_file = file_reader::read_input_json_file(json_file_arg);
                let images = weighted_image_chooser::generate_images_map(&img_json_file);
                nft_metadata_writer::make_nft_metadata(&img_json_file, &images);
                nifty_maker::make_nfts(folder_arg.to_string(), &img_json_file, &images);
            }
        }
        _ => {
            println!("commands not recognized");
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed from start to finish is: \n\n\t{:?}", duration);
}
