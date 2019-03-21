// anevicon: The most powerful UDP-based load generator, written in Rust.
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

// Print all traces and debugging information to stderr
// Print all notifications, warnings and errors to stdout

use std::io::{stderr, stdout};

use super::config::LoggingConfig;

use colored::Colorize;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::Level;
use time::{self, strftime};

pub fn setup_logging(logging_config: &LoggingConfig) {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::Magenta)
        .trace(Color::Cyan);

    let mut dispatch = Dispatch::new()
        // Print fancy colored output to a terminal without a record date
        // and the program name
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{level}] [{time}]: {message}",
                level = colors.color(record.level()).to_string().underline(),
                time = strftime("%X", &time::now()).unwrap().magenta(),
                message = message,
            ));
        })
        // Anyway, print all user-oriented information (notifications, warnings,
        // and errors) to stdout
        .chain(
            Dispatch::new()
                .filter(move |metadata| match metadata.level() {
                    Level::Info | Level::Warn | Level::Error => true,
                    Level::Debug | Level::Trace => false,
                })
                .chain(stdout()),
        );

    // If the debug mode is on, then allow printing all debugging messages
    if logging_config.debug {
        dispatch = dispatch.chain(
            Dispatch::new()
                .filter(move |metadata| match metadata.level() {
                    Level::Info | Level::Warn | Level::Error => false,
                    Level::Debug | Level::Trace => true,
                })
                .chain(stderr()),
        )
    }

    dispatch.apply().expect("Applying the dispatch has failed");
}
