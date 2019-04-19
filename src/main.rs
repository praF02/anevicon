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

use std::mem;

use colored::Colorize as _;
use log::{error, trace};

use config::ArgsConfig;

mod config;
mod helpers;
mod logging;
mod testing;

fn main() {
    let args_config = ArgsConfig::setup();
    title();

    logging::setup_logging(&args_config.logging_config);
    trace!("{:?}", args_config);

    let packet = match helpers::construct_packet(&args_config.packet_config) {
        Err(error) => {
            error!("Constructing the packet failed >>> {}!", error);
            std::process::exit(1);
        }
        Ok(packet) => packet,
    };

    // Expand ordinary lifetimes to 'static ones to avoid using the Arc<RwLock<T>>
    // construction
    match testing::execute_testers(unsafe { mem::transmute(&args_config) }, unsafe {
        mem::transmute(packet.as_slice())
    }) {
        Ok(handles) => {
            for handle in handles {
                handle.join().expect("A thread has panicked during .join()");
            }
        }
        Err(error) => {
            error!("Testing the server failed >>> {}!", error);
            std::process::exit(1);
        }
    }
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
        "{}\n",
        "A high-performance UDP-based load generator, written in Rust"
            .green()
            .underline()
    );
}
