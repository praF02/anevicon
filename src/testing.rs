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

use std::fmt::Display;
use std::io::{self, IoVec};
use std::net::UdpSocket;
use std::num::NonZeroUsize;
use std::thread::{self, Builder, JoinHandle};
use std::time::Duration;

use anevicon_core::{self, TestSummary, Tester};
use colored::ColoredString;
use humantime::format_duration;
use log::{error, info, warn};

use super::config::{ArgsConfig, NetworkConfig};
use super::helpers::{self, SummaryWrapper};

pub fn execute_testers(
    config: &'static ArgsConfig,
    packet: &'static [u8],
) -> io::Result<Vec<JoinHandle<TestSummary>>> {
    wait(config.wait);

    let remaining_packets = unsafe {
        NonZeroUsize::new_unchecked(
            config.exit_config.packets_count.get()
                % config.network_config.packets_per_syscall.get(),
        )
    };

    let sendings_count = (config.exit_config.packets_count.get() - remaining_packets.get())
        / config.network_config.packets_per_syscall.get();

    Ok(init_sockets(&config.network_config)?
        .into_iter()
        .enumerate()
        .map(|(i, socket)| {
            Builder::new()
                .name(config.network_config.receivers[i].to_string())
                .spawn(move || {
                    let (mut ordinary, mut remaining) = (
                        generate_portions(config.network_config.packets_per_syscall, &packet),
                        generate_portions(remaining_packets, &packet),
                    );

                    let mut summary = TestSummary::default();
                    let mut tester = Tester::new(&socket, &mut summary);

                    // Run the main cycle for the current worker, and exit if the allotted time
                    // expires
                    for _ in 0..sendings_count {
                        if let Err(error) = tester.send_multiple(&mut ordinary) {
                            send_multiple_error(error);
                        }

                        display_summary(SummaryWrapper(tester.summary()));

                        if tester.summary().time_passed() >= config.exit_config.test_duration {
                            display_expired_time(SummaryWrapper(tester.summary()));
                            return summary;
                        }

                        thread::sleep(config.send_periodicity);
                    }

                    if let Err(error) = tester.send_multiple(&mut remaining) {
                        send_multiple_error(error);
                    }

                    // We might have a situation when not all the required packets are sent, so fix
                    // it
                    let unsent = unsafe {
                        NonZeroUsize::new_unchecked(
                            tester.summary().packets_expected() - tester.summary().packets_sent(),
                        )
                    };

                    if unsent.get() != 0 {
                        match resend_packets(
                            &mut tester,
                            &packet,
                            unsent,
                            config.exit_config.test_duration,
                        ) {
                            ResendPacketsResult::Completed => {
                                display_packets_sent(SummaryWrapper(tester.summary()));
                            }
                            ResendPacketsResult::TimeExpired => {
                                display_expired_time(SummaryWrapper(tester.summary()))
                            }
                        }
                    } else {
                        display_packets_sent(SummaryWrapper(tester.summary()));
                    }

                    summary
                })
                .expect("Unable to spawn a new thread")
        })
        .collect())
}

