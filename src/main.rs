extern crate walkdir;
extern crate regex;

use std::fs::File;
use walkdir::WalkDir;
use std::io::Read;
use regex::RegexBuilder;

fn extracted_files() -> Vec::<(String, String)> {

    // Create regex
    let key = "foobar";
    let key = RegexBuilder::new(key)
        .case_insensitive(true)
        .build()
        .expect("Invalid Regex");

    // Create filelist Vector, and WalkDir iterator objects to traverse through directories
    let mut data = Vec::<(String, String)>::new();
    for file in WalkDir::new(".\\test").into_iter()
        .filter_map(|file| file.ok()) { //WalkDir impls Iterator and IntoIter for into_iter() and filter_map()
        let filename = match file.path().is_file() {
            true => file.path().display().to_string(), // Returns filenames
            false => continue, // This is not a file; move on
        };
        println!("{}", filename);

        // use File struct to open file from given filename
        match File::open(&filename) {
            Ok(mut f) => {
                let mut content = String::new(); // set up String to hold file contents
                match f.read_to_string(&mut content) {
                    Ok(_) => println!("Successfully read file {}", &filename),
                    Err(e) => println!("Could not print file: {}", e), // File (likely) has
                    // non-UTF-8 data; will need to implement
                };
                f.read_to_string(&mut content).unwrap(); // Store file contents in String

                // Use regex to search through a slice of content
                // find() takes &str; only way to get this out of a String is to slice it.
                match key.find(&mut content[..]) {
                    Some(m) => data.push((filename, content)),
                    None => continue,
                }


            },
            Err(e) => {
              println!("Error opening file {}: {}", filename, e);
            },
        }
    }

    data
}

fn main() {
    let filelist = extracted_files();

    // Check if no files matched the key
    match filelist.is_empty() {
        true => println!("No files matched the key :("),
        false => {
            println!("Found matches for the key");
            for i in filelist.into_iter() {
                println!("{}", i.0); // It's a tuple
            }
        },
    }

}
