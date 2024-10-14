use std::collections::HashMap;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};

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

    let inventory: Arc<Mutex<HashMap<String, usize>>> = Arc::new(Mutex::new(HashMap::new()));

    WalkDir::new(dir_path)
        .max_depth(max_depth)
        .into_iter()
        .filter_entry(|entry| !is_hidden(entry))
        .par_bridge()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| is_text_or_csv_file(entry.path()))
        .for_each(|entry| {
            let path: PathBuf = entry.path().to_path_buf();
            let inventory: Arc<Mutex<HashMap<String, usize>>> = Arc::clone(&inventory);
            if let Ok(line_count) = count_lines(&path, header) {
                // use the lock method to access the inventory
                // in a thread-safe manner
                let mut inventory: MutexGuard<HashMap<String, usize>> = inventory.lock().unwrap();
                inventory.insert(path.to_str().unwrap().to_string(), line_count);
            }
        });

    parse_inventory::write_inventory(inventory, dir_path)?;
    Ok(())
}

fn is_text_or_csv_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase()),
        Some(ref ext)  if ext == "txt" || ext == "csv"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_text_or_csv_file() {
        let txt_file: &Path = Path::new("test.txt");
        let upper_txt_file: &Path = Path::new("test.TXT");
        let csv_file: &Path = Path::new("test.csv");
        let other_file: &Path = Path::new("test,xlsx");

        assert!(is_text_or_csv_file(txt_file));
        assert!(is_text_or_csv_file(upper_txt_file));
        assert!(is_text_or_csv_file(csv_file));
        assert!(!is_text_or_csv_file(other_file));
    }
}