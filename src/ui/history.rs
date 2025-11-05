use crate::models::DayData;

const MAX_HISTORY_DEPTH: usize = 50;

#[derive(Debug)]
pub struct History {
    undo_stack: Vec<DayData>,
    redo_stack: Vec<DayData>,
}

impl History {
    pub fn new() -> Self {
        History {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn push(&mut self, state: DayData) {
        if self.undo_stack.len() >= MAX_HISTORY_DEPTH {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(state);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self, current_state: DayData) -> Option<DayData> {
        if let Some(previous_state) = self.undo_stack.pop() {
            self.redo_stack.push(current_state);
            Some(previous_state)
        } else {
            None
        }
    }

    pub fn redo(&mut self, current_state: DayData) -> Option<DayData> {
        if let Some(next_state) = self.redo_stack.pop() {
            self.undo_stack.push(current_state);
            Some(next_state)
        } else {
            None
        }
    }
}
