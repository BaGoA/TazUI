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

use taz;

use super::ui::UI;

/// Taz calculator application
pub struct Application<UIApp>
where
    UIApp: UI,
{
    ui: UIApp,
    history: Vec<String>,
}

impl<UIApp: UI> Application<UIApp> {
    /// Create a application
    pub fn new() -> Self {
        return Application {
            ui: UIApp::init(),
            history: Vec::with_capacity(5),
        };
    }

    /// Run the application
    pub fn run(&mut self) {
        let start_expression: String = String::from(">>> ");

        loop {
            self.ui.display_string(&start_expression);

            let expression: String = self.ui.get_expression(&self.history);

            if expression == String::from("quit") {
                break;
            }

            match taz::evaluate(&expression) {
                Ok(result) => self.ui.display_value(result),
                Err(message) => self.ui.display_string(&message),
            }

            self.ui.display_string(&String::from('\n'));

            self.history.push(expression);
        }

        self.ui.end();
    }
}