fn wait(duration: Duration) {
    warn!(
        "Waiting {time} and then starting to initialize the sockets and executing the tests...",
        time = helpers::cyan(format_duration(duration))
    );
    thread::sleep(duration);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ResendPacketsResult {
    Completed,
    TimeExpired,
}

fn resend_packets(
    tester: &mut Tester,
    packet: &[u8],
    count: NonZeroUsize,
    limit: Duration,
) -> ResendPacketsResult {
    info!(
        "Trying to resend {count} packets to the {receiver} that weren't sent...",
        count = helpers::cyan(count.get()),
        receiver = current_receiver()
    );

    for _ in 0..count.get() {
        if tester.summary().time_passed() >= limit {
            return ResendPacketsResult::TimeExpired;
        }

        loop {
            if let Err(error) = tester.send_one(IoVec::new(packet)) {
                error!(
                    "An error occurred while sending a packet to the {receiver} >>> {error}! \
                     Retrying the operation...",
                    receiver = current_receiver(),
                    error = error
                );
            } else {
                break;
            }
        }
    }

    info!(
        "{count} packets were successfully resent to the {receiver}.",
        count = helpers::cyan(count.get()),
        receiver = current_receiver()
    );

    ResendPacketsResult::Completed
}

#[inline]
fn display_expired_time(summary: SummaryWrapper) {
    info!(
        "The allotted time has passed for the {receiver} >>> {summary}.",
        receiver = current_receiver(),
        summary = summary,
    );
}

#[inline]
fn display_packets_sent(summary: SummaryWrapper) {
    info!(
        "All the packets were sent for the {receiver} >>> {summary}.",
        receiver = current_receiver(),
        summary = summary
    );
}

#[inline]
fn display_summary(summary: SummaryWrapper) {
    info!(
        "Stats for the {receiver} >>> {summary}.",
        receiver = current_receiver(),
        summary = summary,
    );
}

#[inline]
fn send_multiple_error<E: Display>(error: E) {
    error!(
        "An error occurred while sending packets to the {receiver} >>> {error}!",
        receiver = current_receiver(),
        error = error
    );
}

// Extracts the current receiver from the current thread name and colorizes it
// as cyan
#[inline]
fn current_receiver() -> ColoredString {
    helpers::cyan(thread::current().name().unwrap())
}

fn init_sockets(config: &NetworkConfig) -> io::Result<Vec<UdpSocket>> {
    let mut sockets = Vec::with_capacity(config.receivers.len());

    for receiver in config.receivers.iter() {
        let socket = UdpSocket::bind(config.sender)?;
        socket.connect(receiver)?;
        socket.set_broadcast(config.broadcast)?;
        socket.set_write_timeout(Some(config.send_timeout))?;

        info!(
            "A new socket was initialized to the {receiver} receiver.",
            receiver = helpers::cyan(receiver),
        );

        sockets.push(socket);
    }

    Ok(sockets)
}

fn generate_portions(length: NonZeroUsize, packet: &[u8]) -> Vec<(usize, IoVec)> {
    let mut portions = Vec::with_capacity(length.get());

    for _ in 0..length.get() {
        portions.push((0, IoVec::new(packet)));
    }

    portions
}

#[cfg(test)]
mod tests {
    use super::*;

    use lazy_static::lazy_static;
    use structopt::StructOpt;

    use std::mem;
    use std::time::Duration;

    lazy_static! {
        static ref UDP_SOCKET: UdpSocket = {
            let socket = UdpSocket::bind("0.0.0.0:0").expect("A socket error");
            socket
                .connect(socket.local_addr().unwrap())
                .expect("Cannot connect the socket to itself");
            socket
        };
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
        let mut tester = Tester::new(&UDP_SOCKET, &mut summary);

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

        assert_eq!(tester.summary().packets_sent(), 12);
        assert_eq!(tester.summary().packets_expected(), 12);

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

    #[test]
    fn test_init_sockets() {
        let config = NetworkConfig {
            receivers: vec![
                "45.89.52.36:5236".parse().unwrap(),
                "89.52.36.41:256".parse().unwrap(),
                "85.53.23.57:45687".parse().unwrap(),
            ],
            sender: "0.0.0.0:0".parse().unwrap(),
            send_timeout: Duration::from_secs(25),
            broadcast: true,
            packets_per_syscall: NonZeroUsize::new(500).unwrap(),
        };

        for socket in init_sockets(&config).expect("init_socket() has failed") {
            assert_eq!(socket.local_addr().unwrap().ip().is_global(), false);
            assert_eq!(socket.write_timeout().unwrap(), Some(config.send_timeout));
            assert_eq!(socket.broadcast().unwrap(), config.broadcast);
        }
    }

    #[test]
    fn executes_testers_correctly() {
        let config = ArgsConfig::from_iter(&[
            "anevicon",
            "--receiver",
            &UDP_SOCKET.local_addr().unwrap().to_string(),
            "--packets-count",
            "14",
            "--send-message",
            "Are you gonna take me home tonight?",
        ]);

        let packet = helpers::construct_packet(&config.packet_config)
            .expect("helpers::construct_packet() has failed");

        for handle in execute_testers(unsafe { mem::transmute(&config) }, unsafe {
            mem::transmute(packet.as_slice())
        })
        .expect("execute_testers(...) returned an error")
        {
            assert_eq!(
                handle
                    .join()
                    .expect("handle.join() returned an error")
                    .packets_sent(),
                14
            );
        }
    }
}
