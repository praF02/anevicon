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



use std::io;

use super::config::LoggingConfig;

use colored::Colorize as _;
use fern::colors::{Color, ColoredLevelConfig};
use fern::Dispatch;
use log::{Level, LevelFilter};
use time::{self};

pub fn setup_logging(logging_config: &LoggingConfig) {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::Magenta)
        .trace(Color::Cyan);
    let date_time_format = logging_config.date_time_format.clone();

    let mut dispatch = Dispatch::new()
        // Print fancy colored output to a terminal without a record date
        // and the program name
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{level}] [{time}]: {message}",
                level = colors.color(record.level()).to_string().underline(),
                time = time::strftime(&date_time_format, &time::now())
                    // Now we can unwrap the result because we know that the specified time format
                    // is correct
                    .unwrap()
                    .magenta(),
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
                .chain(io::stdout()),
        )
        .level(associated_level(logging_config.verbosity));

    // If the debug mode is on, then allow printing all debugging messages and
    // traces
    if logging_config.verbosity >= 4 {
        dispatch = dispatch.chain(
            Dispatch::new()
                .filter(move |metadata| match metadata.level() {
                    Level::Info | Level::Warn | Level::Error => false,
                    Level::Debug | Level::Trace => true,
                })
                .chain(io::stderr()),
        )
    }

    dispatch.apply().expect("Applying the dispatch has failed");
}

fn associated_level(verbosity: i32) -> LevelFilter {
    match verbosity {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        5 => LevelFilter::Trace,
        _ => panic!("No such verbosity level in existence"),
    }
}
