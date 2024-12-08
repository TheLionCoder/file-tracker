use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard};

use rust_xlsxwriter::{
    Color, Format, FormatAlign, FormatBorder, Formula, Workbook, Worksheet, XlsxError,
};

pub(crate) fn write_inventory(
    inventory: Arc<Mutex<HashMap<PathBuf, usize>>>,
    path: &Path,
) -> Result<(), XlsxError> {
    // Lock the inventory to print the results
    let inventory: MutexGuard<HashMap<PathBuf, usize>> = inventory.lock().unwrap();
    let mut row: u32 = 1_u32;

    let mut workbook : Workbook = Workbook::new();
    let worksheet: &mut Worksheet = workbook.add_worksheet().set_name("inventory")?;

    let header: Format = Format::new()
        .set_bold()
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::DashDot)
        .set_background_color(Color::Navy)
        .set_font_color(Color::White);

    let data_format: Format = Format::new()
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    let number_format: Format = Format::new()
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_num_format("#,#####");

    let total_string: Format = Format::new()
        .set_bold()
        .set_italic()
        .set_border(FormatBorder::Double)
        .set_background_color(Color::Yellow);

    let total: Format = Format::new()
        .set_bold()
        .set_italic()
        .set_border(FormatBorder::Double)
        .set_background_color(Color::Yellow)
        .set_num_format("#,#####");

    worksheet.set_tab_color(Color::Blue);
    worksheet.autofilter(0, 0, 0, 3)?;

    worksheet.write_with_format(0, 0, "path", &header)?;
    worksheet.write_with_format(0, 1, "ancestor_directory", &header)?;
    worksheet.write_with_format(0, 2, "parent_directory", &header)?;
    worksheet.write_with_format(0, 3, "file", &header)?;
    worksheet.write_with_format(0, 4, "line_count", &header)?;

    for (path, line_count) in &*inventory {
        let path: &PathBuf = path;
        let directory: &str = path.parent().unwrap().to_str().unwrap();
        let parent_component: &str = path
            .parent()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("<NA>");
        let ancestor_component: &str = path
            .parent()
            .and_then(|path| path.parent())
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("<NA");
        let file_name: &str = path.file_name().unwrap().to_str().unwrap();

        worksheet.write_with_format(row, 0, directory, &data_format)?;
        worksheet.write_with_format(row, 1, ancestor_component, &data_format)?;
        worksheet.write_with_format(row, 2, parent_component, &data_format)?;
        worksheet.write_with_format(row, 3, file_name, &data_format)?;
        worksheet.write_with_format(row, 4, *line_count as u32, &number_format)?;
        row += 1;
    }
    worksheet.write_with_format(row, 3, "TOTAL", &total_string)?;
    worksheet.write_with_format(row, 4, Formula::new(format!("SUM(E1:E{})", row)), &total)?;

    worksheet.autofit();
    workbook.save(PathBuf::from(path).join("inventory.xlsx"))?;
    Ok(())
}
