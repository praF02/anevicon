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

//! This file is used to handle incoming ICMP error-messages.

use std::convert::TryInto;
use std::io;
use std::os::raw::c_void;
use std::os::unix::io::RawFd;
use std::ptr;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::core::statistics::TestSummary;

const MSG_CONTROLLEN: usize = 4096;

// For Linux
const SO_EE_ORIGIN_ICMP: i32 = 0;
const IP_RECVERR: i32 = 11;

#[repr(C)]
struct sock_extended_err {
    ee_errno: u32,
    ee_origin: u8,
    ee_type: u8,
    ee_code: u8,
    ee_pad: u8,
    ee_info: u32,
    ee_data: u32,
}

pub fn extract_icmp(fd: RawFd, summary: &mut TestSummary) -> io::Result<()> {
    lazy_static! {
        static ref MSG_CONTROL: Arc<Mutex<Vec<u8>>> =
            Arc::new(Mutex::new(Vec::with_capacity(MSG_CONTROLLEN)));
    };

    let mut msg_control = MSG_CONTROL
        .lock()
        .expect("Another thread has panicked while holding MSG_CONTROL");

    let mut msg_header = libc::msghdr {
        msg_name: ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: ptr::null_mut(),
        msg_iovlen: 0,
        msg_control: msg_control.as_mut_ptr() as *mut _ as *mut c_void,
        msg_controllen: MSG_CONTROLLEN,
        msg_flags: 0,
    };

    unsafe {
        match libc::recvmsg(fd, &mut msg_header, libc::MSG_ERRQUEUE | libc::MSG_DONTWAIT) {
            -1 => Err(io::Error::last_os_error()),
            _ => {
                let mut cmsg_header: *const libc::cmsghdr;
                let mut errors: *const sock_extended_err;

                cmsg_header = libc::CMSG_FIRSTHDR(&msg_header as *const libc::msghdr);
                while !cmsg_header.is_null() {
                    cmsg_header =
                        libc::CMSG_NXTHDR(&msg_header as *const libc::msghdr, cmsg_header);

                    if (*cmsg_header).cmsg_level == libc::IPPROTO_IP
                        && (*cmsg_header).cmsg_type == IP_RECVERR
                    {
                        errors = libc::CMSG_DATA(cmsg_header) as *const sock_extended_err;
                        if (*errors).ee_origin == SO_EE_ORIGIN_ICMP.try_into().unwrap() {
                            summary.update_icmp((*errors).ee_type, (*errors).ee_code);
                        }
                    }
                }

                Ok(())
            }
        }
    }
}
