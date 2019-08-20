// anevicon: A high-performant UDP-based load generator, written in Rust.
// Copyright (C) 2019  Temirkhan Myrzamadi <gymmasssorla@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// For more information see <https://github.com/Gymmasssorla/anevicon>.

use std::fmt::Write;

/// Formats all error causes into `String` (including the error itself). Always
/// use this function to display `failure::Error`. For example:
///
/// ```
/// [ERROR] [23:21:00]: a tester exited unexpectedly!
///     Caused by: Failed to create a socket
///     Caused by: Operation not permitted (os error 1)
/// ```
pub fn display_error_causes(error: &failure::Error) -> String {
    let mut result = String::new();

    let causes = error.iter_chain().collect::<Vec<&dyn failure::Fail>>();
    for cause in causes.iter().take(causes.len() - 1) {
        writeln!(result, "    Caused by: {}", cause).unwrap();
    }
    write!(result, "    Caused by: {}", causes[causes.len() - 1]).unwrap();

    result
}
