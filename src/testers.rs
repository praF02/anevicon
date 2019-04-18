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

use super::config::{ArgsConfig, ExitConfig, NetworkConfig};
use super::helpers::{self, SummaryWrapper};

use anevicon_core::{self, TestSummary};
use humantime::format_duration;
use log::{error, info, trace, warn};
use threadpool::ThreadPool;

use std::io::{self, IoVec};
use std::net::UdpSocket;
use std::num::NonZeroUsize;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Instant;

pub fn execute_tester(
    config: &'static ArgsConfig,
    packet: IoVec<'static>,
    receiver: usize,
) -> io::Result<SummaryWrapper> {
    let socket = init_socket(&config.network_config, receiver)?;

    let mut summary = SummaryWrapper(TestSummary::default());
    let mut tester = anevicon_core::tester::Tester::new(&socket, &mut summary.0);
    let (remaining_bytes, sendings_count) = compute_strategy(
        config.exit_config.packets_count,
        config.network_config.packets_per_syscall,
    );
    let mut portions = generate_portions(&packet, config.network_config.packets_per_syscall);

    // Run the loop for the current worker until the allotted time expires
    for _ in 0..sendings_count {
        if let Err(error) = tester.send_multiple(&mut portions) {
            error!("An error occurred while sending a packet >>> {}!", error);
        }

        info!(
            "Stats for the {receiver} receiver >>> {summary}.",
            receiver = helpers::cyan(config.network_config.receivers[receiver]),
            summary = summary,
        );

        if tester.summary().time_passed() >= config.exit_config.test_duration {
            info!(
                "All the allotted time has passed >>> {summary}.",
                summary = summary
            );
            return Ok(summary);
        }

        thread::sleep(config.send_periodicity);
    }

    tester.send_multiple(&mut generate_portions(
        &packet,
        config.network_config.packets_per_syscall,
    ));
    Ok(summary)
}

// Divide the whole required packets count into sections to send multiple
// packets per one syscall
fn compute_strategy(
    packets_count: NonZeroUsize,
    packets_per_syscall: NonZeroUsize,
) -> (usize, usize) {
    let remaining_packets = packets_count.get() % packets_per_syscall.get();
    let sendings_count = (packets_count.get() - remaining_packets) / packets_per_syscall.get();
    (remaining_packets, sendings_count)
}

fn generate_portions(packet: &IoVec<'static>, count: NonZeroUsize) -> Vec<(usize, IoVec<'static>)> {
    let mut portions = Vec::with_capacity(count.get());

    for i in 0..count.get() {
        portions.push((0, IoVec::new(packet.as_ref())));
    }

    portions
}

// Initialize a `UdpSocket` connected to a receiver by the specified index
fn init_socket(network_config: &NetworkConfig, receiver: usize) -> io::Result<UdpSocket> {
    info!(
        "Initializing the socket to the {receiver} receiver using the {sender} sender address...",
        receiver = helpers::cyan(network_config.receivers[receiver]),
        sender = helpers::cyan(network_config.sender),
    );

    let socket = UdpSocket::bind(network_config.sender)?;
    socket.connect(network_config.receivers[receiver])?;
    socket.set_broadcast(network_config.broadcast)?;
    socket.set_write_timeout(Some(network_config.send_timeout))?;

    trace!("A new initialized socket: {:?}", &socket);
    Ok(socket)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::thread;
    use std::time::Duration;

    use structopt::StructOpt;

    #[test]
    fn tester_runs_correctly() {
        let server = UdpSocket::bind("0.0.0.0:0").expect("Unable to setup the server");

        let config = ArgsConfig::from_iter(&[
            "anevicon",
            "--receiver",
            &server.local_addr().unwrap().to_string(),
            "--packets-count",
            "14",
            "--send-message",
            "Are you gonna take me home tonight?",
        ]);
        let packet = helpers::construct_packet(&config.packet_config)
            .expect("helpers::construct_packet() has failed");
        let packets_count = config.exit_config.packets_count.get();

        assert_eq!(
            Tester::new(
                Arc::new(RwLock::new(config)),
                Arc::new(RwLock::new(packet)),
                0,
            )
            .expect("Tester::new() has failed")
            .run()
            .0
            .packets_sent(),
            packets_count
        );
    }

    #[test]
    fn test_init_socket() {
        let config = NetworkConfig {
            receivers: vec![
                "45.89.52.36:5236".parse().unwrap(),
                "89.52.36.41:256".parse().unwrap(),
                "85.53.23.57:45687".parse().unwrap(),
            ],
            sender: "0.0.0.0:0".parse().unwrap(),
            send_timeout: Duration::from_secs(25),
            broadcast: true,
        };

        let socket = Tester::init_socket(&config, 1).expect("Tester::init_socket() has failed");

        // Test that the specified IP address isn't a global one
        assert_eq!(socket.local_addr().unwrap().ip().is_global(), false);
        assert_eq!(socket.write_timeout().unwrap(), Some(config.send_timeout));
        assert_eq!(socket.broadcast().unwrap(), config.broadcast);

        let config = NetworkConfig {
            receivers: vec![
                "45.89.52.36:5236".parse().unwrap(),
                "89.52.36.41:256".parse().unwrap(),
                "135.225.66.89:45288".parse().unwrap(),
            ],
            sender: "0.0.0.0:0".parse().unwrap(),
            send_timeout: Duration::from_millis(984),
            broadcast: false,
        };

        let socket = Tester::init_socket(&config, 0).expect("Tester::init_socket() has failed");

        // Test that the specified IP address isn't a global one
        assert_eq!(socket.local_addr().unwrap().ip().is_global(), false);
        assert_eq!(socket.write_timeout().unwrap(), Some(config.send_timeout));
        assert_eq!(socket.broadcast().unwrap(), config.broadcast);
    }
}
