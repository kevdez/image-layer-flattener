# nifty-image-magic-maker
An image generator that can generate your 10k NFT project within seconds.

```
Nifty Magic Image Maker 1.0
kevdez <@mintyatmosphere>
A tool that generates NFT images and JSON metadata files

USAGE:
    ./nifty-magic-image-maker [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    clear    clears the results/ directory
    help     Prints this message or the help of the given subcommand(s)
    run
```

The two arguments needed for the "run" command are:

1. `<json input file>` - the instructions to how you want your images to get generated. The order the "layers" matters: The first layers will end up towards the back of the resulting image.
2. `<folder>` - the folder that contains all your layered PNG assets, with transparency, with subfolders matching the names of the ones in `<json input file>`

```
./nifty-magic-image-maker run [OPTIONS] <json input file> <images folder>
```

The output of this command will be generated into the `results/` folder.

## Getting started 

This example command will generate 30 NFTs:
```
./nifty-magic-image-maker run image_map_with_classes.json ExampleImages
```

You can also run the uncompiled program via `cargo run` with:
```
cargo run --  run image_map_with_classes.json ExampleImages
```

Note: the uncompiled version of the code will run much slower. For some perspective: to generate 100 images with over 100 assets, the cargo command took 40 seconds. When it was compiled, the `nifty-magic-image-maker` executable finished in around 8 seconds. 🚀

// TODO: document the [OPTIONS] for img extension ("jpg" vs "png") and img size (default: 800x800)

## The algorithm, more or less:

1. Given an image_map JSON file, load all the counts, layers, file names, and weight distributions. 
2. For each layer of possible facial features: given a statistical weighted distribution and a list of feature images, randomly pick and choose an image within that layer based on weight. The higher the weight, the higher the likelihood it will get chosen. Repeat this for each layer.
3. Generate the JSON Metadata for each NFT given the chosen layers from step 2.
4. Given a directory of images, load each image into a HashMap.
5. Given each NFT metadata and a Hashmap of all the feature images, stack all the respective assets to make a final Image. Repeat until every NFT image is created.
