use llama_rs::{InterfaceParameters, InterfaceResponse, Model, ModelParameters};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

/// Estructura de datos para almacenar el estado de la inteligencia
pub struct AIState {
    model: Arc<Mutex<Model>>,
    conversation_history: Arc<Mutex<Vec<String>>>,
}

/// Funcion para inicializar el modelo AI
// TODO: Verificar los usuarios
#[tauri::command]
pub async fn init_model(handle: tauri::AppHandle) -> Result<String, String> {
    let app_dir = handle.path_resolver().app_dir().unwrap();
    println!("{:?}", app_dir);
    let path = app_dir.join("models/llama-3-8b-q4.gguf");

    let params = ModelParameters {
        path: path.to_str().unwrap().to_string(),
        ..Default::default()
    };

    match Model::load(params) {
        Ok(model) => {
            handle.manage(AIState {
                model: Arc::new(Mutex::new(model)),
                conversation_history: Arc::new(Mutex::new(Vec::new())),
            });
            Ok(String::from("Model loaded successfully"))
        }
        Err(e) => Err(format!("Failed to load model: {}", e)),
    }
}

#[tauri::command]
pub async fn query_ai(message: String, state: tauri::State<'_, AIState>) -> Result<String, String> {
    let mut history = state.conversation_history.lock().unwrap();
    history.push(format!("User: {}", message));

    // Crea el prompt
    let system_prompt = "You are an AI assistant helping with school scheduling. You can modify class schedules, suggest optimal arrangements, and answer questions about the schedule.";

    // Crea un prompt completo, con el historial y el prompt por defecto
    let full_prompt = format!("{}\n\n{}\n\nAssistant:", system_prompt, history.join("\n"));

    let model = state.model.lock().unwrap();
    let params = InterfaceParameters {
        max_tokens: 512, // DEBUG
        temperature: 0.7,
        ..Default::default()
    };

    match model.inference(&full_prompt, &params) {
        Ok(response) => {
            history.push(format!("Assistant: {}", response.text));
            Ok(response.text)
        }
        Err(e) => Err(format!("Inference error: {}", e)),
    }
}

#[tauri::command]
pub async fn download_model(handle: tauri::AppHandle) -> Result<String, String> {
    // Directorio en el que se guardara
    let app_dir = handle.path_resolver().app_dir().unwrap();
    let models_dir = app_dir.join("models");
    let model_path = models_dir.join("tinyllama-1.1b-chat-v1.0.gguf");

    // Create models directory if it doesn't exist
    if !models_dir.exists() {
        std::fs::create_dir_all(&models_dir).map_err(|e| e.to_string())?;
    }

    // Salta el proceso
    if model_path.exists() {
        return Ok("Modelo ya existe".to_string());
    }

    // Download model
    let model_url = "https://huggingface.co/TheBloke/TinyLlama-1.1B-Chat-v1.0-GGUF/resolve/main/tinyllama-1.1b-chat-v1.0.Q4_K_M.gguf";

    let client = reqwest::Client::new();
    let response = client
        .get(model_url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!(
            "No se pudo cargar el modelo, error: {}",
            response.status()
        ));
    }

    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    std::fs::write(&model_path, bytes).map_err(|e| e.to_string())?;

    Ok("Modelo descargado con exito".to_string())
}
