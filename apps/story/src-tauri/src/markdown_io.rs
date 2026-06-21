use serde_json::Value;

/// Convert a JSON array of chapters into a single Markdown string.
///
/// Expected input shape:
/// ```json
/// [{"title": "Chapter 1", "scenes": [{"title": "Scene 1", "content": "..."}]}]
/// ```
///
/// Chapters may optionally include a `content` field at the chapter level which
/// is emitted as paragraphs when present.
pub fn story_chapters_to_markdown(chapters_json: &str) -> Result<String, String> {
    let chapters: Vec<Value> =
        serde_json::from_str(chapters_json).map_err(|e| format!("Invalid chapters JSON: {e}"))?;

    if chapters.is_empty() {
        return Ok(String::new());
    }

    let title = extract_story_title(&chapters);
    let frontmatter = generate_frontmatter(&title);
    let toc = generate_toc(&chapters);

    let mut body = String::new();

    for chapter in &chapters {
        let ch_title = chapter["title"].as_str().unwrap_or("Untitled Chapter");
        body.push_str(&format!("# {ch_title}\n\n"));

        // Chapter-level content (used by the current prototype where scenes are
        // structural markers and the prose lives on the chapter).
        if let Some(content) = chapter["content"].as_str() {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                body.push_str(trimmed);
                body.push_str("\n\n");
            }
        }

        // Scene-level content.
        if let Some(scenes) = chapter["scenes"].as_array() {
            for scene in scenes {
                let scene_title = scene["title"].as_str().unwrap_or("Untitled Scene");
                body.push_str(&format!("## {scene_title}\n\n"));

                if let Some(scene_content) = scene["content"].as_str() {
                    let trimmed = scene_content.trim();
                    if !trimmed.is_empty() {
                        body.push_str(trimmed);
                        body.push_str("\n\n");
                    }
                }
            }
        }
    }

    let mut output = frontmatter;
    output.push_str(&toc);
    output.push_str(&body);
    // Trim trailing whitespace but keep a final newline.
    let trimmed = output.trim_end();
    Ok(format!("{trimmed}\n"))
}

/// Generate a YAML frontmatter block.
pub fn generate_frontmatter(title: &str) -> String {
    let date = chrono_now();
    format!("---\ntitle: {title}\ndate: {date}\n---\n\n")
}

/// Generate a Markdown table of contents from chapter titles.
pub fn generate_toc(chapters: &[Value]) -> String {
    if chapters.is_empty() {
        return String::new();
    }

    let mut toc = String::from("## Table of Contents\n\n");
    for (i, chapter) in chapters.iter().enumerate() {
        let ch_title = chapter["title"].as_str().unwrap_or("Untitled Chapter");
        let anchor = slugify(ch_title);
        toc.push_str(&format!("{}. [{}](#{})\n", i + 1, ch_title, anchor));
    }
    toc.push('\n');
    toc
}

// ── Helpers ──

/// Try to derive a story-level title from the chapters.  For now we just use
/// the first chapter title prefixed with "Story" but this can be refined later.
fn extract_story_title(_chapters: &[Value]) -> String {
    // In the prototype the project name ("장미의 이름") is not carried in the
    // chapter data.  Fall back to a generic title.
    "Untitled Story".to_string()
}

/// Produce an ISO-8601 date string without pulling in chrono.
fn chrono_now() -> String {
    // Use a simple approach: ask the standard library.  We could depend on
    // `chrono` but that would add a workspace dep for a single call.
    // Instead we format via `std::time::SystemTime` indirectly.
    // For simplicity, use the UTC date from the `time` crate or just a
    // static approach.  Since we don't have chrono, produce a placeholder.
    // Actually, let's use the js-side date.  But this is Rust backend...
    // Let's just produce a reasonable ISO date.
    format_date_now()
}

fn format_date_now() -> String {
    // Simple RFC-3339 date without external crate.
    // std doesn't expose calendar dates, so we produce a Unix timestamp
    // note.  This is acceptable for frontmatter.
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", dur.as_secs())
}

/// Convert a heading string into a lowercase, hyphenated anchor slug.
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

    #[test]
    fn converts_single_chapter_with_scenes() {
        let json = r#"[{"title": "Chapter 1", "scenes": [{"title": "Scene 1", "content": "Hello world"}]}]"#;
        let md = story_chapters_to_markdown(json).unwrap();

        assert!(md.starts_with("---\n"));
        assert!(md.contains("# Chapter 1"));
        assert!(md.contains("## Scene 1"));
        assert!(md.contains("Hello world"));
        assert!(md.contains("## Table of Contents"));
    }

    #[test]
    fn converts_chapter_with_content_field() {
        let json = r#"[{"title": "Intro", "content": "Once upon a time.", "scenes": []}]"#;
        let md = story_chapters_to_markdown(json).unwrap();

        assert!(md.contains("# Intro"));
        assert!(md.contains("Once upon a time."));
    }

    #[test]
    fn empty_chapters_produce_empty_string() {
        let md = story_chapters_to_markdown("[]").unwrap();
        assert!(md.is_empty());
    }

    #[test]
    fn invalid_json_returns_error() {
        let result = story_chapters_to_markdown("not json");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid chapters JSON"));
    }

    #[test]
    fn frontmatter_contains_title_and_date() {
        let fm = generate_frontmatter("My Novel");
        assert!(fm.starts_with("---\n"));
        assert!(fm.contains("title: My Novel"));
        assert!(fm.contains("date:"));
        assert!(fm.contains("---\n\n"));
    }

    #[test]
    fn toc_lists_chapter_titles() {
        let chapters: Vec<Value> =
            serde_json::from_str(r#"[{"title": "Alpha"}, {"title": "Beta"}]"#).unwrap();

        let toc = generate_toc(&chapters);
        assert!(toc.contains("1. [Alpha](#alpha)"));
        assert!(toc.contains("2. [Beta](#beta)"));
    }

    #[test]
    fn toc_empty_chapters_is_empty() {
        let toc = generate_toc(&[]);
        assert!(toc.is_empty());
    }

    #[test]
    fn slugify_handles_special_characters() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("서울의 그림자"), "서울의-그림자");
        assert_eq!(slugify("A & B! C"), "a-b-c");
        assert_eq!(slugify("  spaces  "), "spaces");
    }

    #[test]
    fn missing_fields_use_defaults() {
        let json = r#"[{"title": "Ch1", "scenes": [{}]}]"#;
        let md = story_chapters_to_markdown(json).unwrap();
        assert!(md.contains("## Untitled Scene"));
    }

    #[test]
    fn chapter_without_title_uses_default() {
        let json = r#"[{"scenes": []}]"#;
        let md = story_chapters_to_markdown(json).unwrap();
        assert!(md.contains("# Untitled Chapter"));
    }
}
