use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Mutex;

use clap::ArgMatches;
use rayon::prelude::*;
use walkdir::WalkDir;

use crate::file_len_inventory::{count_lines, is_hidden};

mod cli_parser;
mod file_len_inventory;
mod parse_inventory;

#[allow(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    let matches: ArgMatches = cli_parser::parse_cli();
    let dir_path: &str = matches
        .get_one::<String>("dir-path")
        .expect("The directory path is required but was not provided");
    let header: bool = matches.get_flag("header");
    let max_depth: usize = matches
        .get_one::<String>("max-depth")
        .unwrap()
        .parse()
        .unwrap();

    let inventory: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());

    WalkDir::new(dir_path)
        .max_depth(0)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|entry| !is_hidden(entry))
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .for_each(|entry| {
            let path: PathBuf = entry.path().to_path_buf();
            if let Ok(line_count) = count_lines(&path, header) {
                // use the lock method to access the inventory
                // in a thread-safe manner
                let mut inventory = inventory.lock().unwrap();
                inventory.insert(path.to_str().unwrap().to_string(), line_count);
            }
        });

    parse_inventory::write_inventory(inventory, dir_path)?;
    Ok(())
}
