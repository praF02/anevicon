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

// Executes all the tests specified by a user. If an I/O error has occurred, it
// will be returned.
pub fn execute_all(args_config: ArgsConfig, packet: Vec<u8>) -> io::Result<()> {
    let arc_config = Arc::new(RwLock::new(args_config));
    let arc_packet = Arc::new(RwLock::new(packet));
    let unlocked_config = arc_config.read().unwrap();

    let mut testers = Vec::with_capacity(unlocked_config.network_config.receivers.len());
    for i in 0..unlocked_config.network_config.receivers.len() {
        testers.push(Tester::new(arc_config.clone(), arc_packet.clone(), i)?);
    }

    warn!(
        "Sleeping {sleeping_time} and then starting to test until either {packets} packets will \
         be sent or {duration} will be passed for each receiver...",
        sleeping_time = helpers::cyan(format_duration(unlocked_config.wait)),
        packets = helpers::cyan(unlocked_config.exit_config.packets_count),
        duration = helpers::cyan(format_duration(unlocked_config.exit_config.test_duration))
    );
    thread::sleep(unlocked_config.wait);

    let pool = ThreadPool::new(testers.len());
    testers.into_iter().for_each(|tester| {
        pool.execute(move || {
            tester.run();
        })
    });

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

    fn run(&self) -> SummaryWrapper {
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
                    return summary;
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
        Ok(socket)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::num::NonZeroUsize;
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

    #[test]
    fn exits_correctly() {
        let config = ExitConfig {
            test_duration: Duration::from_secs(2156),
            packets_count: unsafe { NonZeroUsize::new_unchecked(15) },
        };
        let mut summary = SummaryWrapper(TestSummary::default());
        assert!(!Tester::is_limit_reached(&summary, &config));

        // Update the TestSummary so that the limit will be reached
        summary.0.update(5648, 15);
        assert!(Tester::is_limit_reached(&summary, &config));

        let config = ExitConfig {
            test_duration: Duration::from_secs(2),
            packets_count: unsafe { NonZeroUsize::new_unchecked(15) },
        };
        let summary = SummaryWrapper(TestSummary::default());
        assert!(!Tester::is_limit_reached(&summary, &config));

        // Wait two seconds so that the allotted time will be exactly passed
        thread::sleep(config.test_duration);
        assert!(Tester::is_limit_reached(&summary, &config));
    }
}
