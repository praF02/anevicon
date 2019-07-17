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
use std::num::NonZeroUsize;
use std::os::raw::c_int;

use super::sendmmsg::sendmmsg;
use super::Portion;

/// A structure representing a raw IPv4/IPv6 socket with a buffer. The buffer is
/// described below, see the `buffer` field.
pub struct UdpSender<'a> {
    descriptor: c_int,

    /// The buffer capacity equals to a number of packets transmitted per a
    /// system call (`--packets-per-syscall`). When this buffer is full, then it
    /// will be flushed to an endpoint using `libc::sendmmsg`.
    buffer: Vec<Portion<'a>>,
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
            descriptor: {
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
}
