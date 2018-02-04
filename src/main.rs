extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho, Window};
use std::collections::{VecDeque};

fn render(window: &Window, lines: &VecDeque<String>) {
  window.clear(); // lol please change this later
  let len = lines.len();
  for line in 0 .. len {
    window.mvprintw(line as i32, 0, &lines[line]);
  }
}

fn main() {

  let window = initscr();

  // assume 4x4 box
  let start = window.get_max_x() / 2 - 2;
  let col = window.get_max_y() / 2 - 2;

  // draw a drink
  window.mvprintw(col - 1, start, "  / ");
  window.mvprintw(col, start, "====");
  window.mvprintw(col + 1, start, "|~~|");
  window.mvprintw(col + 2, start, "|..|");
  window.mvprintw(col + 3, start, "`--`");
  window.mv(0, 0);
  window.refresh();

  noecho();
  window.keypad(true);

  // wait
  // window.getch();

  let mut lines: VecDeque<String> = VecDeque::new();

  loop {
    let c = window.getch().unwrap();
    let (y_pos, x_pos) = window.get_cur_yx();
    if c == Input::KeyLeft {  
      if x_pos == 0 && y_pos == 0 {
        continue;
      } else if x_pos == 0 {
        window.mv(y_pos - 1, (lines[(y_pos - 1) as usize].len() - 1) as i32);
      } else {
        window.mv(y_pos, x_pos - 1);
      }
      window.refresh();
    }
    if c == Input::KeyRight {
      window.mv(window.get_cur_y(), window.get_cur_x() + 1);
      window.refresh();
    }
    if c == Input::KeyUp {
      window.mv(window.get_cur_y() - 1, window.get_cur_x());
      window.refresh();
    }
    if c == Input::KeyDown {
      window.mv(window.get_cur_y() + 1, window.get_cur_x());
      window.refresh();
    }
    if c == Input::KeyBackspace {
      let last_line = lines.len();
      if last_line > 0 {
        if lines[last_line - 1].is_empty() {
          lines.pop_back();
        } else {
          lines[last_line - 1].pop();
        }
        render(&window, &lines);
      }
    } else if c == Input::KeyEnter {
      lines.push_back(String::new());
      render(&window, &lines);
    } else if c == Input::KeyDC {
      break;
    } else {
      match c {
        Input::Character(i) => {
          let mut last_line = lines.len();
          if last_line == 0 {
            lines.push_back(String::new());
            last_line = 1;
          }
          lines[last_line - 1].push(i);
          render(&window, &lines);
        },
        _ => ()
      }
    }
  }
  // slurp
  endwin();
}
