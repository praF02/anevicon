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
use std::io::IoSlice;
use std::num::NonZeroUsize;
use std::os::raw::c_int;

use sendmmsg::sendmmsg;

use crate::core::summary::{SummaryPortion, TestSummary};

mod sendmmsg;

/// A type alias that represents a portion to be sent. The first item is a
/// number of bytes sent, and the second item is a packet to be sent.
pub type Packet<'a> = (usize, IoSlice<'a>);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SupplyResult {
    Flushed,
    NotFlushed,
}

/// A structure representing a raw IPv4/IPv6 socket with a buffer. The buffer is
/// described below, see the `buffer` field.
pub struct UdpSender<'a> {
    fd: c_int,

    /// The buffer capacity equals to a number of packets transmitted per a
    /// system call (`--packets-per-syscall`). When this buffer is full, then it
    /// will be flushed to an endpoint using `libc::sendmmsg`.
    buffer: Vec<Packet<'a>>,
}

impl<'a> UdpSender<'a> {
    /// Creates a socket that allows us to transmit raw IPv4/IPv6 packets
    /// (IPv4/IPv6 header + user's data).
    ///
    /// # Panics
    /// This associated function panics if your OS cannot create a raw IPv4/IPv6
    /// socket (the `socket` syscall failed). Typically this occurs because you
    /// don't have permissions, try running with `sudo`.
    pub fn new(is_ipv4: bool, capacity: NonZeroUsize) -> Self {
        UdpSender {
            fd: {
                let fd = unsafe {
                    libc::socket(
                        if is_ipv4 {
                            libc::AF_INET
                        } else {
                            libc::AF_INET6
                        },
                        libc::SOCK_RAW,
                        libc::IPPROTO_RAW,
                    )
                };

                if fd == -1 {
                    panic!(
                        "Failed to create a raw socket >>> {}",
                        io::Error::last_os_error()
                    );
                } else {
                    fd
                }
            },

            buffer: {
                let mut packets = Vec::new();
                packets.reserve_exact(capacity.get());
                packets
            },
        }
    }

    pub fn supply(
        &mut self,
        summary: &mut TestSummary,
        packet: Packet<'a>,
    ) -> io::Result<SupplyResult> {
        let res = if self.buffer.len() == self.buffer.capacity() {
            let packets_sent = sendmmsg(self.fd, self.buffer.as_mut_slice())?;
            summary.update(self.summary_portion(packets_sent));
            self.buffer.clear();
            SupplyResult::Flushed
        } else {
            SupplyResult::NotFlushed
        };

        self.buffer.push(packet);
        Ok(res)
    }

    pub fn complete(&mut self, summary: &mut TestSummary) -> io::Result<()> {
        if !self.buffer.is_empty() {
            let packets_sent = sendmmsg(self.fd, self.buffer.as_mut_slice())?;
            summary.update(self.summary_portion(packets_sent));
            self.buffer.clear();
        }

        Ok(())
    }

    /// Constructs a `SummaryPortion` instance from `self.buffer` and the
    /// `packets_sent` parameter.
    fn summary_portion(&self, packets_sent: usize) -> SummaryPortion {
        let (mut bytes_expected_total, mut bytes_sent_total) = (0, 0);

        for (bytes_sent, slice) in &self.buffer {
            bytes_expected_total += slice.len();
            bytes_sent_total += *bytes_sent;
        }

        SummaryPortion::new(
            bytes_expected_total,
            bytes_sent_total,
            self.buffer.len(),
            packets_sent,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::io::IoSlice;
    use std::ops::Deref;

    use crate::core::summary::TestSummary;

    use super::*;

    #[test]
    fn constructs_buffer() {
        let buffer = UdpSender::new(true, NonZeroUsize::new(354).unwrap());

        assert_eq!(buffer.buffer.capacity(), 354);
        assert_eq!(buffer.buffer.len(), 0);
    }

    #[test]
    fn test_packets_buffer() {
        let mut summary = TestSummary::default();
        let mut buffer = UdpSender::new(true, NonZeroUsize::new(4).unwrap());

        let check = |buffer: &UdpSender| {
            assert_eq!(buffer.buffer.capacity(), 4);
            assert_eq!(buffer.buffer.last().unwrap().1.deref(), b"Our packet");
        };

        let mut supply = |buffer: &mut UdpSender| {
            buffer
                .supply(&mut summary, (0, IoSlice::new(b"Our packet")))
                .expect("buffer.supply() failed");
        };

        supply(&mut buffer);
        assert_eq!(buffer.buffer.len(), 1);
        check(&buffer);

        supply(&mut buffer);
        assert_eq!(buffer.buffer.len(), 2);
        check(&buffer);

        supply(&mut buffer);
        assert_eq!(buffer.buffer.len(), 3);
        check(&buffer);

        supply(&mut buffer);
        assert_eq!(buffer.buffer.len(), 4);
        check(&buffer);

        // At this moment our buffer must flush itself
        supply(&mut buffer);
        assert_eq!(buffer.buffer.len(), 1);
        check(&buffer);

        supply(&mut buffer);
        assert_eq!(buffer.buffer.len(), 2);
        check(&buffer);

        buffer
            .complete(&mut summary)
            .expect("buffer.complete() failed");
        assert_eq!(buffer.buffer.len(), 0);
        assert_eq!(buffer.buffer.capacity(), 4);
    }
}
