# merger
sequence part file merge tool
## How to use
the avaliable command params list
```
Merges holomotion device image files from an input directory into a single output file

Usage: merger.exe [OPTIONS] --output-file <OUTPUT_FILE>

Options:
  -i, --input-directory <INPUT_DIRECTORY>  the path with .part01,.part02 files [default: .]
  -o, --output-file <OUTPUT_FILE>          the path to save the combined img.xz file.
  -h, --help                               Print help
  -V, --version                            Print version
```

1. download device image parts,put in same dir,such as dir A
2. run `merger` wth below options
   ```
   merger -i ./A  -o ubuntu-24.04-preinstalled-arm64.img.xz
   ```
