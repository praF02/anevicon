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

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::io::Write;
use std::net::SocketAddr;

use termion::color;

/// Displays an interactive menu of network interfaces to a user. Returns a
/// selected address of a network interface.
pub fn select_interface() -> Result<SocketAddr, SelectInterfaceError> {
    info!(
        "The program will display all your network interfaces now. Then you need to enter a \
         source address for all future sockets (it can be located inside IP ranges of network \
         interfaces)."
    );

    // Recognize all network interfaces available in the current machine. If there
    // is no network interfaces, then return an error
    let interfaces = pnet::datalink::interfaces();
    if interfaces.is_empty() {
        return Err(SelectInterfaceError::NoInterfaces);
    }

    // Print all recognized network interfaces and then let a user choose one from
    // the list by its IP address
    println!();
    for interface in &interfaces {
        info!("{}", interface);
    }
    println!();

    let mut stdout = io::stdout();

    print!(
        "Enter a global source address (<IP>:<PORT>) {yellow}>>>#{reset}",
        yellow = color::Fg(color::Yellow),
        reset = color::Fg(color::Reset),
    );
    stdout.flush().unwrap();

    loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("read_line(...) failed");
        choice.pop(); // Delete the ending '\n' character

        match choice.parse::<SocketAddr>() {
            Ok(res) => return Ok(res),
            Err(_) => {
                print!(
                    "Failed to parse the source address. Try again {yellow}>>>#{reset}",
                    yellow = color::Fg(color::Yellow),
                    reset = color::Fg(color::Reset),
                );
                stdout.flush().unwrap();
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SelectInterfaceError {
    NoInterfaces,
}

impl Display for SelectInterfaceError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            SelectInterfaceError::NoInterfaces => {
                write!(fmt, "Your machine has no network interfaces")
            }
        }
    }
}

impl Error for SelectInterfaceError {}
