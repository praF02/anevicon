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
use std::net::UdpSocket;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use anevicon_core::{self, TestSummary, Tester};
use humantime::format_duration;
use termion::color;

use packets_buffer::PacketsBuffer;
use packets_buffer::SupplyResult;

use crate::config::ArgsConfig;

mod packets;
mod packets_buffer;
mod sockets;
mod test_utils;

// A receiver name for this thread.
thread_local!(static RECEIVER: RefCell<String> = RefCell::new(String::from("Undefined")));

/// This is the key function which accepts a whole `ArgsConfig` and returns an
/// exit code (either 1 on failure or 0 on success).
pub fn run(config: ArgsConfig) -> i32 {
    let packets = match packets::construct_packets(&config.packets_config) {
        Err(err) => {
            error!("failed to construct a packet >>> {}!", err);
            return 1;
        }
        Ok(packet) => packet,
    };

    let sockets = match sockets::init_sockets(&config.sockets_config) {
        Err(err) => {
            error!("failed to initialize sockets >>> {}!", err);
            return 1;
        }
        Ok(sockets) => sockets,
    };

    wait(config.wait);

    let packets = Arc::new(packets);
    let config = Arc::new(config);
    let mut workers = Vec::with_capacity(config.sockets_config.receivers.len());

    for (receiver, socket) in sockets {
        let packets = packets.clone();
        let config = config.clone();

        workers.push(thread::spawn(move || {
            init_receiver(receiver);
            run_tester(config, packets, socket);
        }));
    }

    workers
        .into_iter()
        .for_each(|worker| worker.join().expect("A child thread has panicked"));
    0
}

fn run_tester(
    config: Arc<ArgsConfig>,
    packets: Arc<Vec<Vec<u8>>>,
    socket: UdpSocket,
) -> TestSummary {
    let mut summary = TestSummary::default();
    let mut tester = Tester::new(&socket, &mut summary);
    let mut buffer = PacketsBuffer::new(config.tester_config.packets_per_syscall);

    // Run the main cycle for the current worker, and exit if the allotted time
    // expires
    for (packet, _) in packets
        .iter()
        .cycle()
        .zip(0..config.tester_config.exit_config.packets_count.get())
    {
        match buffer.supply(&mut tester, (0, packet.as_slice())) {
            Err(err) => send_multiple_error(err),
            Ok(res) => {
                if res == SupplyResult::Flushed {
                    display_summary(tester.summary);
                }
            }
        }

        if tester.summary.time_passed() >= config.tester_config.exit_config.test_duration {
            display_expired_time();
            return summary;
        }

        thread::sleep(config.tester_config.send_periodicity);
    }

    if let Err(err) = buffer.complete(&mut tester) {
        send_multiple_error(err);
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
            &packets
                .iter()
                .cycle()
                .take(unsent.get())
                .map(|packet| packet.as_slice())
                .collect::<Vec<&[u8]>>(),
            config.tester_config.exit_config.test_duration,
        ) {
            ResendPacketsResult::Completed => display_packets_sent(),
            ResendPacketsResult::TimeExpired => display_expired_time(),
        }
    } else {
        display_packets_sent();
    }

    summary
}

/// Initializes the `RECEIVER` thread-local variable with the given value.
fn init_receiver(value: String) {
    RECEIVER.with(|receiver| *receiver.borrow_mut() = value);
}

/// Returns the thread-local value of a current receiver.
#[inline]
fn current_receiver() -> String {
    RECEIVER.with(|string| string.borrow().clone())
}

