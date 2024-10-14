use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use rust_xlsxwriter::{Color, Format, FormatAlign, FormatBorder, Formula, Workbook, Worksheet, XlsxError};

pub(crate) fn write_inventory(
    inventory: Arc<Mutex<HashMap<String, usize>>>,
    path: &str,
) -> Result<(), XlsxError> {
    // Lock the inventory to print the results
    let inventory = inventory.lock().unwrap();
    let mut row: u32 = 1_u32;

    let mut workbook = Workbook::new();
    let  worksheet: &mut Worksheet = workbook.add_worksheet().set_name("inventory")?;

    let header: Format = Format::new()
        .set_bold()
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::DashDot)
        .set_background_color(Color::Navy)
        .set_font_color(Color::White);

    let total: Format = Format::new()
        .set_bold()
        .set_italic()
        .set_border(FormatBorder::Double)
        .set_background_color(Color::Yellow);

    let data_format: Format = Format::new()
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin);

    let number_format: Format = Format::new()
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Thin)
        .set_num_format("#, #####");

    worksheet.set_tab_color(Color::Blue);
    worksheet.autofilter(0, 0, 0, 3)?;

    worksheet.write_with_format(0, 0, "directory", &header)?;
    worksheet.write_with_format(0, 1, "sub-directory", &header)?;
    worksheet.write_with_format(0, 2, "file", &header)?;
    worksheet.write_with_format(0, 3, "line_count", &header)?;

    for (path, line_count) in &*inventory {
        let path: PathBuf = PathBuf::from(path);
        let file_name: &str = path.file_name().unwrap().to_str().unwrap();
        let parent: &str = path.components()
            .next()
            .unwrap()
            .as_os_str()
            .to_str().unwrap();
        let ancestor_component: &str = path
            .components()
            .nth(1)
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap();

        worksheet.write_with_format(row, 0, parent, &data_format)?;
        worksheet.write_with_format(row, 1, ancestor_component, &data_format)?;
        worksheet.write_with_format(row, 2, file_name, &data_format)?;
        worksheet.write_with_format(row, 3, *line_count as u32, &number_format)?;
        row += 1;
    }
    worksheet.write_with_format(row, 2, "TOTAL", &total)?;
    worksheet.write_with_format(
        row,
        3,
        Formula::new(format!("SUM(D1:D{})", row)),
        &total
    )?;

    worksheet.autofit();
    workbook.save(PathBuf::from(path).join("inventory.xlsx"))?;
    Ok(())
}
