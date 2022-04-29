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

/// Trait to define user interface functionnalities
/// that Taz application needs.
pub trait UI {
    fn init() -> Self;
    fn get_expression(&self, history: &Vec<String>) -> String;
    fn display_string(&self, string: &String);
    fn display_value(&self, result: f64);
    fn end(&mut self);
}
