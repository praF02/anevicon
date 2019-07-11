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

use std::io;
use std::io::IoSlice;
use std::mem;
use std::os::unix::prelude::RawFd;

use libc::{self, c_uint, iovec, mmsghdr, msghdr};

/// A type alias that represents a portion to be sent, typically used in
/// `Tester::send_multiple`.
pub type Portion<'a> = (usize, IoSlice<'a>);

/// Sends all the specified messages of `portions` using one system call,
/// and assigns the total bytes sent for each packet into the first
/// slice elements.
///
/// # Errors
/// If the provided socket isn't connected to a remote server, the
/// `io::ErrorKind::NotConnected` error might be returned. Other kinds
/// of errors can also be returned, see the errors descriptions of
/// `io::ErrorKind` enumeration.
///
/// [sendmmsg]: http://man7.org/linux/man-pages/man2/sendmmsg.2.html
pub fn sendmmsg(fd: RawFd, portions: &mut [Portion]) -> io::Result<usize> {
    sendmmsg_impl(fd, portions)
}

fn sendmmsg_impl(fd: RawFd, portions: &mut [Portion]) -> io::Result<usize> {
    let mut messages: Vec<mmsghdr> = prepare_messages(portions);

    unsafe {
        match libc::sendmmsg(
            fd,
            &mut messages[0] as *mut mmsghdr,
            messages.len() as c_uint,
            0,
        ) {
            // `libc::sendmmsg` returns -1 one failure and initializes `errno`, so create
            // `io::Error` as it follows
            -1 => Err(io::Error::last_os_error()),
            portions_sent => {
                // `libc::sendmmsg` assigns a number of bytes sent for each packet to `mmsghdr`,
                // so copy it into our `DataPortion`
                for i in 0..messages.len() {
                    portions[i].0 = messages[i].msg_len as usize;
                }

                Ok(portions_sent as usize)
            }
        }
    }
}

/// Converts an mutable slice of the `Portion` structure to a vector of
/// `mmsghdr` that is able to be transmitted by `libc::sendmmsg`
fn prepare_messages(portions: &mut [Portion]) -> Vec<mmsghdr> {
    portions
        .iter_mut()
        .map(|(_, portion)| {
            mmsghdr {
                msg_hdr: {
                    let mut message = unsafe { mem::zeroed::<msghdr>() };
                    message.msg_iov = portion as *mut IoSlice as *mut iovec;
                    message.msg_iovlen = 1;

                    message
                },

                // `libc::sendmmsg` assigns a total bytes sent of a particular portion into this
                // value
                msg_len: 0,
            }
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

        let portions = &mut [
            (0, IoSlice::new(b"Welcome to the jungle")),
            (0, IoSlice::new(b"We got fun 'n' games")),
            (0, IoSlice::new(b"We got everything you want")),
        ];

        assert_eq!(
            sendmmsg(socket.as_raw_fd(), portions).expect("socket.sendmmsg(messages) has failed"),
            portions.len()
        );

        for portion in portions {
            assert_eq!(portion.0, portion.1.len());
        }
    }

    #[test]
    fn prepares_messages() {
        let portions = &mut [
            (0, IoSlice::new(b"Welcome to the jungle")),
            (0, IoSlice::new(b"We got fun 'n' games")),
            (0, IoSlice::new(b"We got everything you want")),
        ];

        let messages = prepare_messages(portions);

        for (headers, (_, portion)) in messages.iter().zip(portions.iter()) {
            assert_eq!(headers.msg_len, 0);

            assert_eq!(
                headers.msg_hdr.msg_iov as *const iovec,
                portion as *const IoSlice as *const iovec
            );
            assert_eq!(headers.msg_hdr.msg_iovlen, 1);
        }
    }
}
