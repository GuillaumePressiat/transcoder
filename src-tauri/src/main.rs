#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
 
use serde::{Deserialize, Serialize};
use tauri_plugin_clipboard_manager::ClipboardExt;
 
// ── Types ──────────────────────────────────────────────────────────────────
 
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OutputType {
    List,
    Sql,
    Pipe,
}
 
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Quote {
    Single,
    Double,
    None,
}
 
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Separator {
    CommaSpace,
    Space,
    Comma,
}
 
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TranscodeResult {
    pub output: String,
    pub count: usize,
}
 
// ── Core logic ─────────────────────────────────────────────────────────────
 
fn quote_char(q: &Quote) -> &'static str {
    match q {
        Quote::Single => "'",
        Quote::Double => "\"",
        Quote::None => "",
    }
}
 
fn sep_str(s: &Separator) -> &'static str {
    match s {
        Separator::CommaSpace => ", ",
        Separator::Space => " ",
        Separator::Comma => ",",
    }
}
 
fn enrobeur(items: &[&str], quote: &Quote, sep: &Separator, output_type: &OutputType) -> String {
    let q = quote_char(quote);
    match output_type {
        OutputType::List => items
            .iter()
            .map(|s| format!("{}{}{}", q, s, q))
            .collect::<Vec<_>>()
            .join(sep_str(sep)),
        OutputType::Pipe => items.join("|"),
        OutputType::Sql => items
            .iter()
            .map(|s| format!("{}%{}%{}", q, s, q))
            .collect::<Vec<_>>()
            .join(" | \n"),
    }
}
 
fn split_line<'a>(line: &'a str, sep: &Separator) -> Vec<&'a str> {
    line.split(sep_str(sep)).map(|x| x.trim()).filter(|x| !x.is_empty()).collect()
}
 
fn split_column(text: &str) -> Vec<&str> {
    text.lines().map(|x| x.trim()).filter(|x| !x.is_empty()).collect()
}
 
// ── Tauri commands ─────────────────────────────────────────────────────────
 
#[tauri::command]
fn transcode_line(
    input: String,
    output_type: OutputType,
    quote: Quote,
    separator: Separator,
) -> TranscodeResult {
    let items = split_line(&input, &separator);
    let count = items.len();
    let output = enrobeur(&items, &quote, &separator, &output_type);
    TranscodeResult { output, count }
}
 
#[tauri::command]
fn transcode_column(
    input: String,
    output_type: OutputType,
    quote: Quote,
    separator: Separator,
) -> TranscodeResult {
    let items = split_column(&input);
    let count = items.len();
    let output = enrobeur(&items, &quote, &separator, &output_type);
    TranscodeResult { output, count }
}
 
#[tauri::command]
fn convert_format(input: String, direction: String, separator: Separator) -> TranscodeResult {
    let sep = sep_str(&separator);
    let output = if direction == "column_to_line" {
        input.lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>()
            .join(sep)
    } else {
        input
            .split(sep)
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    };
    let count = output.lines().count().max(output.split(sep).count());
    TranscodeResult { output, count }
}
 
#[tauri::command]
async fn copy_to_clipboard(app: tauri::AppHandle, text: String) -> Result<(), String> {
    app.clipboard().write_text(text).map_err(|e| e.to_string())
}
 
// ── Main ───────────────────────────────────────────────────────────────────
 
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            transcode_line,
            transcode_column,
            convert_format,
            copy_to_clipboard,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
