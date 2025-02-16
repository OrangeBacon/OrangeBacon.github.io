//! This code looks through all folders within the repo and converts them into
//! a website

mod file_system;

use std::{env, error::Error, fs, io::Write, path::PathBuf};

use file_system::get_posts;

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

    // format individual posts
    // if the directory contains only an index file, create a single file, otherwise create a directory

    // create index pages
    let mut index = fs::File::create(config.output_dir.join("index.html"))?;
    let data = indoc::formatdoc! {"
    <!doctype html>
    <html>
        <head>
            <title>{name}</title>
        </head>
        <body>
            {data:#?}
        </body>
    </html>
    ",
    name = config.site_name,
    data = get_posts(&config)?};
    write!(index, "{}", data)?;

    Ok(())
}
