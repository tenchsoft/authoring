//! Tauri IPC commands — bridge between the frontend and the Rust backend.
//!
//! These commands expose file I/O, export/import, and dialog operations
//! to the Tauri webview. The actual document editing happens in
//! `tench_story_core::StoryEngine` on the Rust side.

use std::fs;
use std::path::Path;
use tench_workspace_core::{WorkspaceRuntimeCapabilities, STORY_RUNTIME};

use crate::markdown_io;

/// Return the runtime capabilities for the Story app.
#[tauri::command]
pub fn runtime_capabilities() -> WorkspaceRuntimeCapabilities {
    STORY_RUNTIME.clone()
}

/// Export the full story as a single Markdown file.
#[tauri::command]
pub fn export_to_markdown(content_json: String, output_path: String) -> Result<String, String> {
    let markdown = markdown_io::story_chapters_to_markdown(&content_json)?;

    let path = Path::new(&output_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {e}"))?;
    }

    fs::write(path, &markdown).map_err(|e| format!("Failed to write file: {e}"))?;

    Ok(output_path)
}

/// Export story as HTML file.
#[tauri::command]
pub fn export_to_html(content_json: String, output_path: String) -> Result<String, String> {
    let markdown = markdown_io::story_chapters_to_markdown(&content_json)?;
    let html_body = markdown_to_html(&markdown);
    let html = format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\n\
         <title>Story Export</title>\n<style>\nbody{{font-family:system-ui,sans-serif;\
         max-width:800px;margin:2rem auto;padding:0 1rem;line-height:1.8;}}\n\
         h1,h2,h3{{margin-top:1.5em;}}\n</style>\n</head>\n<body>\n{}\n</body>\n</html>",
        html_body
    );

    let path = Path::new(&output_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {e}"))?;
    }
    fs::write(path, &html).map_err(|e| format!("Failed to write file: {e}"))?;
    Ok(output_path)
}

/// Export story as plain text.
#[tauri::command]
pub fn export_to_txt(content_json: String, output_path: String) -> Result<String, String> {
    let markdown = markdown_io::story_chapters_to_markdown(&content_json)?;
    let text = markdown
        .replace("# ", "")
        .replace("## ", "")
        .replace("### ", "")
        .replace("**", "")
        .replace("__", "");

    let path = Path::new(&output_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {e}"))?;
    }
    fs::write(path, &text).map_err(|e| format!("Failed to write file: {e}"))?;
    Ok(output_path)
}

/// Save a story project bundle (.tench-story ZIP).
#[tauri::command]
pub fn save_project_bundle(json: String, output_path: String) -> Result<String, String> {
    let doc = tench_story_core::StoryDocument::from_json(&json)?;
    let path = Path::new(&output_path);
    tench_story_core::project_io::save_project_bundle(&doc, path)?;
    Ok(output_path)
}

/// Load a story project bundle (.tench-story ZIP).
#[tauri::command]
pub fn load_project_bundle(input_path: String) -> Result<String, String> {
    let path = Path::new(&input_path);
    let doc = tench_story_core::project_io::load_project_bundle(path)?;
    doc.to_json()
}

/// Export a story project as DOCX.
#[tauri::command]
pub fn export_docx(json: String, output_path: String) -> Result<String, String> {
    let doc = tench_story_core::StoryDocument::from_json(&json)?;
    let bytes = tench_story_core::project_io::export_docx(&doc)?;
    let path = Path::new(&output_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {e}"))?;
    }
    fs::write(path, &bytes).map_err(|e| format!("Failed to write DOCX: {e}"))?;
    Ok(output_path)
}

/// Export a story project as PDF.
#[tauri::command]
pub fn export_pdf(json: String, output_path: String) -> Result<String, String> {
    let doc = tench_story_core::StoryDocument::from_json(&json)?;
    let bytes = tench_story_core::project_io::export_pdf(&doc)?;
    let path = Path::new(&output_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {e}"))?;
    }
    fs::write(path, &bytes).map_err(|e| format!("Failed to write PDF: {e}"))?;
    Ok(output_path)
}

/// Export a story project as EPUB.
#[tauri::command]
pub fn export_epub(json: String, output_path: String) -> Result<String, String> {
    let doc = tench_story_core::StoryDocument::from_json(&json)?;
    let bytes = tench_story_core::project_io::export_epub(&doc)?;
    let path = Path::new(&output_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {e}"))?;
    }
    fs::write(path, &bytes).map_err(|e| format!("Failed to write EPUB: {e}"))?;
    Ok(output_path)
}

/// Export a story project as Markdown (using story-core).
#[tauri::command]
pub fn export_markdown(json: String, output_path: String) -> Result<String, String> {
    let doc = tench_story_core::StoryDocument::from_json(&json)?;
    let md = tench_story_core::project_io::export_markdown(&doc);
    let path = Path::new(&output_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {e}"))?;
    }
    fs::write(path, &md).map_err(|e| format!("Failed to write Markdown: {e}"))?;
    Ok(output_path)
}

