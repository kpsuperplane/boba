extern crate pancurses;

use pancurses::{initscr, endwin};

fn main() {
  let window = initscr();
  let start = window.get_max_x() / 2 - 2;
  let col = window.get_max_y() / 2 - 2;
  window.mvprintw(col - 1, start, "  / ");
  window.mvprintw(col, start, "====");
  window.mvprintw(col + 1, start, "|~~|");
  window.mvprintw(col + 2, start, "|..|");
  window.mvprintw(col + 3, start, "`--`");
  window.mv(0, 0);
  window.refresh();
  window.getch();
  endwin();
}