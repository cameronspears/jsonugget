extern crate dirs;
extern crate rfd;
extern crate serde_json;
extern crate chrono;

use chrono::Utc;
use rfd::FileDialog;
use serde_json::Value;
use std::fs::File;
use std::io::{Read, Write, stdout, stdin};
use std::path::Path;

// Function to read a JSON file and return its content as a String
fn read_json_file(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    println!("Reading JSON file...");
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("Successfully read JSON file.");
    Ok(contents)
}

// Function to write the summary to a JSON file in the Downloads folder
fn write_summary_to_file(summary: &Value) -> Result<(), Box<dyn std::error::Error>> {
    println!("Writing summary to file...");
    if let Some(mut path) = dirs::download_dir() {
        let datetime = Utc::now().format("%Y%m%d%H%M%S").to_string();
        path.push(format!("nugget_{}.json", datetime));

        let summary_str = serde_json::to_string_pretty(summary)?;
        let mut summary_file = File::create(path)?;
        summary_file.write_all(summary_str.as_bytes())?;
        println!("Summary saved as 'nugget_{}.json' in the Downloads folder.", datetime);
        Ok(())
    } else {
        Err(Box::from("Could not find Downloads directory"))
    }
}

// Function to get type as a string
fn get_type_as_string(value: &Value) -> String {
    match value {
        Value::String(_) => "String".to_string(),
        Value::Number(_) => "Number".to_string(),
        Value::Bool(_) => "Boolean".to_string(),
        Value::Null => "Null".to_string(),
        _ => "Unknown".to_string(),
    }
}

// Function to summarize a JSON Object
fn summarize_object(map: &serde_json::map::Map<String, Value>) -> Value {
    let mut summary = serde_json::map::Map::new();
    summary.insert("type".to_string(), Value::String("Object".to_string()));
    let mut nested_summaries = serde_json::map::Map::new();
    for (key, sub_value) in map.iter() {
        nested_summaries.insert(key.clone(), summarize_json(sub_value));
    }
    summary.insert("keys".to_string(), Value::Object(nested_summaries));
    Value::Object(summary)
}

// Function to summarize a JSON Array
fn summarize_array(arr: &Vec<Value>) -> Value {
    let mut summary = serde_json::map::Map::new();
    summary.insert("type".to_string(), Value::String("Array".to_string()));
    if let Some(first) = arr.first() {
        summary.insert("first_element_type".to_string(), summarize_json(first));
    } else {
        summary.insert("first_element_type".to_string(), Value::Null);
    }
    Value::Object(summary)
}

// Main function to summarize a JSON value
fn summarize_json(value: &Value) -> Value {
    match value {
        Value::Object(map) => summarize_object(map),
        Value::Array(arr) => summarize_array(arr),
        _ => Value::String(get_type_as_string(value)),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to jsonugget!");

    // Prompt the user to select a file
    println!("Please select a JSON file to summarize...");

    let file_path = FileDialog::new()
        .add_filter("JSON Files", &["json"])
        .pick_file();

    match file_path {
        Some(path) => {
            let contents = read_json_file(&path)?;
            match serde_json::from_str(&contents) {
                Ok(json_value) => {
                    println!("Summarizing JSON structure...");
                    let summary = summarize_json(&json_value);
                    write_summary_to_file(&summary)?;
                }
                Err(_) => println!("Invalid JSON file."),
            }
        }
        None => println!("No file selected."),
    }

    // Keep the terminal open until the user presses Enter
    println!("Press Enter to continue...");
    stdout().flush().unwrap();
    let mut _dummy = String::new();
    stdin().read_line(&mut _dummy).unwrap();

    Ok(())
}
