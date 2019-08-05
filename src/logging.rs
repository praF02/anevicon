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

//! A module containing the `setup_config` function which setups the whole
//! logging system.

use std::io;

use fern::Dispatch;
use log::{Level, LevelFilter};
use termion::{color, style};
use time;

use super::config::LoggingConfig;

/// Setups the logging system from `LoggingConfig`. Before this function, none
/// of log's macros such as `info!` will work.
pub fn setup_logging(logging_config: &LoggingConfig) {
    let dt_format = logging_config.date_time_format.clone();

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{underline}{level_color}{level}{reset_color}{reset_style}] \
                 [{magenta}{time}{reset_color}]: {message}",
                level = record.level(),
                time = time::strftime(&dt_format, &time::now()).unwrap(),
                message = message,
                level_color = associated_color(record.level()),
                magenta = color::Fg(color::Magenta),
                reset_color = color::Fg(color::Reset),
                underline = style::Underline,
                reset_style = style::Reset,
            ));
        })
        // If the debug mode is on, then allow printing all debugging messages and
        // traces
        .chain(
            Dispatch::new()
                .filter(move |metadata| match metadata.level() {
                    Level::Info | Level::Warn | Level::Error => false,
                    Level::Debug | Level::Trace => true,
                })
                .chain(io::stderr()),
        )
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
        .level(associated_level(logging_config.verbosity))
        .apply()
        .expect("Applying the fern::Dispatch has failed");
}

fn associated_color(level: Level) -> &'static str {
    match level {
        Level::Info => color::Green.fg_str(),
        Level::Warn => color::Yellow.fg_str(),
        Level::Error => color::Red.fg_str(),
        Level::Debug => color::Magenta.fg_str(),
        Level::Trace => color::Cyan.fg_str(),
    }
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
