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
use std::net::SocketAddr;
use std::num::NonZeroUsize;

use nix::sys::socket::MsgFlags;
use nix::sys::socket::{InetAddr, IpAddr, SockAddr};

use sendmmsg::sendmmsg;

use crate::config::SocketsConfig;
use crate::core::statistics::{SummaryPortion, TestSummary};

mod sendmmsg;

/// A type alias that represents a portion to be sent. `transmitted` is a
/// number of bytes sent, and `slice` is a packet to be sent.
#[derive(Debug)]
pub struct DataPortion<'a> {
    pub transmitted: usize,
    pub slice: IoSlice<'a>,
}

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
    buffer: Vec<DataPortion<'a>>,
}

impl<'a> UdpSender<'a> {
    /// Creates a socket that allows us to transmit raw IPv4/IPv6 packets
    /// (IPv4/IPv6 header + user's data).
    ///
    /// # Panics
    /// This associated function panics if your OS cannot create a raw IPv4/IPv6
    /// socket or correctly set one of the socket options.
    pub fn new(
        dest: &SocketAddr,
        capacity: NonZeroUsize,
        config: &SocketsConfig,
    ) -> nix::Result<Self> {
        let dest = InetAddr::from_std(dest);
        let is_ipv4 = match dest.ip() {
            IpAddr::V4(_) => true,
            IpAddr::V6(_) => false,
        };

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
        }

        nix::sys::socket::connect(fd, &SockAddr::Inet(dest))?;
        helpers::set_socket_options(fd, config);

