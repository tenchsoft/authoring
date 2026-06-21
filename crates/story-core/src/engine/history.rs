use super::*;

impl StoryEngine {
    // ----- undo / redo -----

    pub fn undo(&mut self) -> bool {
        let Some(snapshot) = self.undo_stack.pop() else {
            return false;
        };
        let current = UndoSnapshot {
            document: self.document.clone(),
        };
        self.redo_stack.push(current);
        self.document = snapshot.document;
        self.dirty = true;
        true
    }

    pub fn redo(&mut self) -> bool {
        let Some(snapshot) = self.redo_stack.pop() else {
            return false;
        };
        let current = UndoSnapshot {
            document: self.document.clone(),
        };
        self.undo_stack.push(current);
        self.document = snapshot.document;
        self.dirty = true;
        true
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
