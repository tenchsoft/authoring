//! Project bundle I/O — save/load story projects as ZIP archives.
//!
//! The default save format is a ZIP containing:
//! - `story.json` — the serialized `StoryDocument`
//! - `assets/` — embedded images and other binary assets

use std::fs;
use std::io::{Cursor, Read, Write};
use std::path::Path;

use crate::models::StoryDocument;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

/// File extension for story project bundles.
pub const STORY_BUNDLE_EXT: &str = "tench-story";

/// Save a story project as a ZIP bundle.
pub fn save_project_bundle(doc: &StoryDocument, path: &Path) -> Result<(), String> {
    let json = doc.to_json()?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {e}"))?;
    }

    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o644);

    // Write the main document JSON.
    writer
        .start_file("story.json", options)
        .map_err(|e| format!("Failed to create story.json: {e}"))?;
    writer
        .write_all(json.as_bytes())
        .map_err(|e| format!("Failed to write story.json: {e}"))?;

    let cursor = writer
        .finish()
        .map_err(|e| format!("Failed to finish ZIP: {e}"))?;
    let bytes = cursor.into_inner();
    fs::write(path, bytes).map_err(|e| format!("Failed to write file: {e}"))?;

    Ok(())
}

/// Load a story project from a ZIP bundle.
pub fn load_project_bundle(path: &Path) -> Result<StoryDocument, String> {
    let file = fs::File::open(path).map_err(|e| format!("Failed to open file: {e}"))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Failed to read ZIP: {e}"))?;

    let mut json = String::new();
    archive
        .by_name("story.json")
        .map_err(|e| format!("Missing story.json: {e}"))?
        .read_to_string(&mut json)
        .map_err(|e| format!("Failed to read story.json: {e}"))?;

    StoryDocument::from_json(&json)
}

/// Create an auto-save backup of the project.
pub fn auto_save_backup(doc: &StoryDocument, backup_dir: &Path) -> Result<(), String> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let filename = format!("autosave_{}.{}", timestamp, STORY_BUNDLE_EXT);
    let path = backup_dir.join(&filename);

    fs::create_dir_all(backup_dir)
        .map_err(|e| format!("Failed to create backup directory: {e}"))?;

    save_project_bundle(doc, &path)
}

