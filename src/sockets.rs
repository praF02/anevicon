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
use std::io::Write;
use std::net::{SocketAddr, UdpSocket};

use colored::ColoredString;
use ifaces::Interface;

use crate::config::SocketsConfig;
use crate::helpers;

// Represents a UDP socket with its colored receiver name
pub struct AneviconSocket {
    socket: UdpSocket,
    receiver: ColoredString,
}

impl AneviconSocket {
    #[inline]
    pub fn socket(&self) -> &UdpSocket {
        &self.socket
    }

    #[inline]
    pub fn receiver(&self) -> &ColoredString {
        &self.receiver
    }
}

// Returns a vector of sockets connected to a certain receivers
pub fn init_sockets(config: &SocketsConfig) -> io::Result<Vec<AneviconSocket>> {
    let if_addr = if config.select_if {
        let if_addr = select_if();
        trace!("{:?} network interface will be used.", if_addr);
        Some(if_addr)
    } else {
        None
    };

    let mut sockets = Vec::with_capacity(config.receivers.len());
    for i in 0..config.receivers.len() {
        sockets.push(init_one_sock(config, i, if_addr)?);
    }

    Ok(sockets)
}

pub fn init_one_sock(
    config: &SocketsConfig,
    receiver: usize,
    if_addr: Option<SocketAddr>,
) -> io::Result<AneviconSocket> {
    let local_addr = if_addr.unwrap_or(config.sender);

    let socket = UdpSocket::bind(local_addr)?;
    socket.connect(config.receivers[receiver])?;
    socket.set_broadcast(config.broadcast)?;
    socket.set_write_timeout(Some(config.send_timeout))?;

    let receiver = helpers::cyan(config.receivers[receiver]);
    info!(
        "A new socket has been initialized to the {receiver} receiver.",
        receiver = receiver,
    );

    Ok(AneviconSocket { socket, receiver })
}

// Displays interactive menu of network interfaces
fn select_if() -> SocketAddr {
    let addrs = Interface::get_all().expect("Failed to get network interfaces");

    print_ifs_table(&addrs);
    println!();

    print!("Select a network interface by a number: #");
    io::stdout().flush().expect("flush() failed");

    loop {
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("read_line(...) failed");
        choice.pop(); // Delete the ending '\n' character

        let choice = match choice.parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                print!("This is not a number. Try again: #");
                io::stdout().flush().expect("flush() failed");
                continue;
            }
        };

        let addr = match addrs.get(choice) {
            Some(interface) => interface,
            None => {
                print!("The number is out of range. Try again: #");
                io::stdout().flush().expect("flush() failed");
                continue;
            }
        };

        return match addr.addr {
            Some(addr) => addr,
            None => {
                print!("The selected interface doesn't contain an address. Try again: #");
                io::stdout().flush().expect("flush() failed");
                continue;
            }
        };
    }
}

fn print_ifs_table(if_addrs: &[Interface]) {
    let mut table = table!(["Number", "Name", "Address"]);

    for i in 0..if_addrs.len() {
        table.add_row(row![
            &format!("#{}", i.to_string()),
            &if_addrs[i].name,
            &if_addrs[i]
                .addr
                .map_or_else(|| String::from("None"), |val| val.to_string()),
        ]);
    }

    table.printstd();
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_init_socket() {
        let config = SocketsConfig {
            receivers: vec![
                "45.89.52.36:5236".parse().unwrap(),
                "89.52.36.41:256".parse().unwrap(),
                "85.53.23.57:45687".parse().unwrap(),
            ],
            sender: "0.0.0.0:0".parse().unwrap(),
            select_if: None,
            send_timeout: Duration::from_secs(25),
            broadcast: true,
        };

        let socket = init_socket(&config, 1, None).expect("init_socket() has failed");
        assert_eq!(socket.local_addr().unwrap().ip().is_global(), false);
        assert_eq!(socket.write_timeout().unwrap(), Some(config.send_timeout));
        assert_eq!(socket.broadcast().unwrap(), config.broadcast);
    }
}
