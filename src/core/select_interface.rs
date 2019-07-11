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

use std::fmt::Write as _;
use std::io;
use std::io::Write as _;
use std::net::SocketAddr;

use pnet::datalink::NetworkInterface;
use termion::{color, style};

/// Displays an interactive menu of network interfaces to a user. Returns a
/// selected address of a network interface.
pub fn select_interface() -> SocketAddr {
    print_interfaces(&pnet::datalink::interfaces());

    let mut stdout = io::stdout();

    print!(
        "Enter the source address for all future sockets (<IP>:<PORT>) {yellow}>>>#{reset}",
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
            Ok(res) => return res,
            Err(_) => {
                print!(
                    "Failed to parse the socket address. Try again {yellow}>>>#{reset}",
                    yellow = color::Fg(color::Yellow),
                    reset = color::Fg(color::Reset),
                );
                stdout.flush().unwrap();
            }
        }
    }
}

fn print_interfaces(interfaces: &[NetworkInterface]) {
    for interface in interfaces {
        let mut output = String::new();

        write!(
            &mut output,
            "found a network interface:\n        Name: \
             {italic}{cyan}{name}{reset_color}{reset_style}\n        ",
            name = interface.name,
            cyan = color::Fg(color::Cyan),
            reset_color = color::Fg(color::Reset),
            italic = style::Italic,
            reset_style = style::Reset,
        )
        .unwrap();

        if let Some(mac) = interface.mac {
            write!(
                &mut output,
                "MAC address: {cyan}{address}{reset_color}\n        ",
                address = mac,
                cyan = color::Fg(color::Cyan),
                reset_color = color::Fg(color::Reset),
            )
            .unwrap();
        }

        if interface.ips.is_empty() {
            write!(
                &mut output,
                "This interface has no IP addresses and netmasks\n        "
            )
            .unwrap();
        } else {
            write!(&mut output, "Addresses:").unwrap();

            for addresses in &interface.ips {
                write!(&mut output, "\n                ").unwrap();

                write!(
                    &mut output,
                    "IP: {cyan}{ip}{reset_color}",
                    ip = addresses.ip(),
                    cyan = color::Fg(color::Cyan),
                    reset_color = color::Fg(color::Reset),
                )
                .unwrap();

                write!(&mut output, "\n                ").unwrap();

                write!(
                    &mut output,
                    "Netmask: {cyan}{netmask}{reset_color}",
                    netmask = addresses.mask(),
                    cyan = color::Fg(color::Cyan),
                    reset_color = color::Fg(color::Reset),
                )
                .unwrap();

                write!(&mut output, "\n                ").unwrap();

                write!(
                    &mut output,
                    "Broadcast: {cyan}{broadcast}{reset_color}\n",
                    broadcast = addresses.broadcast(),
                    cyan = color::Fg(color::Cyan),
                    reset_color = color::Fg(color::Reset),
                )
                .unwrap();
            }
        }

        info!("{}", output);
    }
}
