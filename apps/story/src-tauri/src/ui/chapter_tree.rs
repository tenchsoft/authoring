//! Chapter tree — rendering and hit-testing for the left panel chapter list.
//!
//! Uses real chapter data from the engine instead of hardcoded mock data.

use tench_ui::prelude::Point;

/// Compute the chapter list items from the engine document.
pub fn chapter_titles(state: &super::state::StoryState) -> Vec<String> {
    state
        .engine
        .get_document()
        .chapters
        .iter()
        .map(|ch| ch.title.clone())
        .collect()
}

/// Get the title of a chapter by index.
pub fn title(state: &super::state::StoryState, index: usize) -> String {
    state
        .engine
        .get_document()
        .chapters
        .get(index)
        .map(|ch| ch.title.clone())
        .unwrap_or_else(|| "Untitled".to_string())
}

/// Hit-test a point against the chapter list.
///
/// Returns the chapter index if the point falls within a chapter row.
pub fn hit_test(point: Point, chapter_count: usize) -> Option<usize> {
    if point.x >= 220.0 || point.y < 92.0 {
        return None;
    }
    let idx = ((point.y - 92.0) / 36.0) as usize;
    (idx < chapter_count).then_some(idx)
}