/// Clean old auto-save backups, keeping only the most recent `keep` files.
pub fn cleanup_old_backups(backup_dir: &Path, keep: usize) -> Result<(), String> {
    if !backup_dir.exists() {
        return Ok(());
    }

    let mut entries: Vec<_> = fs::read_dir(backup_dir)
        .map_err(|e| format!("Failed to read backup dir: {e}"))?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == STORY_BUNDLE_EXT)
        })
        .collect();

    if entries.len() <= keep {
        return Ok(());
    }

    // Sort by modification time, newest first.
    entries.sort_by(|a, b| {
        let a_time = a.metadata().and_then(|m| m.modified()).ok();
        let b_time = b.metadata().and_then(|m| m.modified()).ok();
        b_time.cmp(&a_time)
    });

    for entry in entries.iter().skip(keep) {
        let _ = fs::remove_file(entry.path());
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// File format exports
// ---------------------------------------------------------------------------

use tench_document_core::TenchDocument;
use tench_office_io::docs;

/// Export the story as a single concatenated TenchDocument.
pub fn story_to_single_document(doc: &StoryDocument) -> TenchDocument {
    let mut combined = TenchDocument::new(&doc.metadata.title);
    combined.metadata.author = doc.metadata.author.clone();

    for chapter in &doc.chapters {
        // Add chapter heading.
        combined
            .content
            .push(tench_document_core::BlockNode::Heading {
                level: 1,
                content: vec![tench_document_core::InlineNode::Text {
                    text: chapter.title.clone(),
                    marks: tench_document_core::Marks::default(),
                }],
                attrs: tench_document_core::ParagraphAttrs::default(),
            });

        // Add chapter content blocks.
        combined.content.extend(chapter.content.content.clone());

        // Add page break between chapters.
        combined
            .content
            .push(tench_document_core::BlockNode::PageBreak);
    }

    combined
}

/// Export the story as DOCX bytes.
pub fn export_docx(doc: &StoryDocument) -> Result<Vec<u8>, String> {
    let single = story_to_single_document(doc);
    docs::export_docx_bytes(&single)
}

/// Import a DOCX file and convert to chapters.
pub fn import_docx(path: &Path) -> Result<StoryDocument, String> {
    let tdm = docs::import_docx(path)?;
    let mut story = StoryDocument::new(&tdm.metadata.title);
    story.metadata.author = tdm.metadata.author.clone();

    // Split document content into chapters at Heading 1 boundaries.
    let mut current_blocks = Vec::new();
    let mut current_title = "Chapter 1".to_string();
    let mut chapter_idx = 0;
    let mut has_headings = false;

    for block in tdm.content {
        if let tench_document_core::BlockNode::Heading { level, content, .. } = &block {
            if *level == 1 {
                has_headings = true;
                // Save previous chapter if it has content.
                if !current_blocks.is_empty() || chapter_idx > 0 {
                    let id = format!("ch_{chapter_idx}");
                    let mut ch = crate::models::Chapter::new(&id, &current_title, chapter_idx);
                    ch.content = TenchDocument {
                        content: std::mem::take(&mut current_blocks),
                        ..TenchDocument::new(&current_title)
                    };
                    story.chapters.push(ch);
                    chapter_idx += 1;
                }
                // Extract heading text.
                current_title = content
                    .iter()
                    .map(|n| match n {
                        tench_document_core::InlineNode::Text { text, .. } => text.clone(),
                        tench_document_core::InlineNode::Link { text, .. } => text.clone(),
                        _ => String::new(),
                    })
                    .collect();
                continue;
            }
        }
        current_blocks.push(block);
    }

    // Save the last chapter.
    if !current_blocks.is_empty() || (chapter_idx > 0 && has_headings) {
        let id = format!("ch_{chapter_idx}");
        let mut ch = crate::models::Chapter::new(&id, &current_title, chapter_idx);
        ch.content = TenchDocument {
            content: current_blocks,
            ..TenchDocument::new(&current_title)
        };
        story.chapters.push(ch);
    }

    // If no chapters were created, put everything in one chapter.
    if story.chapters.is_empty() {
        let id = "ch_0".to_string();
        let mut ch = crate::models::Chapter::new(&id, "Chapter 1", 0);
        ch.content = TenchDocument::new("Chapter 1");
        story.chapters.push(ch);
    }

    Ok(story)
}

/// Export the story as PDF bytes.
pub fn export_pdf(doc: &StoryDocument) -> Result<Vec<u8>, String> {
    let single = story_to_single_document(doc);
    docs::export_pdf_bytes_from_content(&tench_document_core::OfficeContent::Docs(
        tench_document_core::RichDocumentContent {
            schema: String::new(),
            document: Some(single),
        },
    ))
}

/// Export the story as EPUB bytes.
pub fn export_epub(doc: &StoryDocument) -> Result<Vec<u8>, String> {
    let single = story_to_single_document(doc);
    docs::export_epub_bytes_from_content(&tench_document_core::OfficeContent::Docs(
        tench_document_core::RichDocumentContent {
            schema: String::new(),
            document: Some(single),
        },
    ))
}

/// Export the story as Markdown.
pub fn export_markdown(doc: &StoryDocument) -> String {
    let mut md = String::new();

    // Frontmatter.
    md.push_str(&format!(
        "---\ntitle: {}\nauthor: {}\n---\n\n",
        doc.metadata.title,
        doc.metadata.author.as_deref().unwrap_or("Unknown")
    ));

    // TOC.
    if !doc.chapters.is_empty() {
        md.push_str("## Table of Contents\n\n");
        for (i, ch) in doc.chapters.iter().enumerate() {
            let slug = slugify(&ch.title);
            md.push_str(&format!("{}. [{}](#{})\n", i + 1, ch.title, slug));
        }
        md.push('\n');
    }

    // Chapters.
    for chapter in &doc.chapters {
        md.push_str(&format!("# {}\n\n", chapter.title));
        md.push_str(&chapter.content.to_plain_text());
        md.push_str("\n\n");
    }

    md
}

/// Import a Markdown file and split into chapters.
pub fn import_markdown(text: &str) -> StoryDocument {
    let mut story = StoryDocument::new("Imported");
    let mut current_title = "Chapter 1".to_string();
    let mut current_lines: Vec<String> = Vec::new();
    let mut chapter_idx = 0;

    for line in text.lines() {
        if line.starts_with("# ") && !line.starts_with("## ") {
            // New chapter heading.
            if !current_lines.is_empty() || chapter_idx > 0 {
                let id = format!("ch_{chapter_idx}");
                let content = current_lines.join("\n");
                let mut ch = crate::models::Chapter::new(&id, &current_title, chapter_idx);
                ch.content = TenchDocument::plain_text(&content);
                story.chapters.push(ch);
                chapter_idx += 1;
            }
            current_title = line.trim_start_matches("# ").to_string();
            current_lines.clear();
        } else {
            current_lines.push(line.to_string());
        }
    }

    // Last chapter.
    if !current_lines.is_empty() {
        let id = format!("ch_{chapter_idx}");
        let content = current_lines.join("\n");
        let mut ch = crate::models::Chapter::new(&id, &current_title, chapter_idx);
        ch.content = TenchDocument::plain_text(&content);
        story.chapters.push(ch);
    }

    if story.chapters.is_empty() {
        let id = "ch_0".to_string();
        let mut ch = crate::models::Chapter::new(&id, "Chapter 1", 0);
        ch.content = TenchDocument::plain_text(text);
        story.chapters.push(ch);
    }

    story
}

/// Export the story as HTML.
pub fn export_html(doc: &StoryDocument) -> String {
    let mut html = String::from(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\n",
    );
    html.push_str(&format!("<title>{}</title>\n", doc.metadata.title));
    html.push_str(
        "<style>\nbody{font-family:system-ui,sans-serif;max-width:800px;\
         margin:2rem auto;padding:0 1rem;line-height:1.8;}\n\
         h1,h2,h3{margin-top:1.5em;}\nhr{border:none;border-top:1px solid #ccc;}\n\
         </style>\n</head>\n<body>\n",
    );

    for chapter in &doc.chapters {
        html.push_str(&format!("<h1>{}</h1>\n", chapter.title));
        for line in chapter.content.to_plain_text().lines() {
            if line.is_empty() {
                html.push_str("<br>\n");
            } else {
                html.push_str(&format!("<p>{}</p>\n", line));
            }
        }
        html.push_str("<hr>\n");
    }

    html.push_str("</body>\n</html>");
    html
}

/// Export the story as plain text.
pub fn export_txt(doc: &StoryDocument) -> String {
    let mut text = String::new();
    for chapter in &doc.chapters {
        text.push_str(&format!("{}\n\n", chapter.title));
        text.push_str(&chapter.content.to_plain_text());
        text.push_str("\n\n---\n\n");
    }
    text
}

fn slugify(s: &str) -> String {
    let mut slug = String::new();
    for ch in s.chars() {
        if ch.is_alphanumeric() {
            slug.extend(ch.to_lowercase());
        } else if (ch == ' ' || ch == '-' || ch == '_') && !slug.ends_with('-') && !slug.is_empty()
        {
            slug.push('-');
        }
    }
    slug.trim_end_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    #[test]
    fn save_load_roundtrip() {
        let mut story = StoryDocument::new("Test Novel");
        story.metadata.author = Some("Author".to_string());
        story.chapters.push(Chapter::new("ch1", "Chapter 1", 0));
        story.chapters[0].content = TenchDocument::plain_text("Hello world");

        let dir = std::env::temp_dir().join("tench_story_test_save");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("test.tench-story");

        save_project_bundle(&story, &path).unwrap();
        let loaded = load_project_bundle(&path).unwrap();

        assert_eq!(loaded.metadata.title, "Test Novel");
        assert_eq!(loaded.metadata.author, Some("Author".to_string()));
        assert_eq!(loaded.chapters.len(), 1);
        assert_eq!(loaded.chapters[0].content.to_plain_text(), "Hello world");

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn export_markdown_contains_chapters() {
        let mut story = StoryDocument::new("Test");
        story.chapters.push(Chapter::new("ch1", "Intro", 0));
        story.chapters[0].content = TenchDocument::plain_text("Hello world");
        story.chapters.push(Chapter::new("ch2", "Middle", 1));
        story.chapters[1].content = TenchDocument::plain_text("The end");

        let md = export_markdown(&story);
        assert!(md.contains("# Intro"));
        assert!(md.contains("# Middle"));
        assert!(md.contains("Hello world"));
        assert!(md.contains("The end"));
        assert!(md.contains("Table of Contents"));
    }

    #[test]
    fn import_markdown_splits_chapters() {
        let md = "# Chapter One\n\nHello world\n\n# Chapter Two\n\nGoodbye";
        let story = import_markdown(md);
        assert_eq!(story.chapters.len(), 2);
        assert_eq!(story.chapters[0].title, "Chapter One");
        assert_eq!(story.chapters[1].title, "Chapter Two");
    }

    #[test]
    fn export_html_structure() {
        let mut story = StoryDocument::new("Test");
        story.chapters.push(Chapter::new("ch1", "Ch 1", 0));
        story.chapters[0].content = TenchDocument::plain_text("Content");

        let html = export_html(&story);
        assert!(html.contains("<h1>Ch 1</h1>"));
        assert!(html.contains("<p>Content</p>"));
    }

    #[test]
    fn export_txt_content() {
        let mut story = StoryDocument::new("Test");
        story.chapters.push(Chapter::new("ch1", "Ch 1", 0));
        story.chapters[0].content = TenchDocument::plain_text("Hello");

        let txt = export_txt(&story);
        assert!(txt.contains("Ch 1"));
        assert!(txt.contains("Hello"));
    }

    #[test]
    fn story_to_single_document_combines_chapters() {
        let mut story = StoryDocument::new("Test");
        story.chapters.push(Chapter::new("ch1", "Chapter 1", 0));
        story.chapters[0].content = TenchDocument::plain_text("Hello");
        story.chapters.push(Chapter::new("ch2", "Chapter 2", 1));
        story.chapters[1].content = TenchDocument::plain_text("World");

        let doc = story_to_single_document(&story);
        assert!(doc.content.len() >= 4); // 2 headings + 2 paragraphs + page break
    }

    #[test]
    fn slugify_handles_special() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("서울의 그림자"), "서울의-그림자");
    }
}
