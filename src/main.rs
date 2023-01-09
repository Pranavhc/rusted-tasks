extern crate pancurses;
use pancurses::{
    curs_set, endwin, init_pair, initscr, noecho, start_color, Input, Window, COLOR_BLACK,
    COLOR_MAGENTA, COLOR_PAIR, COLOR_WHITE, COLOR_YELLOW,
};
use std::process::exit;

// - - - - -

struct Task {
    pub text: String,
    pub status: char,
}

impl Task {
    fn new(task: &str) -> Self {
        Self {
            text: task.trim().to_string(),
            status: '×',
        }
    }

    fn toggle_status(&mut self) {
        self.status = if self.status == '✓' { '×' } else { '✓' };
    }
}

// - - - - -

const REGULAR_PAIR: u64 = 0;
const HIGHLIGHT_PAIR: u64 = 1;
const TITLE_PAIR: u64 = 2;
const INFO_PAIR: u64 = 3;

#[derive(Default)]
struct UI {
    curr_task: Option<usize>,
    row: usize,
}

impl UI {
    // row is where ui starts redering
    fn begin(&mut self, row: usize) {
        self.row = row;
    }

    // list starts from given index
    fn begin_list(&mut self, id: usize) {
        self.curr_task = Some(id);
    }

    // print element based on whether it should be regular or highlighted
    fn print_element(&mut self, win: &Window, text: &str, id: usize) {
        let curr_id = self.curr_task.expect("error!");
        let pair = if curr_id == id {
            HIGHLIGHT_PAIR
        } else {
            REGULAR_PAIR
        };
        self.label(win, text, pair);
    }

    // prints the row
    fn label(&mut self, win: &Window, text: &str, pair: u64) {
        win.mv(self.row as i32, 0);
        win.attron(COLOR_PAIR(pair));
        win.addstr(text);
        win.attroff(COLOR_PAIR(pair));
        self.row += 1;
    }

    fn end_list(&mut self) {
        self.curr_task = None;
    }
}

// - - - - -

fn list_up(curr_id: &mut usize) {
    if *curr_id > 0 {
        *curr_id -= 1;
    };
}

fn list_down(curr_id: &mut usize, len: &usize) {
    if *curr_id + 1 < *len {
        *curr_id += 1;
    };
}

fn remove_task(tasks: &mut Vec<Task>, curr_id: &mut usize) {
    if tasks.len() > 0 {
        tasks.remove(*curr_id);
        list_up(curr_id);
    }
}

fn toggle_status(tasks: &mut Vec<Task>, curr_id: &usize) {
    if tasks.len() > 0 {
        tasks[*curr_id].toggle_status()
    }
}

// - - - - -

fn main() {
    let mut tasks: Vec<Task> = vec![
        Task::new("Take a shower"),
        Task::new("Eat lunch"),
        Task::new("Code all day"),
        Task::new("get ready for today"),
        Task::new("I have a test in week and I havn't studied! shit!"),
        Task::new("study for exam"),
    ];

    {
        let args: Vec<String> = std::env::args().collect();
        let options = vec!["-a [string]: add a new task", "-h: help"];

        let help_exit = || {
            println!("\n[Options Available]:");
            for i in options {
                println!("  {}", i);
            }
            exit(0);
        };

        if args.len() > 1 {
            match args[1].as_str() {
                "-a" => {
                    if args.len() > 2 && !args[2].is_empty() {
                        tasks.push(Task::new(&args[2]))
                    } else {
                        help_exit()
                    }
                }
                "-h" | _ => help_exit(),
            }
        }
    }

    let win = initscr();
    win.keypad(true);

    start_color();
    init_pair(REGULAR_PAIR as i16, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR as i16, COLOR_BLACK, COLOR_YELLOW);
    init_pair(TITLE_PAIR as i16, COLOR_BLACK, COLOR_WHITE);
    init_pair(INFO_PAIR as i16, COLOR_WHITE, COLOR_MAGENTA);

    curs_set(0);
    noecho();

    let mut ui = UI::default();
    let mut quit = false;
    let mut curr_id: usize = 0;

    while !quit {
        win.erase();

        ui.begin(1);
        {
            ui.label(&win, "  ALL TASKS [↑/↓ | q: exit ]  ", TITLE_PAIR);

            ui.label(&win, "\n", REGULAR_PAIR);
            ui.begin_list(curr_id);

            if tasks.len() > 0 {
                for (index, task) in tasks.iter().enumerate() {
                    ui.print_element(&win, &format!(" [{}] {} ", task.status, task.text), index);
                }
            } else {
                ui.print_element(&win, " [×] -a [string]: from args to add a new task ", 69);
                ui.print_element(&win, " [×]  t | Enter: toggle task status", 69);
                ui.print_element(&win, " [×]  q: exit | d: delete task", 69);
            }

            ui.label(&win, &format!("\n Tasks: {}  ", tasks.len()), INFO_PAIR);
            ui.end_list();
        }
        win.refresh();

        match win.getch() {
            Some(Input::Character('q') | Input::KeyExit) => quit = true,
            Some(Input::Character('d')) => remove_task(&mut tasks, &mut curr_id),
            Some(Input::Character('k') | Input::KeyUp) => list_up(&mut curr_id),
            Some(Input::Character('j') | Input::KeyDown) => list_down(&mut curr_id, &tasks.len()),
            Some(Input::Character('t' | '\n')) => toggle_status(&mut tasks, &curr_id),
            None | _ => (),
        }
    }
    endwin();
}
