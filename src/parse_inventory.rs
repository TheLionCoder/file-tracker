use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;

use rust_xlsxwriter::{Format, Formula, Workbook, Worksheet, XlsxError};

pub(crate) fn write_inventory(
    inventory: Mutex<HashMap<String, usize>>,
    path: &str,
) -> Result<(), XlsxError> {
    // Lock the inventory to print the results
    let inventory = inventory.lock().unwrap();
    let mut row: u32 = 1_u32;

    let mut workbook = Workbook::new();
    let mut worksheet: Worksheet = Worksheet::new();

    let bold: Format = Format::new().set_bold();
    let number_format: Format = Format::new().set_num_format("#, #####");

    worksheet.write_with_format(0, 0, "directory", &bold)?;
    worksheet.write_with_format(0, 1, "sub-directory", &bold)?;
    worksheet.write_with_format(0, 2, "file", &bold)?;
    worksheet.write_with_format(0, 3, "line_count", &bold)?;

    for (path, line_count) in &*inventory {
        let path: PathBuf = PathBuf::from(path);
        let file_name: &str = path.file_name().unwrap().to_str().unwrap();
        let parent: &str = path.parent().unwrap().to_str().unwrap();
        let ancestor_component: &str = path
            .components()
            .nth(1)
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap();

        worksheet.write(row, 0, parent)?;
        worksheet.write(row, 1, ancestor_component)?;
        worksheet.write(row, 2, file_name)?;
        worksheet.write(row, 3, *line_count as u32)?;
        row += 1;
    }
    worksheet.write_with_format(row, 2, "TOTAL", &bold)?;
    worksheet.write_with_format(
        row,
        3,
        Formula::new(format!("SUM(D1:D{}", row)),
        &number_format,
    )?;

    workbook.save(PathBuf::from(path).join("inventory.xlsx"))?;
    Ok(())
}
