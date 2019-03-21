// anevicon: The most powerful UDP-based load generator, written in Rust.
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

//! The test abstractions to easily describe and execute your own tests.

use std::io;
use std::net::UdpSocket;

use super::summary::TestSummary;

/// Sends a packet using the specified `UdpSocket`, simultaneously updating
/// the `TestSummary`. It returns a bytes sent if an operation succeeds,
/// otherwise, returns an I/O error.
pub fn send(socket: &UdpSocket, packet: &[u8], summary: &mut TestSummary) -> io::Result<usize> {
    match socket.send(packet) {
        Err(error) => Err(error),
        Ok(bytes) => {
            summary.update(bytes, 1);
            Ok(bytes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sends_all_packets() {
        let server = UdpSocket::bind("0.0.0.0:0").expect("A server error");
        let socket = UdpSocket::bind("0.0.0.0:0").expect("A client error");
        socket
            .connect(server.local_addr().unwrap())
            .expect("Cannot connect the socket to the local server");

        let packet = vec![0; 1024];
        let packets_required = 25;

        let mut summary = TestSummary::default();

        for _ in 0..packets_required {
            if let Err(error) = send(&socket, &packet, &mut summary) {
                panic!("{}", error)
            }
        }

        assert_eq!(summary.packets_sent(), packets_required);
    }
}
