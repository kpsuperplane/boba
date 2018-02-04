extern crate pancurses;

use pancurses::{initscr, endwin};

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

  // wait
  window.getch();

  // slurp
  endwin();
}