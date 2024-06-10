mod task;
mod ui;

use task::Task;
use ui::UI;

use pancurses::{
    curs_set, endwin, init_pair, initscr, newwin, noecho, resize_term, start_color, Input, Window,
    COLOR_BLACK, COLOR_RED, COLOR_WHITE, COLOR_YELLOW,
};
use std::fs::{self, File};
use std::process::exit;

const FILE_PATH: &str = "rtasks.json";

fn write_tasks_to_file(path: &str, tasks: &Vec<Task>) {
    let contents: String = serde_json::to_string_pretty(tasks).expect("serde to string failed!");
    fs::write(path, contents).expect("Couldn't write!");
}

fn read_tasks_from_file(path: &str) -> Result<Vec<Task>, serde_json::Error> {
    let file: File = fs::File::open(path).expect("Couldn't open file");
    serde_json::from_reader::<File, Vec<Task>>(file)
}

fn list_up(curr_id: &mut usize) {
    if *curr_id > 0 {
        *curr_id -= 1
    };
}

fn list_down(curr_id: &mut usize, len: &usize) {
    if *curr_id + 1 < *len {
        *curr_id += 1
    };
}

fn add_task(tasks: &mut Vec<Task>) {
    let border_window = newwin(4, 62, 10, 10);
    border_window.draw_box(0, 0);
    border_window.refresh();

    let window = border_window.derwin(2, 60, 1, 1).unwrap();
    window.color_set(ui::UNIQUE_PAIR as i16);

    curs_set(1);

    let mut input = String::new();

    loop {
        match window.getch() {
            Some(Input::Character('\n')) => {
                if input.len() > 0 {
                    tasks.push(Task::new(&input))
                }
                curs_set(0);
                break;
            }
            Some(Input::Character(c)) => {
                if c == '\u{8}' || c == '\u{7f}' {
                    if !input.is_empty() {
                        input.pop();

                        let (y, x) = window.get_cur_yx();
                        if x > 0 {
                            window.mv(y, x - 1); // move back
                            window.delch(); // delete current char
                        }
                    }
                } else {
                    window.color_set(ui::REGULAR_PAIR as i16);
                    input.push(c);
                    window.addch(c);
                }
            }
            _ => (),
        }
    }
}

fn remove_task(tasks: &mut Vec<Task>, curr_id: &mut usize) {
    if tasks.len() > 0 {
        tasks.remove(*curr_id);
        list_up(curr_id); // select the previous task
    }
}

fn toggle_status(tasks: &mut Vec<Task>, curr_id: &usize) {
    if tasks.len() > 0 {
        tasks[*curr_id].toggle_status()
    }
}

fn handle_args(tasks: &mut Vec<Task>) {
    let args: Vec<String> = std::env::args().collect();
    let options: Vec<&str> = vec!["-a [string]: add a new task from args", "-h: help"];

    let help_exit = || {
        println!("\n[Options Available]:");
        for i in options {
            println!("  {}", i)
        }
        exit(0);
    };

    if args.len() > 1 {
        match args[1].as_str() {
            "-a" => {
                if args.len() > 2 && !args[2].is_empty() {
                    tasks.push(Task::new(&args[2]));
                    write_tasks_to_file(&FILE_PATH.to_owned(), tasks);
                    println!("Added task successfully!");
                    exit(0);
                } else {
                    help_exit()
                }
            }
            "-h" | _ => help_exit(),
        }
    }
}

fn create_window() -> Window {
    let win: Window = initscr();
    win.keypad(true);

    start_color();
    init_pair(ui::REGULAR_PAIR as i16, COLOR_WHITE, COLOR_BLACK);
    init_pair(ui::HIGHLIGHT_PAIR as i16, COLOR_BLACK, COLOR_YELLOW);
    init_pair(ui::TITLE_PAIR as i16, COLOR_BLACK, COLOR_WHITE);
    init_pair(ui::INFO_PAIR as i16, COLOR_WHITE, COLOR_RED);
    init_pair(ui::UNIQUE_PAIR as i16, COLOR_YELLOW, COLOR_BLACK);

    curs_set(0);
    noecho();

    win
}

fn draw_ui(win: &Window, tasks: &mut Vec<Task>, curr_id: usize) {
    let mut ui: UI = UI::default();

    win.erase();
    ui.begin(0);

    if tasks.len() > 0 {
        ui.label(
            &win,
            &format!(" [↑/↓] ALL TASKS: {} ", tasks.len()),
            ui::TITLE_PAIR,
        );
    }

    ui.label(&win, "\n", ui::REGULAR_PAIR);
    ui.begin_list(curr_id);

    if tasks.len() > 0 {
        for (index, task) in tasks.iter().enumerate() {
            ui.print_element(&win, &format!(" [{}] {} ", task.status, task.text), index);
        }
    } else {
        ui.label(&win, " <-- CONTROLS --> ", ui::TITLE_PAIR);
        ui.label(&win, "\n", ui::REGULAR_PAIR);
        ui.label(&win, " <shift + a> : Add a new task", ui::UNIQUE_PAIR);
        ui.label(&win, " <shift + t> : Toggle task status", ui::UNIQUE_PAIR);
        ui.label(&win, " <shift + d> : Delete Task", ui::UNIQUE_PAIR);
        ui.label(&win, " <shift + q> : Save & Exit", ui::UNIQUE_PAIR);
    }

    ui.end_list();
}

fn main() {
    match std::fs::File::open(FILE_PATH) {
        Ok(file) => file,
        Err(_) => std::fs::File::create(FILE_PATH).unwrap(),
    };

    let mut tasks: Vec<Task> = read_tasks_from_file(FILE_PATH).unwrap_or(Vec::new());
    handle_args(&mut tasks);

    let win: Window = create_window();
    let mut curr_id: usize = 0;
    let mut quit: bool = false;

    while !quit {
        draw_ui(&win, &mut tasks, curr_id);

        match win.getch() {
            // tried my best to resize the window but it's not that well
            Some(Input::KeyResize | Input::Character('r')) => {
                win.clear();
                resize_term(0, 0);
                win.refresh();
            }
            Some(Input::Character('A')) => add_task(&mut tasks),
            Some(Input::Character('D')) => remove_task(&mut tasks, &mut curr_id),
            Some(Input::Character('T' | '\n')) => toggle_status(&mut tasks, &curr_id),
            Some(Input::Character('k') | Input::KeyUp) => list_up(&mut curr_id),
            Some(Input::Character('j') | Input::KeyDown) => list_down(&mut curr_id, &tasks.len()),
            Some(Input::Character('S')) => write_tasks_to_file(FILE_PATH, &tasks),
            Some(Input::Character('Q') | Input::KeyExit) => {
                write_tasks_to_file(FILE_PATH, &tasks);
                quit = true;
            }
            None | _ => (),
        }
    }
    endwin();
}
