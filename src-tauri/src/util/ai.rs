use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

/// Structure to store the AI state
pub struct AIState {
    api_key: String,
    conversation_history: Arc<Mutex<Vec<String>>>,
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
pub async fn init_model(api_key: String, handle: tauri::AppHandle) -> Result<String, String> {
    handle.manage(AIState {
        api_key,
        conversation_history: Arc::new(Mutex::new(Vec::new())),
    });

    Ok(String::from(
        "OpenRouter API connection initialized successfully",
    ))
}

#[tauri::command]
pub async fn query_ai(message: String, state: tauri::State<'_, AIState>) -> Result<String, String> {
    // Add message to conversation history and create messages before async operation
    let messages = {
        let mut history = state.conversation_history.lock().unwrap();
        history.push(format!("User: {}", message));

        // System prompt
        let system_prompt = r#"You are an AI assistant for a school scheduling application. You have deep knowledge of the application's structure and capabilities.

Key Entities:
1. Teachers:
   - Properties: name, father_lastname, mother_lastname, email, phone, degree, commisioned_hours, active_hours, performance
   - Can be assigned to multiple subjects
   - Have preferred days and modules for teaching

2. Subjects:
   - Properties: name, shorten (abbreviation), color, spec (special requirements), required_modules, priority
   - Can be assigned to multiple teachers
   - Have specific classroom requirements (spec field)

3. Classrooms:
   - Properties: building_id, building_number, building_type, capacity, availability
   - Have specific types (e.g., lab, regular classroom)
   - Track availability by day and module

4. Groups:
   - Properties: grade, group, career, students, max_modules_per_day
   - Have assigned subjects and teachers
   - Follow scheduling constraints

5. Assignments:
   - Links teachers, subjects, and classrooms to specific time slots
   - Tracks day and module_index for each assignment

Available Operations:
- Create, read, update, and delete operations for all entities
- Generate and modify schedules
- Import/export data
- Check for scheduling conflicts
- Validate teacher availability and preferences
- Ensure classroom requirements are met

Response Style Guidelines:
1. Be conversational and friendly
2. Use clear, simple language
3. Format responses with visual hierarchy:
   - Use emojis for main sections (e.g., 📋 for steps, ⚠️ for warnings)
   - Use bullet points (•) for lists
   - Use sub-bullets (◦) for details
   - Use bold for important terms
   - Add line breaks between sections

4. Structure responses as:
   - Brief friendly introduction
   - Clear steps with visual markers
   - Important notes or warnings
   - Next steps or follow-up actions

Example Response Format:
"Hi! I'll help you assign a teacher to a subject. Here's what we need to do:

📋 Step 1: Check Teacher Availability
   • Look at their current schedule
   • Check their preferred teaching times
   • Verify their available hours

📋 Step 2: Review Subject Details
   • Check required modules
   • Note any special requirements
   • Consider subject priority

📋 Step 3: Make the Assignment
   • Select suitable time slots
   • Confirm classroom availability
   • Save the assignment

⚠️ Important Notes:
   • Watch for scheduling conflicts
   • Consider teacher preferences
   • Check classroom requirements

What would you like to do first? I can help you check the teacher's availability or review the subject details."

You can help with:
- Schedule optimization
- Conflict resolution
- Resource allocation
- Constraint validation
- Data management
- Best practices for scheduling"#;

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
                    content: entry.replace("Assistant: ", ""),
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
        .header("X-Title", "School Schedule Assistant") // Replace with your app's name
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
        .header("X-Title", "School Schedule Assistant") // Replace with your app's name
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
