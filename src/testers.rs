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

use anevicon_core::summary::TestSummary;
use anevicon_core::testing;
use humantime::format_duration;
use log::{error, info, trace, warn};
use threadpool::ThreadPool;

use std::io;
use std::net::UdpSocket;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Instant;

pub fn execute(unlocked_config: ArgsConfig, packet: Vec<u8>) -> io::Result<()> {
    let (arc_config, arc_packet) = (
        Arc::new(RwLock::new(unlocked_config)),
        Arc::new(RwLock::new(packet)),
    );
    let unlocked_config = arc_config.read().unwrap();

    let mut testers = Vec::with_capacity(unlocked_config.network_config.receivers.len());
    for i in 0..unlocked_config.network_config.receivers.len() {
        testers.push(Tester::new(arc_config.clone(), arc_packet.clone(), i)?);
    }

    warn!(
        "Sleeping {sleeping_time} and then starting to test until either {packets} packets will \
         be sent or {duration} will be passed...",
        sleeping_time = helpers::cyan(format_duration(unlocked_config.wait)),
        packets = helpers::cyan(unlocked_config.exit_config.packets_count),
        duration = helpers::cyan(format_duration(unlocked_config.exit_config.test_duration))
    );
    thread::sleep(unlocked_config.wait);

    let pool = ThreadPool::new(testers.len());

    testers
        .into_iter()
        .for_each(|tester| pool.execute(move || tester.run()));

    trace!("Spawned testers: {:?}", pool);
    pool.join();
    Ok(())
}

#[derive(Debug)]
struct Tester {
    config: Arc<RwLock<ArgsConfig>>,
    packet: Arc<RwLock<Vec<u8>>>,
    receiver: usize,
    socket: UdpSocket,
}

impl Tester {
    fn new(
        config: Arc<RwLock<ArgsConfig>>,
        packet: Arc<RwLock<Vec<u8>>>,
        receiver: usize,
    ) -> io::Result<Tester> {
        let socket = Tester::init_socket(
            &config
                .read()
                .expect("Error while acquiring reading mode on RwLock")
                .network_config,
            receiver,
        )?;

        Ok(Tester {
            socket,
            config,
            packet,
            receiver,
        })
    }

    fn run(&self) {
        let (packet, config) = (self.packet.read().unwrap(), self.config.read().unwrap());

        let mut summary = SummaryWrapper(TestSummary::default());

        // Run the loop for the current worker until one of the specified exit
        // conditions will become true
        loop {
            let instant = Instant::now();

            while instant.elapsed() < config.display_periodicity {
                if let Err(error) = testing::send(&self.socket, &packet, &mut summary.0) {
                    error!("An error occurred while sending a packet >>> {}!", error);
                }

                if Tester::is_limit_reached(&summary, &config.exit_config) {
                    return;
                }

                thread::sleep(config.send_periodicity);
            }

            info!(
                "Stats for the {receiver} receiver >>> {summary}.",
                receiver = helpers::cyan(config.network_config.receivers[self.receiver]),
                summary = summary,
            );
        }
    }

    // Suggest to inline this function because it is used in a continuous cycle
    #[inline]
    fn is_limit_reached(summary: &SummaryWrapper, exit_config: &ExitConfig) -> bool {
        if summary.0.time_passed() >= exit_config.test_duration {
            info!(
                "All the allotted time has passed >>> {summary}.",
                summary = summary
            );

            true
        } else if summary.0.packets_sent() == exit_config.packets_count.get() {
            info!(
                "All the required packets were sent >>> {summary}.",
                summary = summary
            );

            true
        } else {
            false
        }
    }

    // Initialize a `UdpSocket` connected to a receiver by the specified index
    fn init_socket(network_config: &NetworkConfig, receiver: usize) -> io::Result<UdpSocket> {
        info!(
            "Initializing the socket to the {receiver} receiver using the {sender} sender \
             address...",
            receiver = helpers::cyan(network_config.receivers[receiver]),
            sender = helpers::cyan(network_config.sender),
        );

        let socket = UdpSocket::bind(network_config.sender)?;
        socket.connect(network_config.receivers[receiver])?;
        socket.set_broadcast(network_config.broadcast)?;
        socket.set_write_timeout(Some(network_config.send_timeout))?;

        trace!("A new initialized socket: {:?}", &socket);
        info!(
            "The socket was initialized to the {receiver} receiver using the {sender} sender \
             address successfully.",
            receiver = helpers::cyan(network_config.receivers[receiver]),
            sender = helpers::cyan(network_config.sender),
        );

        Ok(socket)
    }
}
