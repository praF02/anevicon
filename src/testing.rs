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

use std::io::{self, IoVec};
use std::net::UdpSocket;
use std::thread::{self, Builder, JoinHandle};

use anevicon_core::{self, TestSummary, Tester};
use log::{error, info};

use super::config::{ArgsConfig, NetworkConfig};
use super::helpers::{self, SummaryWrapper};

pub fn execute_testers(
    config: &'static ArgsConfig,
    packet: &'static [u8],
) -> io::Result<Vec<JoinHandle<()>>> {
    let remaining_packets =
        config.exit_config.packets_count.get() % config.network_config.packets_per_syscall.get();
    let sendings_count = (config.exit_config.packets_count.get() - remaining_packets)
        / config.network_config.packets_per_syscall.get();

    Ok(init_sockets(&config.network_config)?
        .into_iter()
        .enumerate()
        .map(|(i, socket)| {
            Builder::new()
                .name(config.network_config.receivers[i].to_string())
                .spawn(move || {
                    let (mut ordinary, mut remaining) = (
                        generate_portions(config.network_config.packets_per_syscall.get(), &packet),
                        generate_portions(remaining_packets, &packet),
                    );

                    let mut summary = TestSummary::default();
                    let mut tester = Tester::new(&socket, &mut summary);

                    // Run the loop for the current worker until the allotted time expires or all
                    // the packets will have been sent
                    for _ in 0..sendings_count {
                        if let Err(error) = tester.send_multiple(&mut ordinary) {
                            error!("An error occurred while sending packets >>> {}!", error);
                        }

                        info!(
                            "Stats for the {receiver} receiver >>> {summary}.",
                            receiver = helpers::cyan(thread::current().name().unwrap()),
                            summary = SummaryWrapper(tester.summary()),
                        );

                        if tester.summary().time_passed() >= config.exit_config.test_duration {
                            info!(
                                "All the allotted time has passed >>> {summary}.",
                                summary = SummaryWrapper(tester.summary()),
                            );
                        }

                        thread::sleep(config.send_periodicity);
                    }

                    if let Err(error) = tester.send_multiple(&mut remaining) {
                        error!("An error occurred while sending packets >>> {}!", error);
                    }

                    info!(
                        "Stats for the {receiver} receiver >>> {summary}.",
                        receiver = helpers::cyan(thread::current().name().unwrap()),
                        summary = SummaryWrapper(tester.summary()),
                    );
                })
                .expect("Unable to spawn a new thread")
        })
        .collect())
}

fn init_sockets(config: &NetworkConfig) -> io::Result<Vec<UdpSocket>> {
    let mut sockets = Vec::with_capacity(config.receivers.len());

    for receiver in config.receivers.iter() {
        let socket = UdpSocket::bind(config.sender)?;
        socket.connect(receiver)?;
        socket.set_broadcast(config.broadcast)?;
        socket.set_write_timeout(Some(config.send_timeout))?;

        info!(
            "A new socket was initialized to the {receiver} receiver...",
            receiver = helpers::cyan(receiver),
        );

        sockets.push(socket);
    }

    Ok(sockets)
}

fn generate_portions(length: usize, packet: &[u8]) -> Vec<(usize, IoVec)> {
    let mut portions = Vec::with_capacity(length);

    for _ in 0..length {
        portions.push((0, IoVec::new(packet)));
    }

    portions
}
