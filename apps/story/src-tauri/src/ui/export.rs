//! Export helpers — word count labels and format lists.

/// Format a word count as a display label.
pub fn word_count_label(word_count: usize) -> String {
    format!("{word_count} words")
}

/// Legacy export formats list (used by export modal).
pub fn export_formats() -> [&'static str; 4] {
    ["DOCX", "PDF", "EPUB", "Markdown"]
}
