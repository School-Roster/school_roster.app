use calamine::{open_workbook, Reader, Xlsx};
use std::collections::HashMap;

/// Funcion para leer un archivo de excel
/// # Argumentos
/// * `file_path` - Ruta absoluta del archivo importado
/// Retorna un vector con los headers y los datos si exitoso, de lo contrario arroja error por terminal.
#[tauri::command]
pub fn read_xlsx(file_path: &str) -> Result<(Vec<String>, Vec<HashMap<String, String>>), String> {
    let mut workbook: Xlsx<_> =
        open_workbook(file_path).map_err(|e| format!("Failed to open workbook: {}", e))?;

    match workbook.worksheet_range("Sheet1") {
        Ok(range) => {
            let mut rows = Vec::new();

            // Obtiene los headers
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

            Ok((headers, rows))
        }
        Err(e) => Err(format!("Failed to read worksheet: {}", e)),
    }
}
