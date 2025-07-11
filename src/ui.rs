use pancurses::{Window, COLOR_PAIR};

pub const REGULAR_PAIR: u32 = 0;
pub const HIGHLIGHT_PAIR: u32 = 1;
pub const TITLE_PAIR: u32 = 2;
pub const INFO_PAIR: u32 = 3;
pub const UNIQUE_PAIR: u32 = 4;

#[derive(Default)]
pub(crate) struct UI {
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

    // prints the row
    pub fn label(&mut self, win: &Window, text: &str, pair: u32) {
        win.mv(self.row as i32, 0);
        win.attron(COLOR_PAIR(pair));
        win.addstr(text);
        win.attroff(COLOR_PAIR(pair));
        self.row += 1;
    }

    // print element based on whether it should be regular or highlighted
    pub fn print_element(&mut self, win: &Window, text: &str, id: usize) {
        let curr_id = self.curr_task.expect("error: curr_task is None");
        let pair = if curr_id == id {
            HIGHLIGHT_PAIR
        } else {
            REGULAR_PAIR
        };
        self.label(win, text, pair);
    }

    pub fn end_list(&mut self) {
        self.curr_task = None;
    }
}
