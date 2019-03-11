/* anevicon: The most powerful UDP-based load generator, written in Rust.
 * Copyright (C) 2019  Temirkhan Myrzamadi <gymmasssorla@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * For more information see <https://github.com/Gymmasssorla/anevicon>.
 */

/*!
 * The test abstractions to easily describe and execute your own tests.
 *
 * For examples please take a look at the main documentation page.
*/

use std::io;
use std::net::UdpSocket;

use super::summary::TestSummary;

/**
 * Returns the `TestIterator` instance constructed from the specified
 * arguments. This function can be used as a main entry for your tests.
 */
pub fn execute<'a, 'b, 'c>(
    socket: &'a UdpSocket,
    packet: &'b [u8],
    summary: &'c mut TestSummary,
) -> TestIterator<'a, 'b, 'c> {
    TestIterator {
        socket,
        packet,
        summary,
    }
}

/**
 * The iterator that infinitly sends a packet using the specified
 * `UdpSocket`, simultaneously updating the `TestSummary` instance.
 */
#[derive(Debug)]
pub struct TestIterator<'a, 'b, 'c> {
    socket: &'a UdpSocket,
    packet: &'b [u8],
    summary: &'c mut TestSummary,
}

impl<'a, 'b, 'c> Iterator for TestIterator<'a, 'b, 'c> {
    type Item = io::Result<usize>;

    /**
     * Does the main work (see the `TestIterator` documentation). Note
     * that this method will never return `None`, but might return an I/O
     * error if the packet sending operation has failed.
     */
    fn next(&mut self) -> Option<Self::Item> {
        match self.socket.send(self.packet) {
            Err(error) => Some(Err(error)),
            Ok(bytes) => {
                self.summary.update(bytes, 1);
                Some(Ok(bytes))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sends_all_packets() {
        let server = UdpSocket::bind("0.0.0.0:0").expect("Cannot setup the server");
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Cannot setup the socket");
        socket
            .connect(server.local_addr().unwrap())
            .expect("Cannot connect the socket to the local server");

        let packets_required = 25;
        let mut summary = TestSummary::default();

        execute(&socket, &vec![0; 16384], &mut summary)
            .take(packets_required)
            .for_each(|result| {
                if let Err(error) = result {
                    panic!("{}", error)
                }
            });

        assert_eq!(summary.packets_sent(), packets_required);
    }
}
