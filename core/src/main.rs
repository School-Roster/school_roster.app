// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use school_schedule::class::teacher;
use school_schedule::class::subject;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

#[tauri::command]
fn list_teachers() -> Vec<teacher::Teacher> {
    teacher::get_teachers()
}

#[tauri::command]
fn register_teacher(
    shorten: String,
    name: String,
    flastname: String,
    slastname: String,
    email: Option<String>,
    phone: Option<String>,
    degrees: Option<String>,
    commissioned: i32,
    active: i32,
    stars: u8,
) {
    teacher::register_teacher(
        shorten,
        name,
        flastname,
        slastname,
        email,
        phone,
        degrees,
        commissioned,
        active,
        stars,
    );

    println!("Teacher registered successfully!");
}

#[tauri::command]
fn register_subject(name: &str, shorten: &str, color: &str)
{
    subject::register(name, shorten, color);
}

#[tauri::command]
fn list_subjects() -> Vec<subject::Subject>
{
    subject::get_all()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_teachers,
            register_teacher,
            register_subject,
            list_subjects,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
