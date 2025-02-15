//! This code looks through all folders within the repo and converts them into
//! a website

use std::{error::Error, fs};

const OUTPUT_PATH: &str = "./site";

fn main() -> Result<(), Box<dyn Error>> {
    // create a clean output directory
    if fs::exists(OUTPUT_PATH)? {
        fs::remove_dir_all(OUTPUT_PATH)?;
    }
    fs::create_dir_all(OUTPUT_PATH)?;

    // format individual posts
    // if the directory contains only an index file, create a single file, otherwise create a directory

    // create index pages

    Ok(())
}
