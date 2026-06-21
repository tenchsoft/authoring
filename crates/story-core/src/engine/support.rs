use super::*;

impl StoryEngine {
    // ----- bookmark CRUD -----

    pub fn add_bookmark(
        &mut self,
        chapter_id: &str,
        block_idx: usize,
        offset: usize,
        label: &str,
    ) -> String {
        self.push_undo();
        let id = self.next_id();
        let mut bm = Bookmark::new(&id, chapter_id, block_idx, offset);
        bm.label = label.to_string();
        self.document.bookmarks.push(bm);
        self.dirty = true;
        id
    }

    pub fn delete_bookmark(&mut self, bookmark_id: &str) -> bool {
        self.push_undo();
        let before = self.document.bookmarks.len();
        self.document.bookmarks.retain(|b| b.id != bookmark_id);
        self.dirty = true;
        self.document.bookmarks.len() < before
    }

    // ----- comment CRUD -----

    pub fn add_comment(&mut self, chapter_id: &str, text: &str) -> String {
        self.push_undo();
        let id = self.next_id();
        let comment = StoryComment::new(&id, chapter_id, text);
        self.document.comments.push(comment);
        self.dirty = true;
        id
    }

    pub fn delete_comment(&mut self, comment_id: &str) -> bool {
        self.push_undo();
        let before = self.document.comments.len();
        self.document.comments.retain(|c| c.id != comment_id);
        self.dirty = true;
        self.document.comments.len() < before
    }

    pub fn resolve_comment(&mut self, comment_id: &str) -> bool {
        if let Some(c) = self
            .document
            .comments
            .iter_mut()
            .find(|c| c.id == comment_id)
        {
            c.resolved = true;
            self.dirty = true;
            true
        } else {
            false
        }
    }

    // ----- version history -----

    /// Save a version snapshot.
    pub fn save_version(&mut self, label: &str) -> String {
        let id = self.next_id();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let snapshot_json = self.document.to_json().unwrap_or_default();
        let snapshot = VersionSnapshot::new(&id, label, timestamp, snapshot_json);
        self.document.version_history.push(snapshot);
        self.dirty = true;
        id
    }

    /// Restore a version snapshot by ID.
    pub fn restore_version(&mut self, version_id: &str) -> bool {
        let snapshot_json = {
            let doc = &self.document;
            doc.version_history
                .iter()
                .find(|v| v.id == version_id)
                .map(|s| s.snapshot_json.clone())
        };
        if let Some(json) = snapshot_json {
            self.push_undo();
            match StoryDocument::from_json(&json) {
                Ok(doc) => {
                    self.document = doc;
                    self.dirty = true;
                    true
                }
                Err(_) => false,
            }
        } else {
            false
        }
    }

    // ----- writing goals -----

    pub fn set_writing_goal(&mut self, target_words: usize, period: &str) -> String {
        self.push_undo();
        let id = self.next_id();
        let goal = WritingGoal::new(&id, target_words, period);
        self.document.writing_goals.push(goal);
        self.dirty = true;
        id
    }

    pub fn update_goal_progress(&mut self, goal_id: &str, progress: usize) -> bool {
        if let Some(goal) = self
            .document
            .writing_goals
            .iter_mut()
            .find(|g| g.id == goal_id)
        {
            goal.progress = progress;
            self.dirty = true;
            true
        } else {
            false
        }
    }
}
