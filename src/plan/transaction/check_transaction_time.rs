use std::error::Error;
use std::ffi::OsStr;

use calamine as xr;
use xlsxwriter as xw;
use xr::{Reader, Xlsx};

///读取demo
pub fn read_from_excel(path: &str) {
    let str = OsStr::new(path);
    let mut workbook: Xlsx<_> = xr::open_workbook(str).expect("can't open file!");
    if let Some(Ok(range)) = workbook.worksheet_range_at(1) {
        let rows = range.rows();
        rows.for_each(|dt| {
            println!("{}", dt[0]);
        })
    }
}

///写demo
pub fn write_into_excel(path: &str) -> Result<(), Box<dyn Error>> {
    let workbook = xw::Workbook::new(path);
    let mut sheet = workbook.add_worksheet(None)?;
    sheet.write_string(0, 0, "test", None)?;
    Ok(())
}
