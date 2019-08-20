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

#[macro_use]
extern crate failure_derive;

use std::collections::HashSet;
use std::convert::TryInto;

use termion::{color, style, terminal_size};

use config::ArgsConfig;

mod config;
mod core;
mod errors_utils;
mod logging;

fn main() {
    let config = ArgsConfig::setup();
    title();

    logging::setup_logging(&config.logging_config);
    log::trace!("{:?}", config);

    if check_config(&config).is_err() {
        std::process::exit(libc::EXIT_FAILURE);
    }

    if core::run(config).is_err() {
        std::process::exit(libc::EXIT_FAILURE);
    }
}

fn check_config(config: &ArgsConfig) -> Result<(), ()> {
    let mut keys = HashSet::new();
    for next_endpoints in &config.packets_config.endpoints {
        if next_endpoints.sender().port() == 0 {
            log::warn!(
                "datagrams sent from {source_address} might be dropped by a router because of the \
                 unspecified source port!",
                source_address = next_endpoints.sender(),
            );
        }

        if keys.contains(next_endpoints) {
            log::error!(
                "all endpoints must be uniquely specified, but {sender}&{receiver} has been \
                 specified several times!",
                sender = next_endpoints.sender(),
                receiver = next_endpoints.receiver(),
            );

            return Err(());
        } else {
            keys.insert(next_endpoints);
        }
    }

    Ok(())
}

fn title() {
    let tab = " ".repeat(
        ((terminal_size().expect("Failed to get the terminal size").0 - 54) / 2)
            .try_into()
            .unwrap(),
    );

    #[rustfmt::skip]
    println!(
        "{cyan}{}{reset}",
        format!("\
{tab}+----------------------------------------------------+\n\
{tab}|                            .-.                     |
{tab}|  .-.  .  .-.   .-.  )   .-.`-'.-.  .-._..  .-.     |
{tab}| (  |   )/   )./.-'_(   /  /  (    (   )  )/   )    |
{tab}|  `-'-''/   ( (__.'  \\_/_.(__. `---'`-'  '/   (     |
{tab}|             `-                                `-   |
{tab}+----------------------------------------------------+", tab = tab),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset));

    println!(
        "                      {tab}{red}{bold}version {version}{reset_color}{reset_style}",
        version = structopt::clap::crate_version!(),
        tab = tab,
        bold = style::Bold,
        red = color::Fg(color::Red),
        reset_style = style::Reset,
        reset_color = color::Fg(color::Reset),
    );

    println!(
        "      {tab}{underline}{green}A high-performant UDP-based load \
         generator{reset_style}{reset_color}\n",
        tab = tab,
        underline = style::Underline,
        green = color::Fg(color::Green),
        reset_style = style::Reset,
        reset_color = color::Fg(color::Reset),
    );
}
