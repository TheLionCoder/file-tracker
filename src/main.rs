use clap::ArgMatches;
use parse_inventory::write_inventory;
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};
use tracing::{event, span, Level, Span};
use walkdir::WalkDir;

use crate::file_len_inventory::{count_lines, is_hidden};

mod cli_parsing;
mod file_len_inventory;
mod inventory_processor;
mod parse_inventory;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let matches: ArgMatches = cli_parsing::parse_cli();
    let dir_path_str: &str = matches
        .get_one::<String>("dir-path")
        .expect("The directory path is required but was not provided");
    let header: bool = matches.get_flag("header");
    let max_depth: usize = matches
        .get_one::<String>("max-depth")
        .unwrap()
        .parse()
        .unwrap();

    let dir_path: &Path = Path::new(dir_path_str);

    let inventory: Arc<Mutex<HashMap<PathBuf, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    event!(Level::INFO, "Starting inventory processing");
    WalkDir::new(dir_path)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|entry| !is_hidden(entry))
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| inventory_processor::is_text_or_csv_file(entry.path()))
        .for_each(|entry| {
            let span: Span = span!(Level::INFO, "Calculating line count");
            let _guard = span.enter();

            let path: PathBuf = entry.path().to_path_buf();
            let file_name: &str = path.file_stem().unwrap().to_str().unwrap();

            event!(Level::INFO, "Calculating line count for {}", file_name);
            let inventory: Arc<Mutex<HashMap<PathBuf, usize>>> = Arc::clone(&inventory);
            if let Ok(line_count) = count_lines(&path, header) {
                // use the lock method to access the inventory
                // in a thread-safe manner
                let mut inventory: MutexGuard<HashMap<PathBuf, usize>> = inventory.lock().unwrap();
                inventory.insert(path, line_count);
            }
        });
    event!(Level::INFO, "Writing inventory to xlsx file");
    write_inventory(inventory, dir_path)?;
    event!(Level::INFO, "Inventory processing complete");
    Ok(())
}
