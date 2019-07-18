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

use sendmmsg::sendmmsg;

use crate::config::SocketsConfig;
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
    fd: libc::c_int,

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
    /// socket or correctly set one of the socket options.
    pub fn new(is_ipv4: bool, capacity: NonZeroUsize, config: &SocketsConfig) -> Self {
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
                    helpers::set_socket_options(fd, config);
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
            self.flush(summary)?;
            SupplyResult::Flushed
        } else {
            SupplyResult::NotFlushed
        };

        self.buffer.push(packet);
        Ok(res)
    }

    pub fn flush(&mut self, summary: &mut TestSummary) -> io::Result<()> {
        if !self.buffer.is_empty() {
            let packets_sent = sendmmsg(self.fd, self.buffer.as_mut_slice())?;

            let mut bytes_expected = 0usize;
            let mut bytes_sent = 0usize;
            for (bytes_of_packet, slice) in &self.buffer {
                bytes_expected += slice.len();
                bytes_sent += *bytes_of_packet;
            }

            *summary +=
                SummaryPortion::new(bytes_expected, bytes_sent, self.buffer.len(), packets_sent);
            self.buffer.clear();
        }

        Ok(())
    }
}

impl<'a> Drop for UdpSender<'a> {
    fn drop(&mut self) {
        nix::unistd::close(self.fd).expect("Failed to drop UdpSender");
    }
}

mod helpers {
    use nix::sys::socket::setsockopt;
    use nix::sys::socket::sockopt::{Broadcast, SendTimeout};
    use nix::sys::time::TimeVal;

    use super::*;

    pub fn set_socket_options(fd: libc::c_int, config: &SocketsConfig) {
        // Set the SO_BROADCAST option
        if let Err(err) = setsockopt(fd, Broadcast, &config.broadcast) {
            panic!("Failed to set the SO_BROADCAST option >>> {}", err);
        }

        // Set the SO_SNDTIMEO option
        let send_timeout = TimeVal::from(libc::timeval {
            tv_sec: config.send_timeout.as_secs() as libc::time_t,
            tv_usec: i64::from(config.send_timeout.subsec_micros()),
        });
        if let Err(err) = setsockopt(fd, SendTimeout, &send_timeout) {
            panic!("Failed to set the SO_SNDTIMEO option >>> {}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::IoSlice;
    use std::net::UdpSocket;
    use std::ops::Deref;
    use std::os::unix::io::AsRawFd;
    use std::time::Duration;

    use nix::sys::socket::getsockopt;
    use nix::sys::socket::sockopt::{Broadcast, SendTimeout};
    use nix::sys::time::{suseconds_t, time_t};

    use lazy_static::lazy_static;

    use crate::core::summary::TestSummary;
    use crate::core::udp_sender::helpers::set_socket_options;

    use super::*;

    lazy_static! {
        static ref DEFAULT_SOCKETS_CONFIG: SocketsConfig = SocketsConfig {
            broadcast: false,
            send_timeout: Duration::from_secs(3),
        };
    }

    #[test]
    fn constructs_buffer() {
        let buffer = UdpSender::new(
            true,
            NonZeroUsize::new(354).unwrap(),
            &DEFAULT_SOCKETS_CONFIG,
        );

        assert_eq!(buffer.buffer.capacity(), 354);
        assert_eq!(buffer.buffer.len(), 0);
    }

    #[test]
    fn test_packets_buffer() {
        let mut summary = TestSummary::default();
        let mut buffer =
            UdpSender::new(true, NonZeroUsize::new(4).unwrap(), &DEFAULT_SOCKETS_CONFIG);

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
            .flush(&mut summary)
            .expect("buffer.complete() failed");
        assert_eq!(buffer.buffer.len(), 0);
        assert_eq!(buffer.buffer.capacity(), 4);
    }

    #[test]
    fn test_set_socket_options() {
        let socket = UdpSocket::bind("127.0.0.1:0").expect("Failed to bind UdpSocket");

        let check = |config: &SocketsConfig| {
            assert_eq!(
                socket.broadcast().expect("Failed to get SO_BROADCAST"),
                config.broadcast
            );

            // We don't check SO_SNDTIMEO because a Linux kernel can assign a quite
            // different timeout than we gave.
        };

        // The first check
        let config = SocketsConfig {
            broadcast: true,
            send_timeout: Duration::from_millis(5174),
        };
        set_socket_options(socket.as_raw_fd(), &config);
        check(&config);

        // The second check
        let config = SocketsConfig {
            broadcast: false,
            send_timeout: Duration::from_millis(7183),
        };
        set_socket_options(socket.as_raw_fd(), &config);
        check(&config);
    }
}
