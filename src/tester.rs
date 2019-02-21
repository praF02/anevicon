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

use super::config::ArgsConfig;
use super::summary::TestSummary;

use log::info;

#[derive(Debug)]
pub struct Tester<'a> {
    socket: UdpSocket,
    packet: &'a [u8],
    args_config: &'a ArgsConfig,
}

impl<'a> Tester<'a> {
    pub fn new(args_config: &'a ArgsConfig, packet: &'a [u8]) -> io::Result<Tester<'a>> {
        // Complete any necessary stuff with the specified socket
        let socket = UdpSocket::bind(args_config.sender)?;
        socket.connect(args_config.receiver)?;
        socket.set_write_timeout(args_config.send_timeout)?;

        Ok(Tester {
            socket,
            packet,
            args_config,
        })
    }

    pub fn execute(&self) -> io::Result<TestSummary> {
        info!("The program is starting to test with {}.", self.args_config);

        thread::sleep(self.args_config.wait);
        let mut summary = TestSummary::new();

        loop {
            for _ in 0..self.args_config.display_periodicity.get() {
                summary.update(self.socket.send(self.packet)?, 1);

                if self.check_end_cond(&summary) {
                    return Ok(summary);
                }

                thread::sleep(self.args_config.send_periodicity);
            }

            info!("The test is running with {}.", summary);
        }
    }

    fn check_end_cond(&self, summary: &TestSummary) -> bool {
        if summary.time_passed() >= self.args_config.duration {
            info!(
                "The program is stopping the packet sending because \
                 the allotted time has passed. The total result is: {}.",
                summary
            );
            return true;
        }
        if summary.packets_sent() == self.args_config.packets.get() {
            info!(
                "The program is stopping the packet sending because \
                 all the required packets were sent. The total result is: {}.",
                summary
            );
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::num::NonZeroUsize;

    use lazy_static::lazy_static;
    use structopt::StructOpt;

    use super::super::helpers::random_packet;

    lazy_static! {
        static ref DEFAULT_PACKET: Vec<u8> = random_packet(NonZeroUsize::new(65000).unwrap());
        static ref DEFAULT_SERVER: UdpSocket = UdpSocket::bind("0.0.0.0:0")
            .expect("Cannot setup the testing server with the address 0.0.0.0:0");
    }

    fn default_config() -> ArgsConfig {
        // The first command-line argument doesn't have any meaning for CLAP
        ArgsConfig::from_iter_safe(vec![
            "anevicon",
            "--receiver",
            &DEFAULT_SERVER.local_addr().unwrap().to_string(),
        ])
        .expect("The command-line arguments are incorrectly specified")
    }

    fn setup_tester(args_config: &ArgsConfig) -> Tester {
        Tester::new(args_config, &DEFAULT_PACKET).expect("Cannot setup the tester")
    }

    #[test]
    fn end_conditions_work() {
        let config = default_config();
        let tester = setup_tester(&config);
        let mut summary = TestSummary::new();

        // The default duration and the default packets count are too big,
        // so this line must return false
        assert_eq!(tester.check_end_cond(&summary), false);

        // Update the summary and check that all the packets was sent
        summary.update(1549335, std::usize::MAX);
        assert_eq!(tester.check_end_cond(&summary), true);
    }

    #[test]
    fn sends_all_packets() {
        // Assign a very low required packets count to prevent our
        // lovely Travis CI and your computer for a shameful breaking
        let packets = NonZeroUsize::new(25).unwrap();

        let mut config = default_config();
        config.packets = packets;

        // Check that our tester has successfully sent all the packets
        assert_eq!(
            setup_tester(&config)
                .execute()
                .expect("An error occurred during the test")
                .packets_sent(),
            packets.get()
        );
    }

    #[test]
    fn correctly_constructs_tester() {
        // Specify any valid-formatted addresses, this isn't essential
        let mut config = default_config();
        config.sender = "127.0.0.1:56978".parse().unwrap();

        // Setup our tester with the previous receiver address
        let tester = setup_tester(&config);

        assert_eq!(tester.args_config, &config);
        assert_eq!(
            tester
                .socket
                .write_timeout()
                .expect("A write timeout is unavailable"),
            config.send_timeout
        );
        assert_eq!(
            tester
                .socket
                .local_addr()
                .expect("A local address is unavailable"),
            config.sender
        );
        assert_eq!(
            NonZeroUsize::new(tester.packet.len()).expect("Zero packet length"),
            config.length
        );
    }
}
