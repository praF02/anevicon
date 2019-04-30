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

#![feature(ip)]
#![feature(iovec)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate prettytable;

use std::sync::Arc;
use std::thread;
use std::time::Duration;

use colored::Colorize;
use humantime::format_duration;

use config::ArgsConfig;

use crate::sockets::init_sockets;

mod config;
mod helpers;
mod logging;
mod sockets;
mod testing;

fn main() {
    let config = ArgsConfig::setup();
    title();

    logging::setup_logging(&config.logging_config);
    trace!("{:?}", config);

    let packet = helpers::construct_packet(&config.packet_config).unwrap_or_else(|err| {
        error!("Constructing the packet failed >>> {}!", err);
        std::process::exit(1);
    });

    let sockets = init_sockets(&config.tester_config.sockets_config).unwrap_or_else(|err| {
        error!("Socket initialization failed >>> {}!", err);
        std::process::exit(1);
    });

    wait(config.wait);

    match testing::execute_testers(Arc::new(config.tester_config), Arc::new(packet), sockets) {
        Ok(handles) => {
            for handle in handles {
                handle.join().expect("A thread has panicked during .join()");
            }
        }
        Err(err) => {
            error!("Testing the server failed >>> {}!", err);
            std::process::exit(1);
        }
    }
}

fn wait(duration: Duration) {
    warn!(
        "Waiting {time} and then starting to execute the tests...",
        time = helpers::cyan(format_duration(duration))
    );
    thread::sleep(duration);
}

fn title() {
    println!(
        "         {}",
        r"                        _                 ".cyan()
    );
    println!(
        "         {}",
        r"  __ _ _ __   _____   _(_) ___ ___  _ __  ".cyan()
    );
    println!(
        "         {}",
        r" / _` | '_ \ / _ \ \ / / |/ __/ _ \| '_ \ ".cyan()
    );
    println!(
        "         {}",
        r"| (_| | | | |  __/\ V /| | (_| (_) | | | |".cyan()
    );
    println!(
        "         {}",
        r" \__,_|_| |_|\___| \_/ |_|\___\___/|_| |_|".cyan()
    );
    println!(
        "                       {}",
        format!("version {}", structopt::clap::crate_version!())
            .red()
            .bold()
    );
    println!(
        "{}\n",
        "A high-performant UDP-based load generator, written in Rust"
            .green()
            .underline()
    );
}
