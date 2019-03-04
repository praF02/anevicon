/* anevicon: The most powerful UDP-based load generator, written in Rust.
 * Copyright (C) 2019  Temirkhan Myrzamadi <gymmasssorla@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * For more information see <https://github.com/Gymmasssorla/anevicon>.
 */

// Print all traces and debugging information to stderr
// Print all notifications, warnings and errors to stdout

use std::fmt::Arguments;
use std::fs::File;
use std::io::{self, stderr, stdout};

use super::config::LoggingConfig;

use colored::Colorize;
use fern::colors::{Color, ColoredLevelConfig};
use fern::{log_file, Dispatch};
use log::{Level, LevelFilter};
use time::{self, strftime};

pub fn raw_fatal(message: Arguments) -> ! {
    // Print a colored error message to stdout and exit with the code 1,
    // even without a correctly configured logging system
    println!(
        "[{level}] [{time}]: {message}",
        level = "ERROR".red().underline(),
        time = strftime("%X", &time::now()).unwrap().cyan(),
        message = message
    );
    std::process::exit(1);
}

pub fn setup_logging(logging_config: &LoggingConfig) -> io::Result<()> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::Magenta)
        .trace(Color::Cyan);

    if let Some(ref filename) = logging_config.output {
        Dispatch::new()
            .chain(terminal_dispatch(false, logging_config.debug, colors))
            .chain(file_dispatch(log_file(filename)?))
    } else {
        Dispatch::new().chain(terminal_dispatch(true, logging_config.debug, colors))
    }
    .apply()
    .expect("Applying the dispatch has failed");
    Ok(())
}

fn terminal_dispatch(need_stdout: bool, need_debug: bool, colors: ColoredLevelConfig) -> Dispatch {
    let mut dispatch = Dispatch::new()
        // Print fancy colored output to a terminal without a record date
        // and the program name
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{level}] [{time}]: {message}",
                level = colors.color(record.level()).to_string().underline(),
                time = strftime("%X", &time::now()).unwrap().cyan(),
                message = message,
            ));
        })
        // Anyway, print all debugging information to a terminal
        .chain(
            Dispatch::new()
                .filter(move |metadata| match metadata.level() {
                    Level::Info | Level::Warn | Level::Error => false,
                    Level::Debug | Level::Trace => true,
                })
                .chain(stderr()),
        );

    // If the debug mode is on, then allow printing all debugging messages
    if !need_debug {
        dispatch = dispatch.level(LevelFilter::Info);
    }

    // If a file IS NOT specified, print all notifications, warnings and
    // errors to stdout
    if need_stdout {
        dispatch = dispatch.chain(
            Dispatch::new()
                .filter(move |metadata| match metadata.level() {
                    Level::Info | Level::Warn | Level::Error => true,
                    Level::Debug | Level::Trace => false,
                })
                .chain(stdout()),
        );
    }

    dispatch
}

fn file_dispatch(file: File) -> Dispatch {
    Dispatch::new()
        // Include the program name and a record date, also disable colors
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[anevicon] [{level}] [{date_time}]: {message}",
                level = record.level(),
                date_time = strftime("%x %X %z", &time::now()).unwrap(),
                message = message,
            ));
        })
        // Print to the file only notifications, warnings and errors
        .filter(move |metadata| match metadata.level() {
            Level::Info | Level::Warn | Level::Error => true,
            Level::Debug | Level::Trace => false,
        })
        .chain(file)
}
