//! This code looks through all folders within the repo and converts them into
//! a website

mod file_system;
mod posts;

use std::{env, error::Error, fs, path::PathBuf};

use file_system::get_posts;
use posts::process_site_data;

#[derive(Default)]
pub struct Config {
    site_name: String,
    input_dir: PathBuf,
    output_dir: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::default();
    config.input_dir = env::current_dir()?.canonicalize()?;
    config.output_dir = PathBuf::from("./site").canonicalize()?;
    config.site_name = config
        .input_dir
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    // create a clean output directory
    if fs::exists(&config.output_dir)? {
        fs::remove_dir_all(&config.output_dir)?;
    }
    fs::create_dir_all(&config.output_dir)?;

    // get the data to be stored in the site
    let site = get_posts(&config)?;

    // process site data
    process_site_data(site, &config)?;

    Ok(())
}
