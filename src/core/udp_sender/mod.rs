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

//! This file is used to send raw UDP/IP messages to a web server.

use std::convert::TryInto;
use std::io::IoSlice;
use std::net::{IpAddr, SocketAddr};
use std::num::NonZeroUsize;
use std::os::raw::c_void;
use std::os::unix::io::RawFd;
use std::time::{Duration, Instant};
use std::{io, mem, thread};

use sendmmsg::sendmmsg;

use crate::core::statistics::{SummaryPortion, TestSummary};

mod sendmmsg;

const IP_RECVERR: libc::c_int = 11;

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
    /// system call (`--test-intensity`). When this buffer is full, then it
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
        test_intensity: NonZeroUsize,
        dest: &SocketAddr,
        broadcast: bool,
    ) -> io::Result<Self> {
        let fd = match unsafe {
            libc::socket(
                match dest.ip() {
                    IpAddr::V4(_) => libc::AF_INET,
                    IpAddr::V6(_) => libc::AF_INET6,
                },
                libc::SOCK_RAW,
                libc::IPPROTO_RAW,
            )
        } {
            -1 => return Err(io::Error::last_os_error()),
            value => value,
        };

        set_socket_option_safe(
            fd,
            libc::SOL_SOCKET,
            libc::SO_SNDTIMEO,
            &libc::timeval {
                tv_sec: 1,
                tv_usec: 0,
            },
        )?;

        set_socket_option_safe(
            fd,
            libc::SOL_SOCKET,
            libc::SO_BROADCAST,
            if broadcast { &1 } else { &0 },
        )?;

        set_socket_option_safe(
            fd,
            match dest.ip() {
                IpAddr::V4(_) => libc::SOL_IP,
                IpAddr::V6(_) => libc::SOL_IPV6,
            },
            IP_RECVERR,
            &1,
        )?;

        connect_socket_safe(fd, dest)?;

        Ok(UdpSender {
            fd,
            buffer: {
                let mut packets = Vec::new();
                packets.reserve_exact(test_intensity.get());
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
    /// returning a number of bytes send successfully, or `io::Error`.
    pub fn send_one(&mut self, summary: &mut TestSummary, packet: &[u8]) -> io::Result<usize> {
        match unsafe {
            libc::send(
                self.fd,
                packet as *const _ as *const c_void,
                packet.len(),
                0,
            )
        } {
            -1 => {
                summary.update(SummaryPortion::new(packet.len(), 0, 1, 0));
                Err(io::Error::last_os_error())
            }
            res => {
                let res = res as usize;
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
            let start = Instant::now();

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

            // If the operation took less than a second, then sleep the rest of time
            // according `--test-intensity`:
            if let Some(wait) = Duration::from_secs(1).checked_sub(start.elapsed()) {
                thread::sleep(wait);
            }
        }

        Ok(())
    }
}

impl<'a> Drop for UdpSender<'a> {
    fn drop(&mut self) {
        unsafe {
            if libc::close(self.fd) == -1 {
                panic!("Failed to drop UdpSender");
            }
        }
    }
}

fn set_socket_option_safe<T>(
    fd: RawFd,
    level: libc::c_int,
    name: libc::c_int,
    value: &T,
) -> io::Result<()> {
    match unsafe {
        libc::setsockopt(
            fd,
            level,
            name,
            value as *const _ as *const c_void,
            mem::size_of_val(value).try_into().unwrap(),
        )
    } {
        -1 => Err(io::Error::last_os_error()),
        _ => Ok(()),
    }
}

fn connect_socket_safe(fd: RawFd, dest: &SocketAddr) -> io::Result<()> {
    let ret = match dest {
        SocketAddr::V4(dest_v4) => {
            let octets = dest_v4.ip().octets();

            let addr_v4 = libc::sockaddr_in {
                sin_family: libc::AF_INET.try_into().unwrap(),
                sin_port: dest.port().to_be(),
                sin_addr: libc::in_addr {
                    s_addr: u32::to_be(
                        (u32::from(octets[0]) << 24)
                            | (u32::from(octets[1]) << 16)
                            | (u32::from(octets[2]) << 8)
                            | u32::from(octets[3]),
                    ),
                },
                ..unsafe { mem::zeroed() }
            };

            unsafe {
                libc::connect(
                    fd,
                    &addr_v4 as *const _ as *const libc::sockaddr,
                    mem::size_of::<libc::sockaddr>().try_into().unwrap(),
                )
            }
        }
        SocketAddr::V6(dest_v6) => {
            let addr_v6 = libc::sockaddr_in6 {
                sin6_family: libc::AF_INET6.try_into().unwrap(),
                sin6_port: dest.port().to_be(),
                sin6_addr: libc::in6_addr {
                    s6_addr: dest_v6.ip().octets(),
                },
                sin6_flowinfo: dest_v6.flowinfo(),
                sin6_scope_id: dest_v6.scope_id(),
            };

            unsafe {
                libc::connect(
                    fd,
                    &addr_v6 as *const _ as *const libc::sockaddr,
                    mem::size_of::<libc::sockaddr>().try_into().unwrap(),
                )
            }
        }
    };

    match ret {
        -1 => Err(io::Error::last_os_error()),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;
    use std::net::UdpSocket;
    use std::ops::Deref;

    use etherparse::PacketBuilder;

    use lazy_static::lazy_static;

    use crate::core::statistics::TestSummary;

    use super::*;

    lazy_static! {
        static ref UDP_SERVER: UdpSocket =
            UdpSocket::bind("localhost:0").expect("Failed to setup UDP_SERVER");
        static ref TEST_UDP_PACKET: Vec<u8> = {
            let payload = b"Our packet";

            let builder = PacketBuilder::ipv4(
                Ipv4Addr::LOCALHOST.octets(),
                Ipv4Addr::LOCALHOST.octets(),
                8,
            )
            .udp(
                UDP_SERVER.local_addr().unwrap().port(),
                UDP_SERVER.local_addr().unwrap().port(),
            );

            let mut serialized = Vec::<u8>::with_capacity(builder.size(payload.len()));
            builder
                .write(&mut serialized, payload)
                .expect("Failed to serialize a UDP/IPv4 packet into Vec<u8>");
            serialized
        };
    }

    #[test]
    fn constructs_buffer() {
        let buffer = UdpSender::new(
            NonZeroUsize::new(354).unwrap(),
            &UDP_SERVER.local_addr().unwrap(),
            false,
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
            NonZeroUsize::new(4).unwrap(),
            &UDP_SERVER.local_addr().unwrap(),
            false,
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
    fn test_send_one() {
        let mut summary = TestSummary::default();
        let mut sender = UdpSender::new(
            NonZeroUsize::new(1).unwrap(),
            &UDP_SERVER.local_addr().unwrap(),
            false,
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
