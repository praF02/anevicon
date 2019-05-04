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

//! A module containing the key function `run` which does the main work.

use std::cell::RefCell;
use std::fmt::Display;
use std::io::IoVec;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anevicon_core::{self, Portion, TestSummary, Tester};
use colored::{ColoredString, Colorize};
use humantime::format_duration;

use crate::config::ArgsConfig;
use crate::sockets;

use super::helpers;

// A receiver name for this thread.
thread_local!(static RECEIVER: RefCell<ColoredString> = RefCell::new("Undefined".cyan()));

/// This is the key function which accepts a whole `ArgsConfig` and returns an
/// exit code (either 1 on failure or 0 on success).
pub fn run(config: ArgsConfig) -> i32 {
    let packet = match helpers::construct_packet(&config.packet_config) {
        Err(err) => {
            error!("Constructing a packet failed >>> {}!", err);
            return 1;
        }
        Ok(packet) => packet,
    };

    let sockets = match sockets::init_sockets(&config.sockets_config) {
        Err(err) => {
            error!("Sockets initialization failed >>> {}!", err);
            return 1;
        }
        Ok(sockets) => sockets,
    };

    wait(config.wait);

    let remaining_packets = unsafe {
        NonZeroUsize::new_unchecked(
            config.tester_config.exit_config.packets_count.get()
                % config.tester_config.packets_per_syscall.get(),
        )
    };
    let sendings_count = (config.tester_config.exit_config.packets_count.get()
        - remaining_packets.get())
        / config.tester_config.packets_per_syscall.get();

    let tester_config = Arc::new(config.tester_config);
    let packet = Arc::new(packet);

    for socket in sockets {
        let packet = packet.clone();
        let tester_config = tester_config.clone();

        thread::spawn(move || {
            init_receiver(socket.receiver().clone());

            let (mut ordinary, mut remaining) = (
                generate_portions(tester_config.packets_per_syscall, &packet),
                generate_portions(remaining_packets, &packet),
            );

            let mut summary = TestSummary::default();
            let mut tester = Tester::new(socket.socket(), &mut summary);

            // Run the main cycle for the current worker, and exit if the allotted time
            // expires
            for _ in 0..sendings_count {
                if let Err(error) = tester.send_multiple(&mut ordinary) {
                    send_multiple_error(error);
                }

                display_summary(tester.summary);

                if tester.summary.time_passed() >= tester_config.exit_config.test_duration {
                    display_expired_time();
                    return;
                }

                thread::sleep(tester_config.send_periodicity);
            }

            if let Err(error) = tester.send_multiple(&mut remaining) {
                send_multiple_error(error);
            }

            // We might have a situation when not all the required packets are sent, so fix
            // it
            let unsent = unsafe {
                NonZeroUsize::new_unchecked(
                    tester.summary.packets_expected() - tester.summary.packets_sent(),
                )
            };

            if unsent.get() != 0 {
                match resend_packets(
                    &mut tester,
                    &packet,
                    unsent,
                    tester_config.exit_config.test_duration,
                ) {
                    ResendPacketsResult::Completed => display_packets_sent(),
                    ResendPacketsResult::TimeExpired => display_expired_time(),
                }
            } else {
                display_packets_sent();
            }
        })
        .join()
        .expect("A child thread has panicked")
    }

    return 0;
}

/// Initializes the `RECEIVER` thread-local variable with the given value.
fn init_receiver(value: ColoredString) {
    RECEIVER.with(|receiver| *receiver.borrow_mut() = value);
}

/// Returns the thread-local value of a current receiver.
#[inline]
fn current_receiver() -> ColoredString {
    RECEIVER.with(|string| string.borrow().clone())
}

