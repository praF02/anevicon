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

//! Minimal safe bindings to `libc::sendmmsg`.

use std::io;
use std::io::IoSlice;
use std::mem;

use super::DataPortion;

/// Sends all the specified `packets` using a single system call. `fd` is a
/// file descriptor of a socket.
///
/// # Returns
/// It returns a total number of transmitted messages. It can be less or equal
/// to `packets.len()`.
///
/// # References
/// For more information please read https://linux.die.net/man/2/sendmmsg.
pub fn sendmmsg(fd: libc::c_int, packets: &mut [DataPortion]) -> io::Result<usize> {
    let mut messages: Vec<libc::mmsghdr> = prepare_messages(packets);

    unsafe {
        match libc::sendmmsg(
            fd,
            &mut messages[0] as *mut libc::mmsghdr,
            messages.len() as libc::c_uint,
            0,
        ) {
            -1 => Err(io::Error::last_os_error()),
            portions_sent => {
                // libc::sendmmsg assigns a number of bytes sent for each packet to
                // mmsghdr.msg_len, so copy it into our DataPortion
                for i in 0..messages.len() {
                    packets[i].transmitted = messages[i].msg_len as usize;
                }

                Ok(portions_sent as usize)
            }
        }
    }
}

/// Converts an mutable slice of the `DataPortion` structure to a vector of
/// `mmsghdr` that is able to be transmitted by `libc::sendmmsg`.
fn prepare_messages(packets: &mut [DataPortion]) -> Vec<libc::mmsghdr> {
    packets
        .iter_mut()
        .map(|packet| libc::mmsghdr {
            msg_hdr: {
                let mut message = unsafe { mem::zeroed::<libc::msghdr>() };
                message.msg_iov = &mut packet.slice as *mut IoSlice as *mut libc::iovec;
                message.msg_iovlen = 1;

                message
            },

            msg_len: 0,
        })
        .collect()
}

#[cfg(test)]
mod test {
    use std::net::UdpSocket;
    use std::os::unix::io::AsRawFd;

    use super::*;

    #[test]
    fn sends_all_data() {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("UdpSocket::bind() has failed");
        socket
            .connect(socket.local_addr().unwrap())
            .expect("socket.connect() has failed");

        let packets = &mut [
            DataPortion {
                transmitted: 0usize,
                slice: IoSlice::new(b"Welcome to the jungle"),
            },
            DataPortion {
                transmitted: 0usize,
                slice: IoSlice::new(b"We got fun 'n' games"),
            },
            DataPortion {
                transmitted: 0usize,
                slice: IoSlice::new(b"We got everything you want"),
            },
        ];

        assert_eq!(
            sendmmsg(socket.as_raw_fd(), packets).expect("socket.sendmmsg(messages) has failed"),
            packets.len()
        );

        for packet in packets {
            assert_eq!(packet.transmitted, packet.slice.len());
        }
    }

    #[test]
    fn prepares_messages() {
        let packets = &mut [
            DataPortion {
                transmitted: 0usize,
                slice: IoSlice::new(b"Welcome to the jungle"),
            },
            DataPortion {
                transmitted: 0usize,
                slice: IoSlice::new(b"We got fun 'n' games"),
            },
            DataPortion {
                transmitted: 0usize,
                slice: IoSlice::new(b"We got everything you want"),
            },
        ];

        let messages = prepare_messages(packets);

        for (headers, packet) in messages.iter().zip(packets.iter()) {
            assert_eq!(headers.msg_len, 0);

            assert_eq!(
                headers.msg_hdr.msg_iov as *const libc::iovec,
                &packet.slice as *const IoSlice as *const libc::iovec
            );
            assert_eq!(headers.msg_hdr.msg_iovlen, 1);
        }
    }
}
