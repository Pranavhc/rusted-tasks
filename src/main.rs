extern crate pancurses;
mod task;
mod ui;

use pancurses::{
    curs_set, echo, endwin, init_pair, initscr, noecho, start_color, Input, Window, COLOR_BLACK,
    COLOR_MAGENTA, COLOR_WHITE, COLOR_YELLOW,
};

use task::Task;
use ui::{HIGHLIGHT_PAIR, INFO_PAIR, REGULAR_PAIR, TITLE_PAIR, UI};

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
        // list_down(curr_id, &tasks.len());
        list_up(curr_id);
    }
}

fn toggle_status(tasks: &mut Vec<Task>, curr_id: &usize) {
    if tasks.len() > 0 {
        tasks[*curr_id].toggle_status()
    }
}

fn add_task(window: &Window, tasks: &mut Vec<Task>, text: &mut String) {
    echo();
    match window.getch() {
        Some(Input::Character(c)) => {
            if c != '\n' {
                text.push(c)
            }
        }
        _ => (),
    }
    tasks.push(Task::new(text));
    // *text = String::new();
    noecho();
}

fn main() {
    let mut tasks: Vec<Task> = vec![
        Task::new("Take a shower"),
        Task::new("Eat lunch"),
        Task::new("Code all day"),
        Task::new("get ready for today"),
        Task::new("I have a test in week and I havn't studied! shit!"),
        Task::new("study for exam"),
    ];

    let window = initscr();
    window.keypad(true);

    let mut new_str = String::new();

    start_color();
    init_pair(REGULAR_PAIR as i16, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR as i16, COLOR_BLACK, COLOR_YELLOW);
    init_pair(TITLE_PAIR as i16, COLOR_BLACK, COLOR_WHITE);
    init_pair(INFO_PAIR as i16, COLOR_WHITE, COLOR_MAGENTA);

    curs_set(0);
    noecho();

    let mut ui = UI::default();
    let mut curr_id: usize = 0;
    let mut quit = false;

    while !quit {
        window.erase();

        ui.begin(1);
        {
            ui.label(
                &window,
                "  ALL Tasks  [ q: exit | s/w : ↑/↓ ]  ",
                TITLE_PAIR,
            );

            ui.label(&window, "\n", REGULAR_PAIR);
            ui.begin_list(curr_id);

            if tasks.len() > 0 {
                for (index, task) in tasks.iter().enumerate() {
                    ui.print_element(
                        &window,
                        &format!(" [{}] {} ", task.status, task.text),
                        index,
                    );
                }
            } else {
                ui.print_element(&window, " [×] Add some tasks! ", 0);
            }

            ui.label(&window, &format!("\n Tasks: {} ", tasks.len()), INFO_PAIR);
            ui.end_list();
        }
        window.refresh();

        match window.getch() {
            Some(Input::Character('q') | Input::KeyExit) => quit = true,
            Some(Input::Character('w') | Input::KeyUp) => list_up(&mut curr_id),
            Some(Input::Character('s') | Input::KeyDown) => list_down(&mut curr_id, &tasks.len()),
            Some(Input::Character('d')) => remove_task(&mut tasks, &mut curr_id),
            Some(Input::Character('\n')) => toggle_status(&mut tasks, &curr_id),
            // Some(Input::Character('a')) => add_task(&window, &mut tasks, &mut new_str),
            None | _ => (),
        }
    }
    endwin();
    println!("new_str = {}", new_str);
}
