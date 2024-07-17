extern crate ncurses;

const REGULER_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;
use ncurses::*;
use std::cmp::min;

struct ui {}
impl ui {
    fn begin_list(&mut self) {
        todo!()
    }

    fn end_list(&mut self) {
        todo!()
    }
}

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
    start_color();
    init_pair(REGULER_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;

    let done = Vec::<String>::new();
    let mut todo_curr: usize = 1;

    let mut todos = vec!["write todo app", "sleep", "repeat again!"];
    while !quit {
        for (index, todo) in todos.iter().enumerate() {
            let pair = {
                if todo_curr == index {
                    HIGHLIGHT_PAIR
                } else {
                    REGULER_PAIR
                }
            };
            // render in a  different style
            attron(COLOR_PAIR(pair));
            mv(index as i32, 1);
            addstr(*todo);
            attroff(COLOR_PAIR(pair));
        }
        refresh();

        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'k' => {
                if todo_curr > 0 {
                    todo_curr -= 1
                }
            }
            'j' => todo_curr = min(todo_curr + 1, todos.len() - 1),
            _ => {}
        }
    }
    endwin();
}
