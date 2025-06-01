use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};
use tract_onnx::{prelude::tract_itertools::Groups, tract_core::ndarray::AssignElem};

use super::assignments::{get_all_assignments, Assignment};
use crate::{
    class::{
        classrooms::{get_classrooms, Classroom},
        groups::{get_groups, Group},
        subjects::{get_subjects_with_teachers, SubjectWithTeacher},
        teachers::{get_all_teachers, Teacher},
    },
    AppState,
};

/// Structure to store the AI state
pub struct AIState {
    api_key: String,
    conversation_history: Arc<Mutex<Vec<String>>>,
    schedule_data: Arc<Mutex<Option<ScheduleData>>>,
}

struct ScheduleData {
    teachers: Vec<Teacher>,
    subjects: Vec<SubjectWithTeacher>,
    groups: Vec<Group>,
    classrooms: Vec<Classroom>,
    assignments: Vec<Assignment>,
}

#[derive(Serialize)]
struct ApiRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    stream: bool,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

/// Initialize the AI state with API key
#[tauri::command]
pub async fn init_model(
    api_key: String,
    handle: tauri::AppHandle,
    pool: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let sd = ScheduleData {
        teachers: get_all_teachers(&pool),
        subjects: get_subjects_with_teachers(&pool),
        groups: get_groups(&pool),
        classrooms: get_classrooms(&pool),
        assignments: get_all_assignments(&pool),
    };

    handle.manage(AIState {
        api_key,
        conversation_history: Arc::new(Mutex::new(Vec::new())),
        schedule_data: Arc::new(Mutex::new(Some(sd))),
    });

    Ok(String::from(
        "La conexión a la API se inicializó correctamente",
    ))
}

#[tauri::command]
pub async fn query_ai(message: String, state: tauri::State<'_, AIState>) -> Result<String, String> {
    // Add message to conversation history and create messages before async operation
    let messages = {
        let mut history = state.conversation_history.lock().unwrap();
        history.push(format!("User: {}", message));

        // System prompt
        // let system_prompt = "You are an AI assistant helping with school scheduling. You can modify class schedules, suggest optimal arrangements, and answer questions about the schedule.";
        let system_prompt = "Eres un asistente IA ayudando a hacer horarios escolares. Puedes modificar horarios de clase, sugerir movimientos optimos y contestar preguntas acerca del horario.";

        // Prepare messages for API
        let mut messages = vec![Message {
            role: "system".to_string(),
            content: system_prompt.to_string(),
        }];

        // Add conversation history
        for entry in history.iter() {
            if entry.starts_with("User: ") {
                messages.push(Message {
                    role: "user".to_string(),
                    content: entry.replace("User: ", ""),
                });
            } else if entry.starts_with("Assistant: ") {
                messages.push(Message {
                    role: "assistant".to_string(),
                    content: entry.replace("Asistente: ", ""),
                });
            }
        }

        messages
    }; // MutexGuard is dropped here

    // Prepare API request
    let client = reqwest::Client::new();
    let api_request = ApiRequest {
        model: "deepseek/deepseek-chat-v3-0324:free".to_string(),
        messages,
        max_tokens: Some(512),
        temperature: Some(0.7),
        stream: false,
    };

    // Make API call to OpenRouter
    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", state.api_key))
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "https://schedule-assistant.app") // Replace with your app's URL
        .header("X-Title", "Asistente para School Roster") // Replace with your app's name
        .json(&api_request)
        .send()
        .await
        .map_err(|e| format!("API request error: {}", e))?;

    // Store the status code before consuming the response
    let status = response.status();

    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("OpenRouter API error: {} - {}", status, error_text));
    }

    let api_response: ApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse OpenRouter API response: {}", e))?;

    if api_response.choices.is_empty() {
        return Err("No response from OpenRouter API".to_string());
    }

    let response_text = api_response.choices[0].message.content.clone();

    // Add response to history after receiving the response
    {
        let mut history = state.conversation_history.lock().unwrap();
        history.push(format!("Assistant: {}", response_text));
    } // MutexGuard is dropped here

    Ok(response_text)
}

#[tauri::command]
pub async fn check_api_key(api_key: String) -> Result<String, String> {
    // Make a simple API call to verify the API key
    let client = reqwest::Client::new();
    let api_request = ApiRequest {
        model: "deepseek/deepseek-chat-v3-0324:free".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: "Hello".to_string(),
        }],
        max_tokens: Some(5),
        temperature: Some(0.7),
        stream: false,
    };

    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "https://schedule-assistant.app") // Replace with your app's URL
        .header("X-Title", "Asistente para School Roster") // Replace with your app's name
        .json(&api_request)
        .send()
        .await
        .map_err(|e| format!("API request error: {}", e))?;

    // Store the status code before potentially consuming the response
    let status = response.status();

    if status.is_success() {
        Ok("OpenRouter API key is valid".to_string())
    } else {
        let error_text = response.text().await.unwrap_or_default();
        Err(format!(
            "Invalid OpenRouter API key: {} - {}",
            status, error_text
        ))
    }
}
