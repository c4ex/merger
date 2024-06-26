use clap::Parser;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(name = "merger")]
#[command(version = "1.0")]
#[command(author = "czyt <go.czyt@gmail.com>")]
#[command(
    about = "Merges holomotion device image files from an input directory into a single output file"
)]
struct Cli {
    #[arg(short,long, default_value = ".",help = "the path with .part01,.part02 files")]
    input_directory: String,
    #[arg(short,long,help = "the path to save the combined img.xz file.for example:ubuntu-24.04.img.xz")]
    output_file: String,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    let re = Regex::new(r"\d+").unwrap();

    let mut entries: Vec<_> = fs::read_dir(&args.input_directory)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.is_file() && p.file_name().unwrap().to_str().map_or(false, |s| {
                // Check if the file name contains digits
                re.is_match(s)
            })
        })
        .collect();

    entries.sort_by_key(|p| {
        // Extract the numeric part of the file name and parse it as u32
        let filename = p.file_name().unwrap().to_str().unwrap();
        let num_part = re.find(filename).unwrap().as_str();
        num_part.parse::<u32>().unwrap()
    });

    let output_path = Path::new(&args.output_file);
    let output_file = File::create(&output_path)?;
    let mut writer = BufWriter::new(output_file);

    entries.iter().try_for_each(|entry| -> io::Result<()> {
        println!("Merging {:?}", entry.file_name().unwrap());
        let input_file = File::open(entry)?;
        let mut reader = BufReader::new(input_file);
        io::copy(&mut reader, &mut writer)?;
        Ok(())
    })?;
    println!("Merge completed successfully!");
    Ok(())
}
