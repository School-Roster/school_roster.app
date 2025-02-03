use calamine::{open_workbook, Reader, Xlsx};
use std::collections::HashMap;

#[tauri::command]
pub fn read_xlsx(file_path: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let mut workbook: Xlsx<_> =
        open_workbook(file_path).map_err(|e| format!("Failed to open workbook: {}", e))?;

    match workbook.worksheet_range("Sheet1") {
        Ok(range) => {
            let mut rows = Vec::new();

            // Obtiene los headers de la primera fila
            let headers: Vec<String> = range
                .rows()
                .next()
                .map(|row| row.iter().map(|cell| cell.to_string()).collect())
                .ok_or_else(|| "No headers found".to_string())?;

            // Procesamos cada fila
            for row in range.rows().skip(1) {
                let mut row_data = HashMap::new();
                for (i, cell) in row.iter().enumerate() {
                    if i < headers.len() {
                        row_data.insert(headers[i].clone(), cell.to_string());
                    }
                }
                rows.push(row_data);
            }

            Ok(rows)
        }
        Err(e) => Err(format!("Failed to read worksheet: {}", e)),
    }
}
