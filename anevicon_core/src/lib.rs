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

//! This crate can be used as a bot to build a [botnet] for simulating
//! [UDP-based DDoS attacks] (but only for educational and pentesting purposes,
//! see [the GPLv3 license], under which the library is distributed).
//!
//! This library was designed to be as convenient and reliable as it is
//! possible, but for only Linux-based systems. If you are just interested in
//! one single program, please take a look at [this one].
//!
//! # Examples
//! This example demonstrates sending a couple of messages to the `example.com`
//! domain (just for an example, you should enter here your server):
//!
//! ([`examples/minimal.rs`])
//! ```rust,no_run
//! use std::net::UdpSocket;
//! use std::os::unix::io::AsRawFd;
//! use std::io::IoSlice;
//!
//! use anevicon_core::{TestSummary, Tester};
//!
//! fn main() {
//!     // Setup the socket connected to the example.com domain
//!     let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
//!     socket.connect("93.184.216.34:80").unwrap();
//!
//!     // Setup all the I/O vectors (messages) we want to send
//!     let payload = &mut [
//!         (0, IoSlice::new(b"Generals gathered in their masses")),
//!         (0, IoSlice::new(b"Just like witches at black masses")),
//!         (0, IoSlice::new(b"Evil minds that plot destruction")),
//!         (0, IoSlice::new(b"Sorcerers of death's construction")),
//!     ];
//!
//!     // Send all the created messages using only one system call
//!     let mut summary = TestSummary::default();
//!     let mut tester = Tester::new(socket.as_raw_fd(), &mut summary);
//!
//!     println!(
//!         "The total packets sent: {}, the total seconds passed: {}",
//!         tester.send_multiple(payload).unwrap().packets_sent(),
//!         summary.time_passed().as_secs()
//!     );
//! }
//! ```
//!
//! [UDP-based DDoS attacks]: https://en.wikipedia.org/wiki/UDP_flood_attack
//! [the GPLv3 license]: https://github.com/Gymmasssorla/anevicon/blob/master/LICENSE
//! [this one]: https://crates.io/crates/anevicon
//! [botnet]: https://en.wikipedia.org/wiki/Botnet
//! [here]: https://github.com/Gymmasssorla/anevicon/blob/master/src/main.rs
//! [`examples/minimal.rs`]: https://github.com/Gymmasssorla/anevicon/blob/master/anevicon_core/examples/minimal.rs

pub use sendmmsg::Portion;
pub use summary::{SummaryPortion, TestSummary};
pub use tester::Tester;

mod sendmmsg;
pub mod summary;
pub mod tester;
