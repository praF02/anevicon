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

use std::io;
use std::net::UdpSocket;
use std::thread;

use colored::Colorize as _;
use humantime::format_duration;
use log::info;

use super::config::{ArgsConfig, StopConditionsConfig};
use super::summary::TestSummary;

pub fn execute(args_config: &ArgsConfig, packet: &[u8]) -> io::Result<TestSummary> {
    let test_name = args_config.test_name.magenta().italic();

    info!(
        "The test {test_name} is connecting to the remote server {server_address} \
         using the {sender_address} sender address...",
        test_name = test_name,
        server_address = args_config.receiver.to_string().cyan(),
        sender_address = args_config.sender.to_string().cyan(),
    );

    // Complete any necessary stuff with the specified socket
    let socket = UdpSocket::bind(args_config.sender)?;
    socket.connect(args_config.receiver)?;
    socket.set_write_timeout(Some(args_config.send_timeout))?;

    info!(
        "The test {test_name} has connected to the remote server successfully. Now \
         sleeping {sleeping_time} and then starting to test...",
        test_name = test_name,
        sleeping_time = format_duration(args_config.wait).to_string().cyan(),
    );

    thread::sleep(args_config.wait);
    let mut summary = TestSummary::new();

    info!(
        "The test {test_name} has started to test the {server_address} server \
         until either {packets_count} packets will be sent or {test_duration} \
         will be passed.",
        test_name = test_name,
        server_address = args_config.receiver.to_string().cyan(),
        packets_count = args_config
            .stop_conditions_config
            .packets_count
            .to_string()
            .cyan(),
        test_duration = format_duration(args_config.stop_conditions_config.test_duration)
            .to_string()
            .cyan(),
    );

    // Run a test until either all packets will be sent or alloted
    // time will pass. Return the test summary for future analysis.
    loop {
        for _ in 0..args_config.display_periodicity.get() {
            summary.update(socket.send(packet)?, 1);

            if let Some(reason) = check_end_cond(&args_config.stop_conditions_config, &summary) {
                match reason {
                    EndReason::TimePassed => info!(
                        "The allotted time of the test {test_name} has passed >>> {summary}.",
                        test_name = test_name,
                        summary = summary
                    ),
                    EndReason::PacketsSent => info!(
                        "The test {test_name} has sent all the required packets >>> {summary}.",
                        test_name = test_name,
                        summary = summary
                    ),
                }

                return Ok(summary);
            }

            thread::sleep(args_config.send_periodicity);
        }

        info!(
            "The test {test_name} is running >>> {summary}.",
            test_name = test_name,
            summary = summary,
        );
    }
}

fn check_end_cond(
    stop_conditions_config: &StopConditionsConfig,
    summary: &TestSummary,
) -> Option<EndReason> {
    if summary.time_passed() >= stop_conditions_config.test_duration {
        Some(EndReason::TimePassed)
    } else if summary.packets_sent() == stop_conditions_config.packets_count.get() {
        Some(EndReason::PacketsSent)
    } else {
        None
    }
}

#[derive(Debug, Eq, PartialEq)]
enum EndReason {
    TimePassed,
    PacketsSent,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::num::NonZeroUsize;
    use std::time::Duration;

    use lazy_static::lazy_static;
    use structopt::StructOpt;

    use super::super::helpers::random_packet;

    lazy_static! {
        static ref DEFAULT_PACKET: Vec<u8> =
            unsafe { random_packet(NonZeroUsize::new_unchecked(65000)) };

        static ref DEFAULT_SERVER: UdpSocket = UdpSocket::bind("0.0.0.0:0")
            .expect("Cannot setup the testing server with the address 0.0.0.0:0");

        // The first command-line argument doesn't have any meaning for CLAP
        static ref DEFAULT_CONFIG: ArgsConfig =
            ArgsConfig::from_iter_safe(vec![
                "anevicon",
                "--receiver",
                &DEFAULT_SERVER.local_addr().unwrap().to_string(),
            ])
            .expect("The command-line arguments are incorrectly specified");
    }

    #[test]
    fn sends_all_packets() {
        // Assign a very low required packets count to prevent our
        // lovely Travis CI and your computer for a shameful breaking
        let packets = unsafe { NonZeroUsize::new_unchecked(25) };

        let mut config = DEFAULT_CONFIG.clone();
        config.stop_conditions_config.packets_count = packets;

        // Check that our tester has successfully sent all the packets
        assert_eq!(
            execute(&config, &DEFAULT_PACKET)
                .expect("An error occurred during the test")
                .packets_sent(),
            packets.get()
        );
    }

    #[test]
    fn stops_if_packets_sent() {
        let mut summary = TestSummary::new();
        let stop_config = StopConditionsConfig {
            test_duration: Duration::from_secs(99999),
            packets_count: unsafe { NonZeroUsize::new_unchecked(std::usize::MAX) },
        };

        // The default duration and the default packets count are too big,
        // so this line must return None
        assert_eq!(check_end_cond(&stop_config, &summary), None);

        // Update the summary and check that all the packets were sent
        summary.update(1549335, stop_config.packets_count.get());
        assert_eq!(
            check_end_cond(&stop_config, &summary),
            Some(EndReason::PacketsSent)
        );
    }

    #[test]
    fn stops_if_time_passed() {
        let summary = TestSummary::new();
        let stop_config = StopConditionsConfig {
            test_duration: Duration::from_secs(5),
            packets_count: unsafe { NonZeroUsize::new_unchecked(std::usize::MAX) },
        };

        // The default required time is not reached at this point, so this
        // line must return None
        assert_eq!(check_end_cond(&stop_config, &summary), None);

        // Sleep five seconds and check that the alloted time has passed
        thread::sleep(stop_config.test_duration);
        assert_eq!(
            check_end_cond(&stop_config, &summary),
            Some(EndReason::TimePassed)
        );
    }
}
