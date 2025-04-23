// src-tauri/src/ai_model.rs
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use tokenizers::Tokenizer;
use tract_onnx::prelude::*;

// Define type for model session
type ModelSession = Arc<
    Mutex<
        Option<(
            tract_onnx::prelude::TypedRunnableModel<Graph<TypedFact, Box<dyn TypedOp>>>,
            Tokenizer,
        )>,
    >,
>;

// Status response
#[derive(Serialize, Deserialize)]
pub struct StatusResponse {
    loaded: bool,
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct GenerateRequest {
    prompt: String,
    max_length: Option<usize>,
}

// Load the model and tokenizer
pub fn create_model_session() -> ModelSession {
    Arc::new(Mutex::new(None))
}

#[tauri::command]
pub async fn load_model(
    app_handle: AppHandle,
    model_session: tauri::State<'_, ModelSession>,
) -> Result<StatusResponse, String> {
    // Check if model is already loaded
    if model_session.lock().unwrap().is_some() {
        return Ok(StatusResponse {
            loaded: true,
            message: "Model already loaded".to_string(),
        });
    }

    // Get paths
    let app_data_dir = app_handle.path_resolver().app_data_dir().unwrap();
    let model_path = app_data_dir.join("models").join("model.onnx");

    let resource_path = app_handle
        .path_resolver()
        .resolve_resource("models/tokenizer.json")
        .ok_or_else(|| "Could not resolve tokenizer path".to_string())?;

    // Load tokenizer
    let tokenizer = Tokenizer::from_file(resource_path)
        .map_err(|e| format!("Failed to load tokenizer: {}", e))?;

    // Load model
    let model = load_onnx_model(&model_path).map_err(|e| format!("Failed to load model: {}", e))?;

    // Store model and tokenizer
    *model_session.lock().unwrap() = Some((model, tokenizer));

    Ok(StatusResponse {
        loaded: true,
        message: "Model loaded successfully".to_string(),
    })
}

fn load_onnx_model(
    path: &PathBuf,
) -> Result<
    tract_onnx::prelude::TypedRunnableModel<Graph<TypedFact, Box<dyn TypedOp>>>,
    Box<dyn std::error::Error>,
> {
    // Load the model
    let model = tract_onnx::onnx()
        .model_for_path(path)?
        .into_optimized()?
        .into_runnable()?;

    Ok(model)
}

#[tauri::command]
pub async fn generate_text(
    model_session: tauri::State<'_, ModelSession>,
    request: GenerateRequest,
) -> Result<String, String> {
    let max_length = request.max_length.unwrap_or(50);
    let prompt = request.prompt;

    // Get locked model session
    let session_guard = model_session.lock().unwrap();
    let session = session_guard
        .as_ref()
        .ok_or_else(|| "Model not loaded".to_string())?;

    let (model, tokenizer) = session;

    // Tokenize input
    let encoding = tokenizer
        .encode(prompt, true)
        .map_err(|e| format!("Tokenization error: {}", e))?;

    let input_ids = encoding.get_ids().to_vec();

    // Create input tensor
    let input_tensor = tract_ndarray::Array2::from_shape_vec(
        (1, input_ids.len()),
        input_ids.iter().map(|&id| id as i64).collect(),
    )
    .map_err(|e| format!("Failed to create input tensor: {}", e))?;

    // Run inference
    let outputs = model
        .run(tvec!(input_tensor.into_tensor().into()))
        .map_err(|e| format!("Inference error: {}", e))?;

    // Extract output ids from the model outputs
    // Note: This is simplified - you'll need to adjust based on your model's actual output format
    let output = outputs[0]
        .to_array_view::<i64>()
        .map_err(|e| format!("Failed to convert output: {}", e))?;

    let output_ids: Vec<u32> = output
        .iter()
        .take(max_length)
        .map(|&id| id as u32)
        .collect();

    // Decode output tokens
    let result = tokenizer
        .decode(&output_ids, true)
        .map_err(|e| format!("Decoding error: {}", e))?;

    Ok(result)
}

#[tauri::command]
pub async fn get_model_status(
    model_session: tauri::State<'_, ModelSession>,
) -> Result<StatusResponse, String> {
    let is_loaded = model_session.lock().unwrap().is_some();

    Ok(StatusResponse {
        loaded: is_loaded,
        message: if is_loaded {
            "Model loaded and ready".to_string()
        } else {
            "Model not loaded".to_string()
        },
    })
}
