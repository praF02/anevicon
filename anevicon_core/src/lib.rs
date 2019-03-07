/* anevicon: The most powerful UDP-based load generator, written in Rust.
 * Copyright (C) 2019  Temirkhan Myrzamadi <gymmasssorla@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * For more information see <https://github.com/Gymmasssorla/anevicon>.
 */

/*!
 * This crate can be used as a bot to build a botnet for simulating
 * [UDP-based DDoS attacks] (but only for educational and pentesting purposes,
 * see [the GPLv3 license], under which the library is distributed).
 *
 * This library was designed to be as convenient and reliable as it is
 * possible, and without any external dependencies (except of the standard
 * library). If you are just interested in one single program, please take
 * a look at [this one].
 *
 * # Examples
 * This example demonstrates sending of one hundred thousands packets  to the
 * example.com domain (just for an example, you should enter here your server):
 *
 * ```rust,no_run
 * use std::net::UdpSocket;
 * use std::num::NonZeroUsize;
 *
 * use anevicon_core::summary::TestSummary;
 * use anevicon_core::testing::{execute, HandleErrorResult};
 *
 * // Setup the socket connected to the example.com domain
 * let socket = UdpSocket::bind("0.0.0.0:0").expect("Cannot setup the socket");
 * socket
 *     .connect("93.184.216.34:80")
 *     .expect("Cannot connect the socket to example.com");
 *
 * let mut summary = TestSummary::default();
 *
 * // Finally, execute a test that will send 100000 packets
 * // each containing 32768 bytes.
 * execute(
 *     &socket,
 *     &vec![0; 32768],
 *     NonZeroUsize::new(100000).unwrap(),
 *     &mut summary,
 *     |error| panic!("{}", error),
 * );
 *
 * println!(
 *     "The total minutes passed: {}",
 *     summary.time_passed().as_secs() / 60
 * );
 * ```
 *
 * [UDP-based DDoS attacks]: https://en.wikipedia.org/wiki/UDP_flood_attack
 * [the GPLv3 license]: https://github.com/Gymmasssorla/anevicon/blob/master/LICENSE
 * [this one]: https://crates.io/crates/anevicon
 */

pub mod summary;
pub mod testing;
