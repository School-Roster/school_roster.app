// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use school_schedule::class::teacher;
use school_schedule::class::subject;
use school_schedule::class::group;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

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
fn register_subject(name: &str, shorten: &str, color: &str, stype: &str) -> String
{
    let res: String = subject::register(name, shorten, color, stype);
    format!("{}", res)
}

#[tauri::command]
fn list_subjects() -> Vec<subject::Subject>
{
    subject::get_all()
}

#[tauri::command]
fn register_group(grade: u8, group: String, spec: Option<String>, shift: String, students_count: i32)
    -> String
{
    let check = group::register(grade, group, spec, shift, students_count);
    match check {
        true => String::from("Group registered successfully!"),
        false => format!("Error registering group: {}", check),
    }
}

#[tauri::command]
fn list_groups() -> Vec<group::Group>
{
    group::get_groups()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_teachers,
            register_teacher,
            register_subject,
            list_subjects,
            register_group,
            list_groups,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
