// anevicon: A library for building a UDP-based load generators for Rust.
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
// For more information see <https://github.com/Gymmasssorla/anevicon/tree/master/anevicon_core>.

//! The test abstractions to easily describe and execute your own tests.

use std::convert::TryInto;
use std::io;
use std::os::raw::c_void;
use std::os::unix::prelude::RawFd;

use crate::sendmmsg::{sendmmsg, Portion};
use crate::summary::SummaryPortion;

use super::summary::TestSummary;

/// A tester with which you are able to send packets to a server multiple times.
#[derive(Debug)]
pub struct Tester<'a> {
    pub socket: RawFd,
    pub summary: &'a mut TestSummary,
}

impl<'a> Tester<'a> {
    /// Creates a new instance of `Tester` from the specified `socket` and
    /// `summary`. `socket` is defined as a file descriptor for more
    /// flexibility.
    ///
    /// A specified socket **MUST BE** connected before you supply it to this
    /// function. If not, some methods will return an error because no IP
    /// address was specified.
    #[inline]
    pub fn new(socket: RawFd, summary: &'a mut TestSummary) -> Tester<'a> {
        Tester { socket, summary }
    }

    /// Sends the specified packet once, simultaneously updating the inner
    /// `TestSummary`.
    ///
    /// # Returns
    /// It returns an associated `SummaryPortion` if an operation succeeds,
    /// otherwise, returns an I/O error.
    pub fn send_one(&mut self, packet: &[u8]) -> io::Result<SummaryPortion> {
        let packet_len = packet.len();
        let packet = packet.as_ptr() as *const c_void;

        unsafe {
            match libc::send(self.socket, packet, packet_len, 0) {
                // `libc::send` returns -1 one failure and initializes `errno`, so create
                // `io::Error` as it follows
                -1 => Err(io::Error::last_os_error()),
                bytes => {
                    let portion = SummaryPortion::new(packet_len, bytes.try_into().unwrap(), 1, 1);
                    self.summary.update(portion);
                    Ok(portion)
                }
            }
        }
    }

    /// Sends all the specified `portions` using one system call (that is
    /// similar to [`sendmmsg`]).
    ///
    /// `portions` is a slice consisting of a number of bytes sent of each
    /// packet (the function automatically assigns there values after a call)
    /// and `IoSlice` to send.
    ///
    /// # Returns
    /// This method returns an associated `SummaryPortion` instance consisting
    /// of the concatenated results from [`sendmmsg`].
    ///
    /// [`sendmmsg`]: http://man7.org/linux/man-pages/man2/sendmmsg.2.html
    pub fn send_multiple(&mut self, portions: &mut [Portion]) -> io::Result<SummaryPortion> {
        match sendmmsg(self.socket, portions) {
            Err(error) => Err(error),
            Ok(packets) => {
                let mut bytes_expected_total = 0;
                let mut bytes_sent_total = 0;

                for (bytes_sent, vec) in portions.iter_mut() {
                    bytes_expected_total += vec.len();
                    bytes_sent_total += *bytes_sent;
                }

                let portion = SummaryPortion::new(
                    bytes_expected_total,
                    bytes_sent_total,
                    portions.len(),
                    packets,
                );

                self.summary.update(portion);
                Ok(portion)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::IoSlice;
    use std::net::UdpSocket;
    use std::os::unix::io::AsRawFd;

    use lazy_static::lazy_static;

    use super::*;

    lazy_static! {
        static ref UDP_SOCKET: UdpSocket = {
            let socket = UdpSocket::bind("0.0.0.0:0").expect("A socket error");
            socket
                .connect(socket.local_addr().unwrap())
                .expect("Cannot connect the socket to itself");
            socket
        };
    }

    #[test]
    fn test_send_multiple() {
        let messages = &mut [
            (0, IoSlice::new(b"Generals gathered in their masses")),
            (0, IoSlice::new(b"Just like witches at black masses")),
            (0, IoSlice::new(b"Evil minds that plot destruction")),
            (0, IoSlice::new(b"Sorcerers of death's construction")),
        ];

        let result = Tester::new(UDP_SOCKET.as_raw_fd(), &mut TestSummary::default())
            .send_multiple(messages)
            .expect("tester.send_multiple() has failed");

        assert_eq!(result.packets_sent(), messages.len());
        assert_eq!(result.packets_expected(), messages.len());
    }

    #[test]
    fn test_send_once() {
        let message = b"Generals gathered in their masses";

        let result = Tester::new(UDP_SOCKET.as_raw_fd(), &mut TestSummary::default())
            .send_one(message)
            .expect("tester.send_once() has failed");

        assert_eq!(result.packets_sent(), 1);
        assert_eq!(result.packets_expected(), 1);
    }
}
