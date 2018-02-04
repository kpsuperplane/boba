extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho, Window};


fn render(window: &Window, lines: &Vec<String>) {
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
  window.getch();

  let mut lines: Vec<String> = Vec::new();

  loop {
    let c = window.getch().unwrap();
    if c == Input::KeyBackspace {
      let last_line = lines.len();
      if last_line > 0 {
        if lines[last_line - 1].is_empty() {
          lines.pop();
        } else {
          lines[last_line - 1].pop();
        }
        render(&window, &lines);
      }
    } else if c == Input::KeyEnter {
      lines.push(String::new());
      render(&window, &lines);
    } else if c == Input::KeyDC {
      break;
    } else {
      match c {
        Input::Character(i) => {
          let mut last_line = lines.len();
          if last_line == 0 {
            lines.push(String::new());
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