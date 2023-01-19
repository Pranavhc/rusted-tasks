use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Task {
    pub text: String,
    pub status: char,
}

impl Task {
    pub fn new(task: &str) -> Self {
        Self {
            text: task.trim().to_string(),
            status: '×',
        }
    }

    pub fn toggle_status(&mut self) {
        self.status = if self.status == '✓' { '×' } else { '✓' };
    }
}