fn wait(duration: Duration) {
    warn!(
        "Waiting {time} and then starting to execute the tests...",
        time = helpers::cyan(format_duration(duration))
    );
    thread::sleep(duration);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ResendPacketsResult {
    Completed,
    TimeExpired,
}

/// Resends `count` packets using the given `tester`. If the `limit` is reached,
/// it will return `TimeExpired`, otherwise, `Completed`.
fn resend_packets(
    tester: &mut Tester,
    packet: &[u8],
    count: NonZeroUsize,
    limit: Duration,
) -> ResendPacketsResult {
    info!(
        "Trying to resend {count} packets to the {receiver} that weren't sent yet...",
        count = helpers::cyan(count.get()),
        receiver = current_receiver()
    );

    for _ in 0..count.get() {
        if tester.summary.time_passed() >= limit {
            return ResendPacketsResult::TimeExpired;
        }

        loop {
            if let Err(error) = tester.send_one(IoVec::new(packet)) {
                error!(
                    "Sending a packet to the {receiver} failed >>> {error}! Retrying the \
                     operation...",
                    receiver = current_receiver(),
                    error = error
                );
            } else {
                break;
            }
        }
    }

    info!(
        "{count} packets have been resent to the {receiver}.",
        count = helpers::cyan(count.get()),
        receiver = current_receiver()
    );

    ResendPacketsResult::Completed
}

#[inline]
fn display_expired_time() {
    info!(
        "The allotted time has passed for the {receiver}.",
        receiver = current_receiver()
    );
}

#[inline]
fn display_packets_sent() {
    info!(
        "All the packets were sent for the {receiver}.",
        receiver = current_receiver()
    );
}

#[inline]
fn display_summary(summary: &TestSummary) {
    info!(
        "Stats for {receiver}:\n\tData Sent:     {data_sent}\n\tAverage Speed: \
         {average_speed}\n\tTime Passed:   {time_passed}",
        receiver = current_receiver(),
        data_sent = helpers::cyan(format!(
            "{packets} packets ({megabytes} MB)",
            packets = summary.packets_sent(),
            megabytes = summary.megabytes_sent(),
        )),
        average_speed = helpers::cyan(format!(
            "{packets_per_sec} packets/sec ({mbps} Mbps)",
            packets_per_sec = summary.packets_per_sec(),
            mbps = summary.megabites_per_sec(),
        )),
        time_passed = helpers::cyan(format_duration(summary.time_passed())),
    );
}

#[inline]
fn send_multiple_error<E: Display>(error: E) {
    error!(
        "Sending packets to the {receiver} failed >>> {error}!",
        receiver = current_receiver(),
        error = error
    );
}

/// Generates exactly `length` portions referring to a `packet`. They will be
/// sent to all receivers later.
fn generate_portions(length: NonZeroUsize, packet: &[u8]) -> Vec<Portion> {
    let mut portions = Vec::with_capacity(length.get());

    for _ in 0..length.get() {
        portions.push((0, IoVec::new(packet)));
    }

    portions
}

#[cfg(test)]
mod tests {
    use std::net::UdpSocket;
    use std::time::Duration;

    use super::*;

    /// Returns a `UdpSocket` connected to itself for testing reasons.
    fn loopback_socket() -> UdpSocket {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("A socket error");
        socket
            .connect(socket.local_addr().unwrap())
            .expect("Cannot connect the socket to itself");
        socket
    }

    #[test]
    fn test_generate_portions() {
        let portion: &[u8] = b"Something very very useful for all of us";

        for (bytes, vec) in generate_portions(NonZeroUsize::new(5).unwrap(), portion) {
            // This value must be always zero for future use of sendmmsg
            assert_eq!(bytes, 0);
            assert_eq!(portion, vec.as_ref());
        }
    }

    #[test]
    fn resends_all_packets() {
        let mut summary = TestSummary::default();
        let socket = loopback_socket();
        let mut tester = Tester::new(&socket, &mut summary);

        let message = b"Trying to resend packets which weren't sent yet";

        // All the packets will be sent because the allotted time is too long to be
        // expired
        assert_eq!(
            resend_packets(
                &mut tester,
                message,
                NonZeroUsize::new(12).unwrap(),
                Duration::from_secs(3656),
            ),
            ResendPacketsResult::Completed
        );

        assert_eq!(tester.summary.packets_sent(), 12);
        assert_eq!(tester.summary.packets_expected(), 12);

        // Now the allotted time eventually expires, so check that resend_packets
        // returns TimeExpired
        assert_eq!(
            resend_packets(
                &mut tester,
                message,
                NonZeroUsize::new(12).unwrap(),
                Duration::from_nanos(1),
            ),
            ResendPacketsResult::TimeExpired
        );
    }
}
