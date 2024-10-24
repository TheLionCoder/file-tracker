use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};

use rayon::prelude::*;
use walkdir::WalkDir;

use crate::file_len_inventory::{count_lines, is_hidden};

enum FileExtension {
    Txt,
    Csv,
    Other,
}

impl FileExtension {
    fn from_path(path: &Path) -> Self {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) if ext.eq_ignore_ascii_case("txt") => FileExtension::Txt,
            Some(ext) if ext.eq_ignore_ascii_case("csv") => FileExtension::Csv,
            _ => FileExtension::Other,
        }
    }
}

pub(crate) fn process_inventory(
    dir_path: &Path,
    max_depth: usize,
    header: bool,
) -> Arc<Mutex<HashMap<PathBuf, usize>>> {
    let inventory: Arc<Mutex<HashMap<PathBuf, usize>>> = Arc::new(Mutex::new(HashMap::new()));

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
            let inventory: Arc<Mutex<HashMap<PathBuf, usize>>> = Arc::clone(&inventory);

            if let Ok(line_count) = count_lines(&path, header) {
                // use the lock method to access the inventory
                // in a thread-safe manner
                let mut inventory: MutexGuard<HashMap<PathBuf, usize>> = inventory.lock().unwrap();
                inventory.insert(path, line_count);
            }
        });
    inventory
}

fn is_text_or_csv_file(path: &Path) -> bool {
    matches!(
        FileExtension::from_path(path),
        FileExtension::Txt | FileExtension::Csv
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
