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

use config::{ArgsConfig, ExitConfig};
use helpers::construct_packet;
use logging::setup_logging;

use std::io;
use std::net::UdpSocket;
use std::thread;

mod config;
mod helpers;
mod logging;

use colored::{ColoredString, Colorize as _};
use humantime::format_duration;
use log::{error, info, trace, warn};
use termion::color;

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

    if let Err(error) = execute(&args_config, &packet) {
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

fn execute(args_config: &ArgsConfig, packet: &[u8]) -> io::Result<()> {
    let test_name = format!("\"{}\"", args_config.test_name).magenta().italic();
    let socket = init_socket(args_config, &test_name)?;

    warn!(
        "The test {test_name} has initialized the socket to the remote server successfully. Now \
         sleeping {sleeping_time} and then starting to test...",
        test_name = test_name,
        sleeping_time = format_duration(args_config.wait).to_string().cyan(),
    );

    thread::sleep(args_config.wait);

    info!(
        "The test {test_name} has started to test the {server_address} server until either \
         {packets_count} packets will be sent or {test_duration} will be passed.",
        test_name = test_name,
        server_address = args_config.receiver.to_string().cyan(),
        packets_count = args_config.exit_config.packets_count.to_string().cyan(),
        test_duration = format_duration(args_config.exit_config.test_duration)
            .to_string()
            .cyan(),
    );

    let mut summary = TestSummary::default();

    loop {
        for _ in 0..args_config.display_periodicity.get() {
            if let Err(error) = testing::send(&socket, packet, &mut summary) {
                error!("An error occurred while sending a packet >>> {}!", error);
            }

            if is_limit_reached(&args_config.exit_config, &summary, &test_name) {
                return Ok(());
            }

            thread::sleep(args_config.send_periodicity);
        }

        info!(
            "The test {test_name} is running >>> {summary}.",
            test_name = test_name,
            summary = format_summary(&summary),
        );
    }
}

// Suggest to inline this function because it is used in a continious cycle
#[inline]
fn is_limit_reached(
    exit_config: &ExitConfig,
    summary: &TestSummary,
    test_name: &ColoredString,
) -> bool {
    if summary.time_passed() >= exit_config.test_duration {
        info!(
            "The allotted time of the test {test_name} has passed >>> {summary}.",
            test_name = test_name,
            summary = format_summary(&summary)
        );

        true
    } else if summary.packets_sent() == exit_config.packets_count.get() {
        info!(
            "The test {test_name} has sent all the required packets >>> {summary}.",
            test_name = test_name,
            summary = format_summary(&summary)
        );

        true
    } else {
        false
    }
}

fn init_socket(args_config: &ArgsConfig, test_name: &ColoredString) -> io::Result<UdpSocket> {
    info!(
        "The test {test_name} is initializing the socket to the remote server {server_address} \
         using the {sender_address} sender address...",
        test_name = test_name,
        server_address = args_config.receiver.to_string().cyan(),
        sender_address = args_config.sender.to_string().cyan(),
    );

    let socket = UdpSocket::bind(args_config.sender)?;
    socket.connect(args_config.receiver)?;
    socket.set_write_timeout(Some(args_config.send_timeout))?;

    Ok(socket)
}

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
