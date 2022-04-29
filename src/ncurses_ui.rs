/*
   TazUI is simple terminal calculator
   Copyright (C) 2022  Bastian Gonzalez Acevedo

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

extern crate ncurses;

use super::ui::UI;

/// Taz user interface implementation with ncurses
pub struct NCursesTui {
    window: ncurses::WINDOW,
}

impl UI for NCursesTui {
    fn init() -> Self {
        return NCursesTui {
            window: ncurses::initscr(),
        };
    }

    fn get_expression(&self, history: &Vec<String>) -> String {
        ncurses::keypad(self.window, true);

        let mut expression: String = String::new();
        let mut history_iter = history.iter().rev(); // browse history from the end

        let mut pos: usize = 0;
        let mut key: i32 = ncurses::getch();

        while key != '\n' as i32 {
            match key {
                ncurses::constants::KEY_UP => {
                    if !history.is_empty() {
                        match history_iter.next() {
                            Some(last_expression) => {
                                // Delete expression already present
                                for _i in 0..expression.len() {
                                    ncurses::mv(
                                        ncurses::getcury(self.window),
                                        ncurses::getcurx(self.window) - 1,
                                    );
                                    ncurses::delch();
                                }

                                expression = last_expression.to_string();
                                pos = expression.len();

                                ncurses::addstr(expression.as_str());
                            }
                            None => (),
                        }
                    }
                }
                ncurses::constants::KEY_LEFT => {
                    if pos > 0 {
                        ncurses::mv(
                            ncurses::getcury(self.window),
                            ncurses::getcurx(self.window) - 1,
                        );
                        pos -= 1;
                    }
                }
                ncurses::constants::KEY_RIGHT => {
                    if pos < expression.len() {
                        ncurses::mv(
                            ncurses::getcury(self.window),
                            ncurses::getcurx(self.window) + 1,
                        );
                        pos += 1;
                    }
                }
                ncurses::constants::KEY_BACKSPACE => {
                    if pos > 0 {
                        ncurses::delch();
                        expression.remove(pos - 1);
                        pos -= 1;
                    } else {
                        // Backspace key move the curso to left
                        // then we move cursor to right to cancel it
                        ncurses::mv(
                            ncurses::getcury(self.window),
                            ncurses::getcurx(self.window) + 1,
                        );
                    }
                }
                _ => {
                    if pos == expression.len() {
                        expression.push(key as u8 as char);
                    } else {
                        expression.insert(pos, key as u8 as char);

                        // Rewrite right part of expression on ui
                        // to emulate the substring decay
                        let cursor_pos_x: i32 = ncurses::getcurx(self.window);

                        ncurses::addstr(&expression[pos + 1..]);
                        ncurses::mv(ncurses::getcury(self.window), cursor_pos_x);
                    }

                    pos += 1;
                }
            }

            key = ncurses::getch();
        }

        ncurses::mv(ncurses::getcury(self.window) + 1, 0);

        return expression;
    }

    fn display_string(&self, string: &String) {
        ncurses::addstr(string.as_str());
    }

    fn display_value(&self, result: f64) {
        self.display_string(&result.to_string());
    }

    fn end(&mut self) {
        ncurses::endwin();
    }
}
