use std::path::{Path};

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

pub(crate) fn is_text_or_csv_file(path: &Path) -> bool {
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
