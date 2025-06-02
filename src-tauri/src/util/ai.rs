use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

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

#[derive(Deserialize, Debug)]
struct ApiResponse {
    choices: Vec<Choice>,
    #[serde(default)]
    error: Option<ApiError>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize, Debug)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize, Debug)]
struct ApiError {
    message: String,
    #[serde(rename = "type")]
    error_type: Option<String>,
}

/// Initialize the AI state with API key
#[tauri::command]
pub async fn init_model(
    api_key: String,
    handle: tauri::AppHandle,
    pool: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let sd = ScheduleData {
        teachers: get_all_teachers(pool.clone())
            .await?
            .into_iter()
            .map(|(t, _)| t)
            .collect::<Vec<Teacher>>(),
        subjects: get_subjects_with_teachers(pool.clone()).await?,
        groups: get_groups(pool.clone())
            .await?
            .into_iter()
            .map(|(g, _)| g)
            .collect::<Vec<Group>>(),
        classrooms: get_classrooms(pool.clone()).await?,
        assignments: get_all_assignments(pool.clone()).await?,
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

// Funcion para mandar el horario a la IA
fn format_schedule_context(schedule_data: &ScheduleData) -> String {
    let mut context = String::new();
    context.push_str("INFORMACION DEL HORARIO ACTUAL:\n\n");

    // Mandar informacion de los maestros
    context.push_str("MAESTROS:\n");
    for teacher in &schedule_data.teachers {
        context.push_str(&format!(
            "- ID: {}, Nombre: {} {}\n",
            teacher.id.unwrap_or(0),
            teacher.name,
            teacher.father_lastname
        ));
    }
    context.push_str("\n");

    // Informacion de las materias
    context.push_str("MATERIAS:\n");
    for subject in &schedule_data.subjects {
        context.push_str(&format!(
            "ID: {}, Nombre: {}, Abreviacion: {}, Profesor asignado: {}\n",
            subject.id,
            subject.name,
            subject.shorten,
            subject
                .assigned_teacher
                .as_ref()
                .map(|t| format!("{} {} (ID: {})", t.name, t.father_lastname, t.id.unwrap_or(0)))
                .unwrap_or(String::from("No asignado")),
        ));
    }
    context.push_str("\n");

    // Informacion de los grupos
    context.push_str("GRUPOS:\n");
    for group in &schedule_data.groups {
        context.push_str(&format!(
            "ID: {}, Grado: {}, Grupo: {}, Cantidad de alumnos: {}\n",
            group.id.unwrap_or(0),
            group.grade,
            group.group,
            group.students.unwrap_or(-1)
        ));
    }
    context.push_str("\n");

    // Informacion de aulas
    context.push_str("AULAS:\n");
    for cr in &schedule_data.classrooms {
        context.push_str(&format!(
            "ID: {}, Edificio: {}{}, Tipo: {}, Capacidad: {}\n",
            cr.id.unwrap_or(0),
            cr.building_id.clone().unwrap_or_default(),
            cr.building_number,
            cr.building_type.as_ref().unwrap_or(&"N/A".to_string()),
            cr.capacity.unwrap_or(0)
        ));
    }
    context.push_str("\n");

    // Asignaciones dentro del horario
    context.push_str("ASIGNACIONES ACTUALES:\n");
    let max_modules: u8 = 9;
    for assignment in &schedule_data.assignments {
        context.push_str(&format!(
            "ID: {}, Materia: {} (ID: {}), Maestro ID: {}, Grupo ID: {}, Aula ID: {}, Dia: {}, Modulo: {} de {}\n",
            assignment.id.unwrap_or(0), 
            assignment.subject_shorten, 
            assignment.subject_id, 
            assignment.teacher_id, 
            assignment.group_id, 
            assignment.classroom_id, 
            assignment.day, 
            assignment.module_index, 
            max_modules
        ));
    }

    context.push_str("\nPuedes ayudar al usuario a:\n");
    context.push_str("- Consultar información sobre maestros, materias, grupos y aulas\n");
    context.push_str("- Analizar conflictos de horarios\n");
    context.push_str("- Sugerir optimizaciones del horario\n");
    context.push_str("- Responder preguntas sobre las asignaciones actuales\n");
    context.push_str("- Ayudar con la planificación de horarios\n\n");

    context
}

#[tauri::command]
pub async fn query_ai(
    message: String,
    state: tauri::State<'_, AIState>,
    pool: tauri::State<'_, AppState>,
) -> Result<String, String> {
    // Refresh schedule data from database
    let schedule_data = ScheduleData {
        teachers: get_all_teachers(pool.clone())
            .await?
            .into_iter()
            .map(|(t, _)| t)
            .collect::<Vec<Teacher>>(),
        subjects: get_subjects_with_teachers(pool.clone()).await?,
        groups: get_groups(pool.clone())
            .await?
            .into_iter()
            .map(|(g, _)| g)
            .collect::<Vec<Group>>(),
        classrooms: get_classrooms(pool.clone()).await?,
        assignments: get_all_assignments(pool.clone()).await?,
    };

    // Update stored schedule data
    {
        let mut stored_data = state.schedule_data.lock().unwrap();
        *stored_data = Some(schedule_data);
    }

    // Agrega mensaje al historial de conversaciones y crear mensajes antes de la operación asincrónica
    let messages = {
        let mut history = state.conversation_history.lock().unwrap();
        history.push(format!("Usuario: {}", message));

        // Consigue los datos del horario actuales
        let schedule_context = {
            let stored_data = state.schedule_data.lock().unwrap();
            if let Some(ref data) = *stored_data {
                format_schedule_context(data)
            } else {
                "No hay datos de horario disponibles actualmente.\n".to_string()
            }
        };

        // System prompt with schedule context
        let system_prompt = format!(
            "Eres un asistente IA especializado en ayudar con horarios escolares. Puedes modificar horarios de clase, sugerir movimientos óptimos, identificar conflictos y contestar preguntas acerca del horario.\n\n{}",
            schedule_context
        );

        // Prepara los mensajes para la API
        let mut messages = vec![Message {
            role: "system".to_string(),
            content: system_prompt,
        }];

        // Historial de la conversacion - FIXED: Use correct OpenAI role names
        for entry in history.iter() {
            if entry.starts_with("Usuario: ") {
                messages.push(Message {
                    role: "user".to_string(), // Changed from "usuario" to "user"
                    content: entry.replace("Usuario: ", ""),
                });
            } else if entry.starts_with("Asistente: ") {
                messages.push(Message {
                    role: "assistant".to_string(), // Changed from "asistente" to "assistant"
                    content: entry.replace("Asistente: ", ""),
                });
            }
        }

        messages
    }; // MutexGuard is dropped here

    // Prepare API request
    let client = reqwest::Client::new();
    let api_request = ApiRequest {
        model: "deepseek/deepseek-chat".to_string(), // Simplified model name
        messages,
        max_tokens: Some(1024), // Increased token limit
        temperature: Some(0.7),
        stream: false,
    };

    // Make API call to OpenRouter with better error handling
    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", state.api_key))
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "https://schedule-assistant.app")
        .header("X-Title", "Asistente para School Roster")
        .json(&api_request)
        .send()
        .await
        .map_err(|e| format!("API request error: {}", e))?;

    // Store the status code before consuming the response
    let status = response.status();
    
    // Get response text for debugging
    let response_text = response.text().await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    if !status.is_success() {
        return Err(format!("OpenRouter API error: {} - {}", status, response_text));
    }

    // Debug: Print response for troubleshooting
    println!("API Response: {}", response_text);

    // Try to parse the JSON response
    let api_response: ApiResponse = serde_json::from_str(&response_text)
        .map_err(|e| format!("Failed to parse OpenRouter API response: {} - Response body: {}", e, response_text))?;

    // Check for API errors
    if let Some(error) = api_response.error {
        return Err(format!("OpenRouter API returned error: {}", error.message));
    }

    if api_response.choices.is_empty() {
        return Err("No response from OpenRouter API".to_string());
    }

    let response_content = api_response.choices[0].message.content.clone();

    // Agrega la respuesta al historial
    {
        let mut history = state.conversation_history.lock().unwrap();
        history.push(format!("Asistente: {}", response_content));
    } // MutexGuard is dropped here

    Ok(response_content)
}

#[tauri::command]
pub async fn check_api_key(api_key: String) -> Result<String, String> {
    // Make a simple API call to verify the API key
    let client = reqwest::Client::new();
    let api_request = ApiRequest {
        model: "deepseek/deepseek-chat".to_string(), // Simplified model name
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
        .header("HTTP-Referer", "https://schedule-assistant.app")
        .header("X-Title", "Asistente para School Roster")
        .json(&api_request)
        .send()
        .await
        .map_err(|e| format!("API request error: {}", e))?;

    let status = response.status();
    let response_text = response.text().await.unwrap_or_default();

    if status.is_success() {
        // Try to parse to make sure it's valid JSON
        let _: ApiResponse = serde_json::from_str(&response_text)
            .map_err(|e| format!("API key validation failed - invalid response format: {}", e))?;
        
        Ok("OpenRouter API key is valid".to_string())
    } else {
        Err(format!(
            "Invalid OpenRouter API key: {} - {}",
            status, response_text
        ))
    }
}
