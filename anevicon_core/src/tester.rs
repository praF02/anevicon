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

//! The test abstractions to easily describe and execute your own tests.

use std::io::{self, IoVec};
use std::net::UdpSocket;

use sendmmsg::SendMMsg;

use super::options::SendOptions;
use super::summary::TestSummary;
use crate::summary::SummaryPortion;

/// A tester with which you are able to send packets to a server multiple times.
#[derive(Debug)]
pub struct Tester<'a, 'b> {
    socket: &'a UdpSocket,
    summary: &'b mut TestSummary,
}

impl<'a, 'b> Tester<'a, 'b> {
    /// Creates a new instance of `Tester` from the specified `socket` and
    /// `summary`.
    #[inline]
    pub fn new(socket: &'a UdpSocket, summary: &'b mut TestSummary) -> Tester<'a, 'b> {
        Tester { socket, summary }
    }

    /// Sends the specified packet once, simultaneously updating the inner
    /// `TestSummary`. It returns an associated `SummaryPortion` if an operation
    /// succeeds, otherwise, returns an I/O error.
    #[inline]
    pub fn send_one(&mut self, packet: IoVec, options: SendOptions) -> io::Result<SummaryPortion> {
        match self.socket.send(&packet) {
            Err(error) => Err(error),
            Ok(bytes) => {
                let portion = SummaryPortion::new(packet.len(), bytes, 1, 1);

                if options.update {
                    self.summary.update(portion);
                }

                Ok(portion)
            }
        }
    }

    /// Sends all the specified `portions` using one system call (that is
    /// similar to [`sendmmsg`]).
    ///
    /// `portions` is a slice consisting of a number of bytes sent of each
    /// packet (the function automatically assigns there values after a call)
    /// and `IoVec` to send.
    ///
    /// This method return an associated `SummaryPortion` instance consisting of
    /// the concatenated results from [`sendmmsg`].
    ///
    /// [`sendmmsg`]: http://man7.org/linux/man-pages/man2/sendmmsg.2.html
    #[inline]
    pub fn send_multiple(
        &mut self,
        portions: &mut [(usize, IoVec)],
        options: SendOptions,
    ) -> io::Result<SummaryPortion> {
        match self.socket.sendmmsg(portions) {
            Err(error) => Err(error),
            Ok(packets) => {
                let (mut bytes_expected_total, mut bytes_sent_total) = (0, 0);

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

                if options.update {
                    self.summary.update(portion);
                }

                Ok(portion)
            }
        }
    }

    /// Returns a reference to the inner `UdpSocket`.
    #[inline]
    pub fn socket(&self) -> &UdpSocket {
        self.socket
    }

    /// Returns an immutable reference to the inner `TestSummary`.
    #[inline]
    pub fn summary(&self) -> &TestSummary {
        self.summary
    }

    /// Returns a mutable reference to the inner `TestSummary`.
    #[inline]
    pub fn summary_mut(&mut self) -> &mut TestSummary {
        self.summary
    }
}

#[cfg(test)]
mod tests {
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

    fn messages() -> Vec<(usize, IoVec<'static>)> {
        vec![
            (0, IoVec::new(b"Generals gathered in their masses")),
            (0, IoVec::new(b"Just like witches at black masses")),
            (0, IoVec::new(b"Evil minds that plot destruction")),
            (0, IoVec::new(b"Sorcerers of death's construction")),
        ]
    }

    // Normally test that all the packets were sent correctly and the specified
    // `TestSummary` was updates as well.
    #[test]
    fn test_send_multiple() {
        let mut messages = messages();

        let mut summary = TestSummary::default();
        let mut tester = Tester::new(&UDP_SOCKET, &mut summary);

        let result = tester
            .send_multiple(messages.as_mut_slice(), SendOptions::default())
            .expect("tester.send_multiple() has failed");

        assert_eq!(result.packets_sent(), messages.len());
        assert_eq!(summary.packets_sent(), messages.len());
    }

    // Now expect that the specified `TestSummary` wasn't updated if we create an
    // appropriate `SendOptions`.
    #[test]
    fn send_multiple_does_not_update() {
        let mut messages = messages();

        let mut summary = TestSummary::default();
        let mut tester = Tester::new(&UDP_SOCKET, &mut summary);

        let result = tester
            .send_multiple(
                messages.as_mut_slice(),
                SendOptions::default().update(false),
            )
            .expect("tester.send_multiple() has failed");

        assert_eq!(result.packets_sent(), messages.len());
        assert_eq!(
            summary.packets_expected() + summary.packets_sent(),
            0,
            "The packets value isn't equal to zero"
        );
        assert_eq!(
            summary.megabytes_expected() + summary.megabytes_sent(),
            0,
            "The megabytes value isn't equal to zero"
        );
    }

    #[test]
    fn test_send_one() {
        let message = b"Generals gathered in their masses";

        let result = Tester::new(&UDP_SOCKET, &mut TestSummary::default())
            .send_one(IoVec::new(message), SendOptions::default())
            .expect("tester.send_once() has failed");

        assert_eq!(result.packets_sent(), 1);
        assert_eq!(result.bytes_sent(), message.len());
    }

    #[test]
    fn send_one_does_not_update() {
        let message = b"Generals gathered in their masses";

        let mut summary = TestSummary::default();
        let mut tester = Tester::new(&UDP_SOCKET, &mut summary);

        let result = tester
            .send_one(IoVec::new(message), SendOptions::default().update(false))
            .expect("tester.send_once() has failed");

        assert_eq!(result.bytes_sent(), message.len());
        assert_eq!(result.bytes_expected(), message.len());
        assert_eq!(
            summary.packets_expected() + summary.packets_sent(),
            0,
            "The packets value isn't equal to zero"
        );
        assert_eq!(
            summary.megabytes_expected() + summary.megabytes_sent(),
            0,
            "The megabytes value isn't equal to zero"
        );
    }
}
