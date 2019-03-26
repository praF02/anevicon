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
//! possible, and without any external dependencies (except the standard
//! library). If you are just interested in one single program, please take a
//! look at [this one].
//!
//! # Examples
//! This example demonstrates sending of one thousand packets to the example.com
//! domain (just for an example, you should enter here your server):
//!
//! ([`examples/minimal.rs`])
//! ```rust,no_run
//! use anevicon_core::summary::TestSummary;
//! use anevicon_core::testing::send;
//!
//! // Setup the socket connected to the example.com domain
//! let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
//! socket.connect("93.184.216.34:80").unwrap();
//!
//! let packet = vec![0; 32768];
//! let mut summary = TestSummary::default();
//!
//! // Execute a test that will send one thousand packets
//! // each containing 32768 bytes.
//! for _ in 0..1000 {
//!     if let Err(error) = send(&socket, &packet, &mut summary) {
//!         panic!("{}", error);
//!     }
//! }
//!
//! println!(
//!     "The total seconds passed: {}",
//!     summary.time_passed().as_secs()
//! );
//! ```
//!
//! For a real-world example please go [here].
//!
//! [UDP-based DDoS attacks]: https://en.wikipedia.org/wiki/UDP_flood_attack
//! [the GPLv3 license]: https://github.com/Gymmasssorla/anevicon/blob/master/LICENSE
//! [this one]: https://crates.io/crates/anevicon
//! [botnet]: https://en.wikipedia.org/wiki/Botnet
//! [here]: https://github.com/Gymmasssorla/anevicon/blob/master/src/main.rs
//! [`examples/minimal.rs`]: https://github.com/Gymmasssorla/anevicon/blob/master/anevicon_core/examples/minimal.rs

pub mod summary;
pub mod testing;
