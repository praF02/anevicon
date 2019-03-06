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
use std::net::{SocketAddr, UdpSocket};
use std::num::NonZeroUsize;
use std::thread;
use std::time::Duration;

use colored::Colorize as _;
use humantime::format_duration;
use log::{error, info};

use super::summary::TestSummary;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TestLaunchOptions<'a> {
    /// A receiver of generated traffic.
    pub receiver: SocketAddr,

    /// A sender of generated traffic.
    pub sender: SocketAddr,

    /// A waiting time span before a test execution used to prevent a
    /// launch of an erroneous (unwanted) test.
    pub wait: Duration,

    /// A periodicity of sending packets. This option can be used to
    /// decrease test intensity.
    pub send_periodicity: Duration,

    /// A count of packets per displaying test summaries. It is highly
    /// recommended to not set a too small value (say, 6).
    pub display_periodicity: NonZeroUsize,

    /// A timeout of sending every single packet. If a timeout is
    /// reached, an error will be printed.
    pub send_timeout: Duration,

    /// A name of a future test. This option lets produce the program
    /// beautiful output and doesn't make any sense on test performing.
    pub test_name: String,

    /// A count of packets for sending. When this limit is reached,
    /// then the program will exit.
    pub packets_count: NonZeroUsize,

    /// A whole test duration. When this limit is reached, then the
    /// program will exit.
    pub test_duration: Duration,

    // A single packet for sending multiple times.
    pub packet: &'a [u8],
}

pub fn execute(launch_options: &TestLaunchOptions) -> io::Result<TestSummary> {
    let test_name = launch_options.test_name.magenta().italic();

    info!(
        "The test {test_name} is initializing the socket to the remote server \
         {server_address} using the {sender_address} sender address...",
        test_name = test_name,
        server_address = launch_options.receiver.to_string().cyan(),
        sender_address = launch_options.sender.to_string().cyan(),
    );

    // Complete any necessary stuff with the specified socket
    let socket = UdpSocket::bind(launch_options.sender)?;
    socket.connect(launch_options.receiver)?;
    socket.set_write_timeout(Some(launch_options.send_timeout))?;

    info!(
        "The test {test_name} has initialized the socket to the remote server \
         successfully. Now sleeping {sleeping_time} and then starting to test...",
        test_name = test_name,
        sleeping_time = format_duration(launch_options.wait).to_string().cyan(),
    );

    thread::sleep(launch_options.wait);
    let mut summary = TestSummary::new();

    info!(
        "The test {test_name} has started to test the {server_address} server \
         until either {packets_count} packets will be sent or {test_duration} \
         will be passed.",
        test_name = test_name,
        server_address = launch_options.receiver.to_string().cyan(),
        packets_count = launch_options.packets_count.to_string().cyan(),
        test_duration = format_duration(launch_options.test_duration)
            .to_string()
            .cyan(),
    );

    // Run a test until either all packets will be sent or alloted
    // time will pass. Return the test summary for future analysis.
    loop {
        for _ in 0..launch_options.display_periodicity.get() {
            match socket.send(launch_options.packet) {
                Err(error) => error!("An error occurred while sending a packet >>> {}!", error),
                Ok(bytes) => summary.update(bytes, 1),
            }

            if let Some(reason) = check_end_cond(launch_options, &summary) {
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

            thread::sleep(launch_options.send_periodicity);
        }

        info!(
            "The test {test_name} is running >>> {summary}.",
            test_name = test_name,
            summary = summary,
        );
    }
}

fn check_end_cond(launch_options: &TestLaunchOptions, summary: &TestSummary) -> Option<EndReason> {
    if summary.time_passed() >= launch_options.test_duration {
        Some(EndReason::TimePassed)
    } else if summary.packets_sent() == launch_options.packets_count.get() {
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

    use super::super::helpers::random_packet;

    lazy_static! {
        static ref DEFAULT_SERVER: UdpSocket = UdpSocket::bind("0.0.0.0:0")
            .expect("Cannot setup the testing server with the address 0.0.0.0:0");

        static ref DEFAULT_PACKET: Vec<u8> = unsafe {
            random_packet(NonZeroUsize::new_unchecked(32768))
        };

        static ref DEFAULT_OPTIONS: TestLaunchOptions<'static> = TestLaunchOptions {
            // Use our local testing server receiver address to be sure that
            // we send packets to the right place
            receiver: DEFAULT_SERVER.local_addr().unwrap(),

            // Use our local receiver address to correctly setup the tester
            sender: "0.0.0.0:0".parse().unwrap(),

            // Wait zero seconds because it is still just a test and a future
            // test is fully expected
            wait: Duration::from_secs(0),

            // Also wait zero seconds between sending packets to speed up the
            // tests
            send_periodicity: Duration::from_secs(0),

            // By default, logging is not setuped while running the tests, that
            // is, we absolutely don't cary about this value
            display_periodicity: unsafe { NonZeroUsize::new_unchecked(1) },

            // Use our default send timeout as it was specified in ArgsConfig
            send_timeout: Duration::from_secs(10),

            // Let me know if somebody knows who is it :)))
            test_name: String::from("Axl Rose"),

            // Assign a very low required packets count to prevent our lovely
            // Travis CI and your computer for a shameful breaking
            packets_count: unsafe { NonZeroUsize::new_unchecked(25) },

            // Use a very long duration to be sure that our test won't stop
            // unexpectedly
            test_duration: Duration::from_secs(9999999),

            // Use our default packet length as it was specified in ArgsConfig
            packet: &DEFAULT_PACKET,
        };
    }

    #[test]
    fn sends_all_packets() {
        // Check that our tester has successfully sent all the packets
        assert_eq!(
            execute(&DEFAULT_OPTIONS)
                .expect("An error occurred during the test")
                .packets_sent(),
            DEFAULT_OPTIONS.packets_count.get()
        );
    }

    #[test]
    fn stops_if_packets_sent() {
        let mut summary = TestSummary::new();

        // The default duration and the default packets count are too big,
        // so this line must return None
        assert_eq!(check_end_cond(&DEFAULT_OPTIONS, &summary), None);

        // Update the summary and check that all the packets were sent
        summary.update(1549335, DEFAULT_OPTIONS.packets_count.get());
        assert_eq!(
            check_end_cond(&DEFAULT_OPTIONS, &summary),
            Some(EndReason::PacketsSent)
        );
    }

    #[test]
    fn stops_if_time_passed() {
        let summary = TestSummary::new();

        let mut custom_options = DEFAULT_OPTIONS.clone();
        custom_options.test_duration = Duration::from_secs(5);

        // The default required time is not reached at this point, so this
        // line must return None
        assert_eq!(check_end_cond(&custom_options, &summary), None);

        // Sleep five seconds and check that the alloted time has passed
        thread::sleep(custom_options.test_duration);
        assert_eq!(
            check_end_cond(&custom_options, &summary),
            Some(EndReason::TimePassed)
        );
    }
}
