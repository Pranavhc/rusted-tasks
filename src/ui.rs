#![allow(dead_code)]

extern crate pancurses;
use pancurses::{Window, COLOR_PAIR};

pub const REGULAR_PAIR: u64 = 0;
pub const HIGHLIGHT_PAIR: u64 = 1;
pub const TITLE_PAIR: u64 = 2;
pub const INFO_PAIR: u64 = 3;

#[derive(Default)]
pub struct UI {
    curr_task: Option<usize>,
    row: usize,
}

impl UI {
    // row is where ui starts redering
    pub fn begin(&mut self, row: usize) {
        self.row = row;
    }

    // list starts from given index
    pub fn begin_list(&mut self, id: usize) {
        self.curr_task = Some(id);
    }

    // print element based on whether it should be regular or highlighted
    pub fn print_element(&mut self, window: &Window, text: &str, id: usize) {
        let curr_id = self.curr_task.expect("error!");
        let pair = if curr_id == id {
            HIGHLIGHT_PAIR
        } else {
            REGULAR_PAIR
        };
        self.label(window, text, pair);
    }

    // prints the row
    pub fn label(&mut self, window: &Window, text: &str, pair: u64) {
        window.mv(self.row as i32, 0);
        window.attron(COLOR_PAIR(pair));
        window.addstr(text);
        window.attroff(COLOR_PAIR(pair));
        self.row += 1;
    }

    pub fn end_list(&mut self) {
        self.curr_task = None;
    }
}
