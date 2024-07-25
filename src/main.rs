extern crate ncurses;

const REGULER_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;
use ncurses::*;
use std::cmp::min;

type Id = usize;

#[derive(Default)]
struct Ui {
    list_curr: Option<Id>,
    row: usize,
    col: usize,
}

impl Ui {
    fn begin(&mut self, row: usize, col: usize) {
        self.row = row;
        self.col = col;
    }

    fn begin_list(&mut self, id: Id) {
        assert!(self.list_curr.is_none(), "Nested list are not allowed");
        self.list_curr = Some(id);
    }

    fn list_element(&mut self, label: &str, id: Id) -> bool {
        let id_curr = self
            .list_curr
            .expect("Not Allowed to create list elements outside of list");

        self.label(label, {
            if id_curr == id {
                HIGHLIGHT_PAIR
            } else {
                REGULER_PAIR
            }
        });
        return false;
    }

    fn label(&mut self, text: &str, pair: i16) {
        mv(self.row as i32, self.col as i32);
        attron(COLOR_PAIR(pair));
        addstr(text);
        attroff(COLOR_PAIR(pair));
        self.row += 1;
    }

    fn end_list(&mut self) {
        self.list_curr = None;
    }

    fn end(&mut self) {}
}

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
    start_color();
    init_pair(REGULER_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;

    let mut todo_curr: usize = 0;
    let mut done_curr: usize = 0;

    let mut todos: Vec<String> = vec![
        "write todo app".to_string(),
        "sleep".to_string(),
        "repeat again!".to_string(),
        "coding rust is horiabel!".to_string(),
    ];

    let mut dones: Vec<String> = vec![
        "started coding rust".to_string(),
        "started hate rust".to_string(),
    ];

    let mut ui = Ui::default();
    while !quit {
        erase();
        ui.begin(0, 0);
        {
            ui.label("TODO:", REGULER_PAIR);
            ui.begin_list(todo_curr);
            for (index, todo) in todos.iter().enumerate() {
                ui.list_element(&format!(" - [ ] {}", todo), index);
            }
            ui.end_list();

            ui.label("--------------------------------", REGULER_PAIR);
            ui.label("DONE:", REGULER_PAIR);
            ui.begin_list(done_curr);
            for (index, done) in dones.iter().enumerate() {
                ui.list_element(&format!("- [x] {}", &done), index + 1);
            }
            ui.end_list();
        }

        ui.end();
        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'k' => {
                if todo_curr > 0 {
                    todo_curr -= 1
                }
            }
            'j' => {
                if todo_curr + 1 < todos.len() {
                    todo_curr += 1;
                }
            }
            '\n' => {
                if (todo_curr < todos.len()) {
                    dones.push(todos.remove(todo_curr));
                }
            }
            _ => {}
        }
    }
    endwin();
}
