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
 * The test abstractions to easily describe and execute your own tests.
 *
 * For examples please take a look at the main documentation page.
*/

use std::io;
use std::net::UdpSocket;
use std::num::NonZeroUsize;

use super::summary::TestSummary;

/**
 * Sends the specified `packet` `packets_count` times, using the `socket`
 * and at the time updating the specified `summary`.
 *
 * `error_handler` is used for handling errors that can be produced by
 * the [`send`] method. You can either just print an error message or
 * terminate the test.
 *
 * [`send`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.send
 */
pub fn execute<F: Fn(io::Error) -> HandleErrorResult>(
    socket: &UdpSocket,
    packet: &[u8],
    packets_count: NonZeroUsize,
    summary: &mut TestSummary,
    error_handler: F,
) -> TestResult {
    for _ in 0..packets_count.get() {
        match socket.send(packet) {
            Err(error) => match error_handler(error) {
                HandleErrorResult::Continue => {
                    continue;
                }
                HandleErrorResult::Terminate => {
                    return TestResult::Terminated;
                }
            },
            Ok(bytes) => summary.update(bytes, 1),
        }
    }

    TestResult::Succeed
}

/// A test handling error result, either continuing or terminating.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HandleErrorResult {
    /// Continue packet sending
    Continue,

    /// Terminate a whole test
    Terminate,
}

/// A test total result that returns the `execute` method.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TestResult {
    /// The `execute` function will return this variant if either your error
    /// handling function returned the `HandleErrorResult::Continue` or it
    /// wasn't called.
    Succeed,

    /// The `execute` function will return this variant if and only if your
    /// error handling function returned the `HandleErrorResult::Terminate`
    /// variant.
    Terminated,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sends_all_packets() {
        let packets_count = unsafe { NonZeroUsize::new_unchecked(25) };

        let server = UdpSocket::bind("0.0.0.0:0").expect("Cannot setup the server");
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Cannot setup the socket");
        socket
            .connect(server.local_addr().unwrap())
            .expect("Cannot connect the socket to the local server");

        let mut summary = TestSummary::default();

        assert_eq!(
            execute(
                &socket,
                &vec![0; 32768],
                packets_count,
                &mut summary,
                |error| { panic!("{}", error) }
            ),
            TestResult::Succeed
        );

        assert_eq!(summary.packets_sent(), packets_count.get());
    }
}
