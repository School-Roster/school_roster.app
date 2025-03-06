use calamine::{open_workbook, Reader, Xlsx};
use std::collections::HashMap;
use std::path::Path;

/// Funcion para leer un archivo de excel
/// # Argumentos
/// * `file_path` - Ruta absoluta del archivo importado
/// Retorna un vector con los headers y los datos si exitoso, de lo contrario arroja error por terminal.
#[tauri::command]
pub fn read_xlsx(
    file_path: &str,
) -> Result<(Vec<String>, Vec<HashMap<String, serde_json::Value>>), String> {
    let mut workbook: Xlsx<_> =
        open_workbook(file_path).map_err(|e| format!("Failed to open workbook: {}", e))?;

    // Get all sheet names first
    let sheet_names = workbook.sheet_names().to_vec();

    // If no sheets found, return error
    if sheet_names.is_empty() {
        return Err("No sheets found in workbook".to_string());
    }

    // Use the first sheet by default
    let sheet_name = &sheet_names[0];

    match workbook.worksheet_range(sheet_name) {
        Ok(range) => {
            let mut rows = Vec::new();

            // Get the headers from the first row
            let headers: Vec<String> = range
                .rows()
                .next()
                .map(|row| row.iter().map(|cell| cell.to_string()).collect())
                .ok_or_else(|| "No headers found".to_string())?;

            // Process each row with proper type conversion
            for row in range.rows().skip(1) {
                let mut row_data = HashMap::new();

                for (i, cell) in row.iter().enumerate() {
                    if i < headers.len() {
                        // Convert cell to appropriate JSON value based on its type
                        let value = match cell {
                            <dyn calamine::DataType>::Int(i) => {
                                serde_json::Value::Number((*i).into())
                            }
                            <dyn calamine::DataType>::Float(f) => {
                                if let Some(n) = serde_json::Number::from_f64(*f) {
                                    serde_json::Value::Number(n)
                                } else {
                                    serde_json::Value::String(f.to_string())
                                }
                            }
                            <dyn calamine::DataType>::String(s) => {
                                serde_json::Value::String(s.clone())
                            }
                            <dyn calamine::DataType>::Bool(b) => serde_json::Value::Bool(*b),
                            _ => serde_json::Value::String(cell.to_string()),
                        };

                        row_data.insert(headers[i].clone(), value);
                    }
                }

                rows.push(row_data);
            }

            // Debug logging for MacOS troubleshooting
            println!(
                "Successfully read worksheet '{}' with {} rows",
                sheet_name,
                rows.len()
            );

            Ok((headers, rows))
        }
        Err(e) => Err(format!("Failed to read worksheet: {}", e)),
    }
}
