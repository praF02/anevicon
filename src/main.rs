// anevicon: The most powerful UDP-based load generator, written in Rust.
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

use anevicon_core::summary::TestSummary;
use anevicon_core::testing;

use config::{ArgsConfig, ExitConfig, NetworkConfig};
use helpers::construct_packet;
use logging::setup_logging;

use std::io;
use std::net::UdpSocket;
use std::sync::{Arc, RwLock};
use std::thread;

mod config;
mod helpers;
mod logging;

use colored::Colorize as _;
use humantime::format_duration;
use log::{error, info, trace, warn};
use termion::color;
use threadpool::ThreadPool;

fn main() {
    let args_config = ArgsConfig::setup();
    title();

    setup_logging(&args_config.logging_config);
    trace!("{:?}", args_config);

    let packet = match construct_packet(&args_config.packet_config) {
        Err(error) => {
            error!("Constructing the packet failed >>> {}!", error);
            std::process::exit(1);
        }
        Ok(packet) => packet,
    };

    if let Err(error) = execute(args_config, packet) {
        error!("Testing the server failed >>> {}!", error);
        std::process::exit(1);
    }
}

fn title() {
    println!(
        "         {}",
        r"                        _                 ".cyan()
    );
    println!(
        "         {}",
        r"  __ _ _ __   _____   _(_) ___ ___  _ __  ".cyan()
    );
    println!(
        "         {}",
        r" / _` | '_ \ / _ \ \ / / |/ __/ _ \| '_ \ ".cyan()
    );
    println!(
        "         {}",
        r"| (_| | | | |  __/\ V /| | (_| (_) | | | |".cyan()
    );
    println!(
        "         {}",
        r" \__,_|_| |_|\___| \_/ |_|\___\___/|_| |_|".cyan()
    );
    println!(
        "{}\n",
        "A high-performance UDP-based load generator, written in Rust"
            .green()
            .underline()
    );
}

fn execute(args_config: ArgsConfig, packet: Vec<u8>) -> io::Result<()> {
    let mut sockets = Vec::with_capacity(args_config.network_config.receivers.len());
    for i in 0..args_config.network_config.receivers.len() {
        sockets.push(init_socket(&args_config.network_config, i)?);
    }

    warn!(
        "All the sockets were initialized successfully. Now sleeping {sleeping_time} and then \
         starting to test all the specified receivers...",
        sleeping_time = format_duration(args_config.wait).to_string().cyan(),
    );
    thread::sleep(args_config.wait);

    // Spawn the workers in parallel mode and block the current thread until they
    // all finished their work
    spawn_workers(
        Arc::new(RwLock::new(args_config)),
        Arc::new(RwLock::new(packet)),
        sockets,
    )
    .join();
    Ok(())
}

// Initialize a `UdpSocket` connected to the `network_config.receivers[index]`
fn init_socket(network_config: &NetworkConfig, index: usize) -> io::Result<UdpSocket> {
    info!(
        "Initializing the socket to the {receiver} receiver using the {sender} sender address...",
        receiver = network_config.receivers[index].to_string().cyan(),
        sender = network_config.sender.to_string().cyan(),
    );

    let socket = UdpSocket::bind(network_config.sender)?;
    socket.connect(network_config.receivers[index])?;
    socket.set_write_timeout(Some(network_config.send_timeout))?;

    info!(
        "The socket was initialized to the {receiver} receiver using the {sender} sender address \
         successfully.",
        receiver = network_config.receivers[index].to_string().cyan(),
        sender = network_config.sender.to_string().cyan(),
    );

    Ok(socket)
}

fn spawn_workers(
    args_config: Arc<RwLock<ArgsConfig>>,
    packet: Arc<RwLock<Vec<u8>>>,
    sockets: Vec<UdpSocket>,
) -> ThreadPool {
    let workers = ThreadPool::new(sockets.len());
    let local_config = args_config.read().unwrap();

    for (socket, &receiver) in sockets
        .into_iter()
        .zip(local_config.network_config.receivers.iter())
    {
        let local_config = args_config.clone();
        let local_packet = packet.clone();

        workers.execute(move || {
            let local_config = local_config.read().unwrap();
            let local_packet = local_packet.read().unwrap();

            let mut summary = TestSummary::default();

            // Run the loop for the current worker until one of the specified exit
            // conditions will become true
            loop {
                for _ in 0..local_config.display_periodicity.get() {
                    if let Err(error) = testing::send(&socket, &local_packet, &mut summary) {
                        error!("An error occurred while sending a packet >>> {}!", error);
                    }

                    if is_limit_reached(&local_config.exit_config, &summary) {
                        return;
                    }

                    thread::sleep(local_config.send_periodicity);
                }

                info!(
                    "Stats for the {receiver} receiver >>> {summary}.",
                    receiver = receiver.to_string().cyan(),
                    summary = format_summary(&summary),
                );
            }
        });
    }

    workers
}

// Suggest to inline this function because it is used in a continious cycle
#[inline]
fn is_limit_reached(exit_config: &ExitConfig, summary: &TestSummary) -> bool {
    if summary.time_passed() >= exit_config.test_duration {
        info!(
            "All the allotted time has passed >>> {summary}.",
            summary = format_summary(&summary)
        );

        true
    } else if summary.packets_sent() == exit_config.packets_count.get() {
        info!(
            "All the required packets were sent >>> {summary}.",
            summary = format_summary(&summary)
        );

        true
    } else {
        false
    }
}

// Format a `TestSummary` in a fancy style with colors, styles and other stuff
fn format_summary(summary: &TestSummary) -> String {
    format!(
        "Packets sent: {style}{packets} ({megabytes} MB){reset_style}, the average speed: \
         {style}{mbps} Mbps ({packets_per_sec} packets/sec){reset_style}, time passed: \
         {style}{time_passed}{reset_style}",
        packets = summary.packets_sent(),
        megabytes = summary.megabytes_sent(),
        mbps = summary.megabites_per_sec(),
        packets_per_sec = summary.packets_per_sec(),
        time_passed = format_duration(summary.time_passed()),
        style = format_args!("{}", color::Fg(color::Cyan)),
        reset_style = format_args!("{}", color::Fg(color::Reset)),
    )
}
