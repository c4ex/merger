use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_directory> <output_file>", args[0]);
        return Ok(());
    }

    let input_dir = &args[1];
    let output_file = &args[2];

    let re = Regex::new(r"\d+").unwrap(); // Regex to match one or more digits

    let mut entries: Vec<_> = fs::read_dir(input_dir)?
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

    let output_path = Path::new(output_file);
    let output_file = File::create(&output_path)?;
    let mut writer = BufWriter::new(output_file);

    for entry in entries {
        println!("Merging {:?}", entry.file_name().unwrap());
        let input_file = File::open(entry)?;
        let mut reader = BufReader::new(input_file);
        io::copy(&mut reader, &mut writer)?;
    }
    println!("Merge completed successfully!");
    Ok(())
}
