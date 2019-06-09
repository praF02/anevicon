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

use std::net::UdpSocket;

/// Returns a `UdpSocket` connected to itself for testing reasons.
pub fn loopback_socket() -> UdpSocket {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("A socket error");
    socket
        .connect(socket.local_addr().unwrap())
        .expect("Cannot connect the socket to itself");
    socket
}
