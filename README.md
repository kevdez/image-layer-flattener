# nifty-image-magic-maker
An image generator that can generate your 10k NFT project within seconds.

_If you happen to clone this repo or use this CLI tool, please follow me on Twitter: https://twitter.com/mintyatmosphere_

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

// TODO: document the [OPTIONS] for img extension ("jpg" vs "png") and img size (default: 800x800)