fn wait(duration: Duration) {
    warn!(
        "waiting {cyan}{time}{reset} and then starting to execute the tests...",
        time = format_duration(duration),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
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
fn resend_packets(tester: &mut Tester, packets: &[&[u8]], limit: Duration) -> ResendPacketsResult {
    info!(
        "trying to resend {cyan}{count}{reset} packets to {cyan}{receiver}{reset} that weren't \
         sent yet...",
        count = packets.len(),
        receiver = current_receiver(),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );

    for &packet in packets {
        if tester.summary.time_passed() >= limit {
            return ResendPacketsResult::TimeExpired;
        }

        while let Err(error) = tester.send_one(packet) {
            error!(
                "failed to send a packet to {cyan}{receiver}{reset} >>> {error}! Retrying the \
                 operation...",
                receiver = current_receiver(),
                error = error,
                cyan = color::Fg(color::Cyan),
                reset = color::Fg(color::Reset),
            );
        }
    }

    info!(
        "{cyan}{count}{reset} packets have been resent to {cyan}{receiver}{reset}.",
        count = packets.len(),
        receiver = current_receiver(),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );

    ResendPacketsResult::Completed
}

#[inline]
fn display_expired_time() {
    info!(
        "the allotted time has passed for {cyan}{receiver}{reset}.",
        receiver = current_receiver(),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );
}

#[inline]
fn display_packets_sent() {
    info!(
        "all the packets were sent for {cyan}{receiver}{reset}.",
        receiver = current_receiver(),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );
}

#[inline]
fn display_summary(summary: &TestSummary) {
    info!(
        "stats for {cyan}{receiver}{reset}:\n\tData Sent:     {cyan}{data_sent}{reset}\n\tAverage \
         Speed: {cyan}{average_speed}{reset}\n\tTime Passed:   {cyan}{time_passed}{reset}",
        receiver = current_receiver(),
        data_sent = format!(
            "{packets} packets ({megabytes} MB)",
            packets = summary.packets_sent(),
            megabytes = summary.megabytes_sent(),
        ),
        average_speed = format!(
            "{packets_per_sec} packets/sec ({mbps} Mbps)",
            packets_per_sec = summary.packets_per_sec(),
            mbps = summary.megabites_per_sec(),
        ),
        time_passed = format_duration(summary.time_passed()),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );
}

#[inline]
fn send_multiple_error<E: Display>(error: E) {
    error!(
        "failed to send packets to {cyan}{receiver}{reset} >>> {error}!",
        receiver = current_receiver(),
        error = error,
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use structopt::StructOpt;

    use test_utils::loopback_socket;

    use crate::core::packets::construct_packets;

    use super::*;

    #[test]
    fn resends_all_packets() {
        let mut summary = TestSummary::default();
        let socket = loopback_socket();
        let mut tester = Tester::new(&socket, &mut summary);

        let message = "Trying to resend packets which weren't sent yet".as_bytes();

        // All the packets will be sent because the allotted time is too long to be
        // expired
        assert_eq!(
            resend_packets(&mut tester, &vec![message; 12], Duration::from_secs(3656),),
            ResendPacketsResult::Completed
        );

        assert_eq!(tester.summary.packets_sent(), 12);
        assert_eq!(tester.summary.packets_expected(), 12);

        // Now the allotted time eventually expires, so check that resend_packets
        // returns TimeExpired
        assert_eq!(
            resend_packets(&mut tester, &vec![message; 12], Duration::from_nanos(1),),
            ResendPacketsResult::TimeExpired
        );
    }

    #[test]
    fn test_run_tester() {
        let socket = loopback_socket();

        let config = ArgsConfig::from_iter(&[
            "anevicon",
            "--receiver",
            &format!("{}", socket.local_addr().unwrap()),
            "--packets-count",
            "100",
            "--send-message",
            "My first message",
            "--send-message",
            "My second message",
            "--send-message",
            "My third message",
            "--send-file",
            "files/packet.txt",
            "--packet-length",
            "3000",
            "--wait",
            "0secs",
        ]);

        let packets = construct_packets(&config.packets_config).expect("Cannot construct packets");
        assert_eq!(packets.len(), 5);

        let summary = run_tester(Arc::new(config), Arc::new(packets), socket);

        assert_eq!(summary.packets_expected(), 100);
        assert_eq!(summary.packets_sent(), 100);
    }
}