/// Export a story project as HTML (using story-core).
#[tauri::command]
pub fn export_html(json: String, output_path: String) -> Result<String, String> {
    let doc = tench_story_core::StoryDocument::from_json(&json)?;
    let html = tench_story_core::project_io::export_html(&doc);
    let path = Path::new(&output_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {e}"))?;
    }
    fs::write(path, &html).map_err(|e| format!("Failed to write HTML: {e}"))?;
    Ok(output_path)
}

/// Export a story project as plain text (using story-core).
#[tauri::command]
pub fn export_txt(json: String, output_path: String) -> Result<String, String> {
    let doc = tench_story_core::StoryDocument::from_json(&json)?;
    let text = tench_story_core::project_io::export_txt(&doc);
    let path = Path::new(&output_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {e}"))?;
    }
    fs::write(path, &text).map_err(|e| format!("Failed to write TXT: {e}"))?;
    Ok(output_path)
}

/// Import a DOCX file and return a StoryDocument JSON.
#[tauri::command]
pub fn import_docx(input_path: String) -> Result<String, String> {
    let path = Path::new(&input_path);
    let doc = tench_story_core::project_io::import_docx(path)?;
    doc.to_json()
}

/// Import a Markdown file and return a StoryDocument JSON.
#[tauri::command]
pub fn import_markdown(content: String) -> Result<String, String> {
    let doc = tench_story_core::project_io::import_markdown(&content);
    doc.to_json()
}

/// Open a native save-file dialog and return the chosen path.
#[tauri::command]
pub async fn pick_save_path(
    default_name: String,
    app: tauri::AppHandle,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let file_path = app
        .dialog()
        .file()
        .set_file_name(&default_name)
        .add_filter("Story Bundle", &["tench-story"])
        .blocking_save_file();

    match file_path {
        Some(path) => Ok(Some(path.to_string())),
        None => Ok(None),
    }
}

/// Open a native save-file dialog for any format.
#[tauri::command]
pub async fn pick_export_path(
    default_name: String,
    filters: Vec<(String, Vec<String>)>,
    app: tauri::AppHandle,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let mut dialog = app.dialog().file().set_file_name(&default_name);
    for (name, exts) in &filters {
        let ext_refs: Vec<&str> = exts.iter().map(|s| s.as_str()).collect();
        dialog = dialog.add_filter(name, &ext_refs);
    }

    let file_path = dialog.blocking_save_file();
    match file_path {
        Some(path) => Ok(Some(path.to_string())),
        None => Ok(None),
    }
}

/// Open a native open-file dialog for importing.
#[tauri::command]
pub async fn pick_import_path(
    filters: Vec<(String, Vec<String>)>,
    app: tauri::AppHandle,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;

    let mut dialog = app.dialog().file();
    for (name, exts) in &filters {
        let ext_refs: Vec<&str> = exts.iter().map(|s| s.as_str()).collect();
        dialog = dialog.add_filter(name, &ext_refs);
    }

    let file_path = dialog.blocking_pick_file();
    match file_path {
        Some(path) => Ok(Some(path.to_string())),
        None => Ok(None),
    }
}

/// Read a text file from the given path.
#[tauri::command]
pub fn read_text_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {e}"))
}

/// Write a text file to the given path.
#[tauri::command]
pub fn write_text_file(path: String, content: String) -> Result<String, String> {
    let file_path = Path::new(&path);
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {e}"))?;
    }
    fs::write(file_path, &content).map_err(|e| format!("Failed to write file: {e}"))?;
    Ok(path)
}

// ── Helpers ──

/// Simple markdown-to-HTML converter (no external deps).
fn markdown_to_html(md: &str) -> String {
    let mut html = String::new();
    for line in md.lines() {
        if let Some(stripped) = line.strip_prefix("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", stripped));
        } else if let Some(stripped) = line.strip_prefix("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", stripped));
        } else if let Some(stripped) = line.strip_prefix("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", stripped));
        } else if line.starts_with("---") {
            html.push_str("<hr>\n");
        } else if line.starts_with("- ") || line.starts_with("* ") {
            html.push_str(&format!("<li>{}</li>\n", &line[2..]));
        } else if line.starts_with("1. ") || line.starts_with("2. ") || line.starts_with("3. ") {
            let content = line.split_once(". ").map(|x| x.1).unwrap_or(line);
            html.push_str(&format!("<li>{}</li>\n", content));
        } else if line.is_empty() {
            html.push_str("<br>\n");
        } else {
            html.push_str(&format!("<p>{}</p>\n", line));
        }
    }
    html
}
