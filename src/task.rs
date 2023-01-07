#![allow(dead_code)]

pub struct Task {
    pub text: String,
    pub status: char,
}

impl Task {
    pub fn new(task: &str) -> Self {
        Self {
            text: task.to_string(),
            status: '×',
        }
    }

    pub fn set_status(&mut self, status: char) {
        self.status = status;
    }

    pub fn toggle_status(&mut self) {
        if self.status == '✓' {
            self.status = '×';
        } else {
            self.status = '✓';
        }
    }
}
