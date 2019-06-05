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

//! A module containing abstractions for socket initialization and future usage.

use std::io::{self, Write};
use std::net::{SocketAddr, UdpSocket};

use ifaces::Interface;
use termion::{color, style};

use crate::config::SocketsConfig;

/// Represents a UDP socket with its colored receiver name.
pub struct AneviconSocket {
    socket: UdpSocket,
    receiver: String,
}

impl AneviconSocket {
    #[inline]
    pub fn socket(&self) -> &UdpSocket {
        &self.socket
    }

    #[inline]
    pub fn receiver(&self) -> &str {
        &self.receiver
    }
}

/// Returns a vector of sockets connected to certain receivers or `io::Error`
/// because initializations might fail.
pub fn init_sockets(config: &SocketsConfig) -> io::Result<Vec<AneviconSocket>> {
    let if_addr = if config.select_if {
        let if_addr = select_if();
        debug!(
            "bind all future sockets to the {cyan}{}{reset} network interface.",
            if_addr,
            cyan = color::Fg(color::Cyan),
            reset = color::Fg(color::Reset),
        );
        Some(if_addr)
    } else {
        None
    };

    let mut sockets = Vec::with_capacity(config.receivers.len());
    for i in 0..config.receivers.len() {
        sockets.push(init_one_socket(config, i, if_addr)?);
    }

    Ok(sockets)
}

/// Initializes **ONLY ONE** socket connected to `config.receivers[receiver]`.
/// If `if_addr` is any, it will bind a socket to it.
fn init_one_socket(
    config: &SocketsConfig,
    receiver: usize,
    if_addr: Option<SocketAddr>,
) -> io::Result<AneviconSocket> {
    let local_addr = if_addr.unwrap_or(config.sender);

    let socket = UdpSocket::bind(local_addr)?;
    socket.connect(config.receivers[receiver])?;
    socket.set_broadcast(config.broadcast)?;
    socket.set_write_timeout(Some(config.send_timeout))?;

    if let Some(val) = config.ip_ttl {
        socket.set_ttl(val)?;
    }

    let receiver = config.receivers[receiver].to_string();
    debug!(
        "a new socket has been initialized to {cyan}{receiver}{reset}.",
        receiver = receiver,
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );

    Ok(AneviconSocket { socket, receiver })
}

/// Displays an interactive menu of network interfaces to a user.
fn select_if() -> SocketAddr {
    let addrs = Interface::get_all().expect("Failed to get network interfaces");
    print_ifs(&addrs);

    let mut stdout = io::stdout();

    print!(
        "Select a network interface {yellow}>>>#{reset}",
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

        let choice = match choice.parse::<usize>() {
            Ok(num) => num,
            Err(_) => {
                print!(
                    "This is not a number {yellow}>>>#{reset}",
                    yellow = color::Fg(color::Yellow),
                    reset = color::Fg(color::Reset),
                );
                stdout.flush().unwrap();
                continue;
            }
        };

        let addr = match addrs.get(choice) {
            Some(interface) => interface,
            None => {
                print!(
                    "The number is out of range {yellow}>>>#{reset}",
                    yellow = color::Fg(color::Yellow),
                    reset = color::Fg(color::Reset),
                );
                stdout.flush().unwrap();
                continue;
            }
        };

        return match addr.addr {
            Some(addr) => addr,
            None => {
                print!(
                    "Cannot get an address {yellow}>>>#{reset}",
                    yellow = color::Fg(color::Yellow),
                    reset = color::Fg(color::Reset),
                );
                stdout.flush().unwrap();
                continue;
            }
        };
    }
}

/// Prints all the given network interfaces to a user.
fn print_ifs(if_addrs: &[Interface]) {
    for i in 0..if_addrs.len() {
        info!(
            "found a network interface {cyan}#{number}{reset_color}:\n\tName:    \
             {italic}{cyan}{name}{reset_color}{reset_style}\n\tAddress: \
             {cyan}{ip}{reset_color}\n\tNetmask: {cyan}{mask}{reset_color}",
            number = i,
            name = if_addrs[i].name,
            ip = if_addrs[i]
                .addr
                .map_or_else(|| String::from("none"), |val| val.to_string(),),
            mask = if_addrs[i]
                .mask
                .map_or_else(|| String::from("none"), |val| val.to_string(),),
            cyan = color::Fg(color::Cyan),
            reset_color = color::Fg(color::Reset),
            italic = style::Italic,
            reset_style = style::Reset,
        );
    }

    println!();
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
            select_if: false,
            send_timeout: Duration::from_secs(25),
            ip_ttl: Some(13),
            broadcast: true,
        };

        let res = init_one_socket(&config, 1, None).expect("init_one_socket() has failed");
        let socket = res.socket;

        assert_eq!(socket.local_addr().unwrap().ip().is_global(), false);
        assert_eq!(socket.write_timeout().unwrap(), Some(config.send_timeout));
        assert_eq!(socket.broadcast().unwrap(), config.broadcast);
        assert_eq!(socket.ttl().unwrap(), config.ip_ttl.unwrap());

        assert_eq!(res.receiver, config.receivers[1].to_string());
    }
}
