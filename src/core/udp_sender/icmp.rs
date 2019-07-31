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

//! This file is used to handle incoming ICMP error-messages. Further reading: https://linux.die.net/man/2/recvfrom (`MSG_ERRQUEUE` section).

use std::io;
use std::os::raw::c_void;
use std::os::unix::io::RawFd;
use std::ptr;
use std::sync::{Arc, Mutex};

use lazy_static::lazy_static;

use crate::core::statistics::TestSummary;

#[allow(dead_code)]
const SO_EE_ORIGIN_NONE: u8 = 0;
#[allow(dead_code)]
const SO_EE_ORIGIN_LOCAL: u8 = 1;
const SO_EE_ORIGIN_ICMP: u8 = 2;
#[allow(dead_code)]
const SO_EE_ORIGIN_ICMP6: u8 = 3;

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
            Arc::new(Mutex::new(Vec::with_capacity(4096)));
    };

    let mut msg_control = MSG_CONTROL
        .lock()
        .expect("Another thread has panicked while holding MSG_CONTROL");

    let mut msghdr = libc::msghdr {
        msg_name: ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: ptr::null_mut(),
        msg_iovlen: 0,
        msg_control: msg_control.as_mut_ptr() as *mut _ as *mut c_void,
        msg_controllen: msg_control.capacity(),
        msg_flags: 0,
    };

    match unsafe { libc::recvmsg(fd, &mut msghdr, libc::MSG_ERRQUEUE) } {
        -1 => {
            let error = io::Error::last_os_error();
            if error.kind() == io::ErrorKind::WouldBlock {
                dbg!("WouldBlock");
                Ok(())
            } else {
                Err(error)
            }
        }
        _ => {
            unsafe { parse_msghdr(&msghdr as *const libc::msghdr, summary) };
            Ok(())
        }
    }
}

unsafe fn parse_msghdr(msghdr: *const libc::msghdr, summary: &mut TestSummary) {
    let mut cmsghdr = libc::CMSG_FIRSTHDR(msghdr);

    while !cmsghdr.is_null() {
        if (*cmsghdr).cmsg_level == libc::IPPROTO_IP && (*cmsghdr).cmsg_type == super::IP_RECVERR {
            #[allow(clippy::cast_ptr_alignment)]
            let error = libc::CMSG_DATA(cmsghdr) as *const sock_extended_err;

            if (*error).ee_origin == SO_EE_ORIGIN_ICMP {
                summary.update_icmp((*error).ee_type, (*error).ee_code);
            }
        }

        cmsghdr = libc::CMSG_NXTHDR(msghdr, cmsghdr);
    }
}
