mod task;
mod ui;

extern crate pancurses;
extern crate serde;
use pancurses::{
    curs_set, endwin, init_pair, initscr, noecho, start_color, Input, Window, COLOR_BLACK,
    COLOR_RED, COLOR_WHITE, COLOR_YELLOW,
};
use std::fs::{self, File};
use std::process::exit;
use task::Task;
use ui::UI;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

fn write_tasks_to_file(path: &String, tasks: &Vec<Task>) {
    fs::write(
        path,
        serde_json::to_string_pretty(tasks).expect("serde to string failed!"),
    )
    .expect("Couldn't write!");
}

fn read_tasks_from_file(path: &String) -> Result<Vec<Task>, Vec<Task>> {
    let file = fs::File::open(path).expect("Couldn't open file");
    let tasks =
        serde_json::from_reader::<File, Vec<Task>>(file).or::<Vec<Task>>(Ok(Vec::<Task>::new()));

    tasks
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

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

fn handle_args(tasks: &mut Vec<Task>) {
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
                    tasks.push(Task::new(&args[2]));
                    write_tasks_to_file(&"data.json".to_owned(), tasks);
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
    let win = initscr();
    win.keypad(true);

    start_color();
    init_pair(ui::REGULAR_PAIR as i16, COLOR_WHITE, COLOR_BLACK);
    init_pair(ui::HIGHLIGHT_PAIR as i16, COLOR_BLACK, COLOR_YELLOW);
    init_pair(ui::TITLE_PAIR as i16, COLOR_BLACK, COLOR_WHITE);
    init_pair(ui::INFO_PAIR as i16, COLOR_WHITE, COLOR_RED);

    curs_set(0);
    noecho();

    win
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

fn main() {
    let file_path = "data.json".to_owned();

    match std::fs::File::open(&file_path) {
        Ok(file) => file,
        Err(_) => std::fs::File::create(&file_path).unwrap(),
    };

    let mut tasks: Vec<Task> = read_tasks_from_file(&file_path).unwrap();
    handle_args(&mut tasks);

    let win = create_window();
    let mut ui = UI::default();

    let mut curr_id: usize = 0;
    let mut quit = false;

    while !quit {
        win.erase();
        ui.begin(1);

        {
            ui.label(&win, "  ALL TASKS [↑/↓ | q: exit ]  ", ui::TITLE_PAIR);

            ui.label(&win, "\n", ui::REGULAR_PAIR);
            ui.begin_list(curr_id);

            if tasks.len() > 0 {
                for (index, task) in tasks.iter().enumerate() {
                    ui.print_element(&win, &format!(" [{}] {} ", task.status, task.text), index);
                }
            } else {
                ui.print_element(&win, " [×] -a [string]: from args to add a new task ", 69);
                ui.print_element(&win, " [×]  t | Enter: toggle task status", 69);
                ui.print_element(&win, " [×]  q: save & exit | d: delete task", 69);
            }
            ui.label(&win, &format!("\n Tasks: {}  ", tasks.len()), ui::INFO_PAIR);
            ui.end_list();
        }

        win.refresh();

        match win.getch() {
            Some(Input::Character('q') | Input::KeyExit) => {
                quit = true;
                write_tasks_to_file(&"data.json".to_owned(), &tasks);
            }
            Some(Input::Character('d')) => remove_task(&mut tasks, &mut curr_id),
            Some(Input::Character('k') | Input::KeyUp) => list_up(&mut curr_id),
            Some(Input::Character('j') | Input::KeyDown) => list_down(&mut curr_id, &tasks.len()),
            Some(Input::Character('t' | '\n')) => toggle_status(&mut tasks, &curr_id),
            None | _ => (),
        }
    }
    endwin();
}
