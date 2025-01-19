use calamine::{open_workbook, Reader, Xlsx};
use serde::Serialize;
use tauri::Manager;

#[derive(Serialize)]
struct ExcelRow {
    cells: Vec<String>,
}

#[tauri::command]
pub fn read_excel(file_path: String) -> Result<Vec<ExcelRow>, String> {
    let mut workbook: Xlsx<_> = open_workbook(&file_path).map_err(|e| e.to_string())?;
    let mut data = Vec::new();

    if let Some(Ok(sheet)) = workbook.worksheet_range("Sheet1") {
        for row in sheet.rows() {
            data.push(ExcelRow {
                cells: row.iter().map(|cell| cell.to_string()).collect(),
            });
        }
    }

    Ok(data)
}
