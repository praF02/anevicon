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
extern crate log;

use std::convert::TryInto;

use termion::{color, style, terminal_size};

use config::ArgsConfig;
use select_interface::select_interface;

mod config;
mod core;
mod logging;
mod select_interface;

fn main() -> Result<(), ()> {
    setup_ctrlc_handler();

    let mut config = ArgsConfig::setup();
    title();

    logging::setup_logging(&config.logging_config);
    trace!("{:?}", config);

    check_config(&config)?;
    prepare_config(&mut config)?;

    core::run(config)
}

fn check_config(config: &ArgsConfig) -> Result<(), ()> {
    // Allowing --packets-count be less than --packets-per-syscall is a logical
    // mistake, so display the error to a user
    if config.packets_per_syscall > config.exit_config.packets_count {
        error!(
            "a value of {green}--packets-count{reset} must be higher or equal to a value of \
             {green}--packets-per-syscall{reset}!",
            green = color::Fg(color::Green),
            reset = color::Fg(color::Reset)
        );

        return Err(());
    }

    // A sender and all receivers must be both specified as either IPv4 or IPv6
    // addresses, because we cannot put both IPv4 and IPv6 address in a
    // single IP packet
    let is_ipv4_sender = config.packets_config.sender.is_ipv4();

    for receiver in &config.packets_config.receivers {
        let is_ipv4_receiver = receiver.is_ipv4();
        if is_ipv4_sender != is_ipv4_receiver {
            error!(
                "a sender and all receivers must be both specified as either IPv4 or IPv6 \
                 addresses!"
            );

            return Err(());
        }
    }

    Ok(())
}

fn prepare_config(config: &mut ArgsConfig) -> Result<(), ()> {
    config.packets_config.sender = if config.select_if {
        match select_interface() {
            Err(err) => {
                error!("failed to select a network interface >>> {}!", err);
                return Err(());
            }
            Ok(interface) => interface,
        }
    } else {
        config.packets_config.sender
    };

    Ok(())
}

fn setup_ctrlc_handler() {
    ctrlc::set_handler(move || {
        info!("cancellation has been received. Exiting the program...");
        std::process::exit(0);
    })
    .expect("Error while setting the Ctrl-C handler");

    trace!("the Ctrl-C handler has been configured.");
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
