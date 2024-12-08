use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::DirEntry;

pub(crate) fn count_lines(file_path: &Path, header: bool) -> Result<usize, std::io::Error> {
    let skip: usize = if header { 1 } else { 0 };
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);
    Ok(reader.lines().skip(skip).count())
}

pub(crate) fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::{Error, Write};
    use walkdir::WalkDir;

    #[test]
    fn write_test() -> Result<(), Error> {
        let path: &Path = Path::new("./assets/test.txt");
        let mut file: File = File::create(path)?;
        write!(
            file,
            "Hello, World!\nThis is a test file.\n\
        I am testing the file_len_inventory module.\n"
        )?;
        Ok(())
    }

    #[test]
    fn test_count_files_with_header() {
        let path: &Path = Path::new("./assets/data/ac_sample.txt");
        assert_eq!(count_lines(path, false).unwrap(), 10);
    }

    #[test]
    fn test_count_files_without_header() {
        let path: &Path = Path::new("./assets/test.txt");
        assert_eq!(count_lines(path, true).unwrap(), 2);
    }

    #[test]
    fn test_is_hidden() {
        let path: &Path = Path::new("./assets/.env");
        let entry: DirEntry = WalkDir::new(path.parent().unwrap())
            .into_iter()
            .filter_map(Result::ok)
            .find(|entry| entry.path() == path)
            .unwrap();

        assert!(is_hidden(&entry));
    }
}
