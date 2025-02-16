use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use crate::Config;

/// All discovered data about the site that is being built
#[derive(Debug)]
pub struct SiteData {
    /// All posts found within the site
    pub included_data: Node,
}

/// A single directory containing a post or a collection of posts.
#[derive(Debug)]
pub enum Node {
    Folder {
        /// Name of the node/group of posts
        name: String,

        /// Path to the folder containing the node
        path: PathBuf,

        /// All child nodes, no specific order
        children: Vec<Node>,
    },
    File {
        /// Full file name, including the extension
        name: String,

        /// Path to the folder containing the node
        path: PathBuf,

        /// Data read from the file
        data: String,
    },
}

/// Get all posts that are to be included in the website
pub fn get_posts(config: &Config) -> Result<SiteData, Box<dyn Error>> {
    Ok(SiteData {
        included_data: iter_directory(&config.input_dir, true)?,
    })
}

/// Recursively iterate through the input directory and add the discovered posts
/// to the output data.  If `filtered` is true then the source and build files and
/// directories for this site generator get filtered out.
fn iter_directory(path: &Path, filtered: bool) -> Result<Node, Box<dyn Error>> {
    let mut children = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = file_name(entry.path().canonicalize()?);

        if entry.file_type()?.is_dir() {
            if filtered
                && [".git", ".vscode", "site", "src", "target", "templates"]
                    .iter()
                    .any(|s| *s == file_name)
            {
                continue;
            }
            children.push(iter_directory(&entry.path(), false)?);
        } else {
            if filtered {
                // No files in the base of the directory are posts for the website
                continue;
            }
            let path = entry.path();
            children.push(Node::File {
                name: file_name,
                path: path.to_path_buf(),
                data: fs::read_to_string(&path)
                    .map_err(|e| format!("File read error @ {}: {e}", path.display()))?,
            });
        }
    }

    Ok(Node::Folder {
        name: file_name(path),
        path: path.to_path_buf(),
        children,
    })
}

fn file_name<T: AsRef<Path>>(path: T) -> String {
    path.as_ref()
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}
