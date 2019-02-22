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
use std::sync::{Arc, RwLock};
use std::thread;

use super::config::ArgsConfig;
use super::summary::TestSummary;

use log::info;

#[derive(Debug)]
pub struct Tester {
    socket: UdpSocket,
    packet: Arc<RwLock<Vec<u8>>>,
    args_config: Arc<RwLock<ArgsConfig>>,
}

impl Tester {
    pub fn new(
        args_config: Arc<RwLock<ArgsConfig>>,
        packet: Arc<RwLock<Vec<u8>>>,
    ) -> io::Result<Tester> {
        let locked_config = args_config.read().unwrap();

        // Complete any necessary stuff with the specified socket
        let socket = UdpSocket::bind(locked_config.sender)?;
        socket.connect(locked_config.receiver)?;
        socket.set_write_timeout(locked_config.send_timeout)?;

        std::mem::drop(locked_config);

        Ok(Tester {
            socket,
            packet,
            args_config,
        })
    }

    pub fn execute(&self, summary: Arc<RwLock<TestSummary>>) -> io::Result<()> {
        let locked_config = self.args_config.read().unwrap();
        let locked_packet = self.packet.read().unwrap();

        loop {
            for _ in 0..locked_config.display_periodicity.get() {
                summary
                    .write()
                    .unwrap()
                    .update(self.socket.send(&locked_packet)?, 1);

                if self.check_end_cond(&summary.read().unwrap()) {
                    return Ok(());
                }

                thread::sleep(locked_config.send_periodicity);
            }

            info!("Running with {}.", summary.read().unwrap());
        }
    }

    fn check_end_cond(&self, summary: &TestSummary) -> bool {
        if summary.time_passed() >= self.args_config.read().unwrap().duration {
            info!(
                "The test is stopping the packet sending because \
                 the allotted time has passed. The total result is: {}.",
                summary
            );
            return true;
        }
        if summary.packets_sent() == self.args_config.read().unwrap().packets.get() {
            info!(
                "The test is stopping the packet sending because \
                 all the required packets were sent. The total result is: {}.",
                summary
            );
            return true;
        }

        false
    }
}