        Ok(UdpSender {
            fd,
            buffer: {
                let mut packets = Vec::new();
                packets.reserve_exact(capacity.get());
                packets
            },
        })
    }

    /// Puts `packet` into an inner buffer. If a buffer is full, then all its
    /// content will be flushed and a specified `summary` will be updated.
    pub fn supply(
        &mut self,
        summary: &mut TestSummary,
        packet: &'a [u8],
    ) -> io::Result<SupplyResult> {
        let res = if self.buffer.len() == self.buffer.capacity() {
            self.flush(summary)?;
            SupplyResult::Flushed
        } else {
            SupplyResult::NotFlushed
        };

        self.buffer.push(DataPortion {
            transmitted: 0,
            slice: IoSlice::new(packet),
        });
        Ok(res)
    }

    /// Sends the a specified `packet` immediately (without buffering),
    /// returning a number of bytes send successfully, or `nix::Error`.
    pub fn send_one(&mut self, summary: &mut TestSummary, packet: &[u8]) -> nix::Result<usize> {
        match nix::sys::socket::send(self.fd, packet, MsgFlags::empty()) {
            Err(err) => {
                summary.update(SummaryPortion::new(packet.len(), 0, 1, 0));
                Err(err)
            }
            Ok(res) => {
                summary.update(SummaryPortion::new(packet.len(), res, 1, 1));
                Ok(res)
            }
        }
    }

    /// Flushes contents of an inner buffer (sends data to an endpoint),
    /// simultaneously updating a specified `summary`. A buffer will be
    /// empty after this operation.
    pub fn flush(&mut self, summary: &mut TestSummary) -> io::Result<()> {
        if !self.buffer.is_empty() {
            let packets_sent = sendmmsg(self.fd, self.buffer.as_mut_slice())?;

            let mut bytes_expected = 0usize;
            let mut bytes_sent = 0usize;
            for packet in &self.buffer {
                bytes_expected += packet.slice.len();
                bytes_sent += packet.transmitted;
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
    use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
    use std::ops::Deref;
    use std::os::unix::io::AsRawFd;
    use std::time::Duration;

    use pnet::packet::Packet;

    use lazy_static::lazy_static;

    use crate::core::construct_packets::construct_ipv4_udp_packet;
    use crate::core::statistics::TestSummary;
    use crate::core::udp_sender::helpers::set_socket_options;

    use super::*;

    lazy_static! {
        static ref DEFAULT_SOCKETS_CONFIG: SocketsConfig = SocketsConfig {
            broadcast: false,
            send_timeout: Duration::from_secs(3),
        };
        static ref UDP_SERVER: UdpSocket =
            UdpSocket::bind("127.0.0.1:0").expect("Failed to setup UDP_SERVER");
        static ref TEST_UDP_PACKET: Vec<u8> = {
            let address = SocketAddrV4::new(
                Ipv4Addr::new(127, 0, 0, 1),
                UDP_SERVER.local_addr().unwrap().port(),
            );
            construct_ipv4_udp_packet(&address, &address, b"Our packet", 8)
                .packet()
                .to_owned()
        };
    }

    #[test]
    fn constructs_buffer() {
        let buffer = UdpSender::new(
            &UDP_SERVER.local_addr().unwrap(),
            NonZeroUsize::new(354).unwrap(),
            &DEFAULT_SOCKETS_CONFIG,
        )
        .expect("UdpSender::new(...) failed");

        assert_eq!(buffer.buffer.capacity(), 354);
        assert_eq!(buffer.buffer.len(), 0);
    }

    #[test]
    fn test_packets_buffer() {
        const SUPPLY_COUNT: usize = 6;

        let mut summary = TestSummary::default();
        let mut buffer = UdpSender::new(
            &UDP_SERVER.local_addr().unwrap(),
            NonZeroUsize::new(4).unwrap(),
            &DEFAULT_SOCKETS_CONFIG,
        )
        .expect("UdpSender::new(...) failed");

        let check = |buffer: &UdpSender| {
            assert_eq!(buffer.buffer.capacity(), 4);
            assert_eq!(
                buffer.buffer.last().unwrap().slice.deref(),
                TEST_UDP_PACKET.as_slice()
            );
        };

        let mut supply = |buffer: &mut UdpSender| {
            buffer
                .supply(&mut summary, TEST_UDP_PACKET.as_ref())
                .expect("buffer.supply() failed");
        };

        supply(&mut buffer); // 1
        assert_eq!(buffer.buffer.len(), 1);
        check(&buffer);

        supply(&mut buffer); // 2
        assert_eq!(buffer.buffer.len(), 2);
        check(&buffer);

        supply(&mut buffer); // 3
        assert_eq!(buffer.buffer.len(), 3);
        check(&buffer);

        supply(&mut buffer); // 4
        assert_eq!(buffer.buffer.len(), 4);
        check(&buffer);

        // At this moment our buffer must flush itself
        supply(&mut buffer); // 5
        assert_eq!(buffer.buffer.len(), 1);
        check(&buffer);

        supply(&mut buffer); // 6
        assert_eq!(buffer.buffer.len(), 2);
        check(&buffer);

        buffer.flush(&mut summary).expect("buffer.flush() failed");
        assert_eq!(buffer.buffer.len(), 0);
        assert_eq!(buffer.buffer.capacity(), 4);

        // Check that our UdpSender has updates the TestSummary
        assert!(
            summary.megabytes_expected() == summary.megabytes_sent()
                && summary.megabytes_sent() == (SUPPLY_COUNT * TEST_UDP_PACKET.len()) / 1024 / 1024
        );
        assert!(
            summary.packets_expected() == summary.packets_sent()
                && summary.packets_sent() == SUPPLY_COUNT
        );
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

    #[test]
    fn test_send_one() {
        let mut summary = TestSummary::default();
        let mut sender = UdpSender::new(
            &UDP_SERVER.local_addr().unwrap(),
            NonZeroUsize::new(1).unwrap(),
            &DEFAULT_SOCKETS_CONFIG,
        )
        .expect("UdpSender::new(...) failed");
        dbg!();
        assert_eq!(summary.megabytes_expected(), 0);
        assert_eq!(summary.megabytes_sent(), 0);
        assert_eq!(summary.packets_expected(), 0);
        assert_eq!(summary.packets_sent(), 0);

        sender
            .send_one(&mut summary, TEST_UDP_PACKET.as_slice())
            .expect("sender.send_one(...) failed");

        // Check that our UdpSender has updates the TestSummary
        assert!(
            summary.megabytes_expected() == summary.megabytes_sent()
                && summary.megabytes_sent() == TEST_UDP_PACKET.len() / 1024 / 1024
        );
        assert!(
            summary.packets_expected() == summary.packets_sent() && summary.packets_sent() == 1
        );
    }
}
