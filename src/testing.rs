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

use log::info;

use super::config::ArgsConfig;
use super::summary::TestSummary;

pub fn execute(args_config: &ArgsConfig, packet: &[u8]) -> io::Result<TestSummary> {
    // Complete any necessary stuff with the specified socket
    let socket = UdpSocket::bind(args_config.sender)?;
    socket.connect(args_config.receiver)?;
    socket.set_write_timeout(args_config.send_timeout)?;

    info!("The test is starting with >>> {}.", args_config);
    thread::sleep(args_config.wait);
    let mut summary = TestSummary::new();

    loop {
        for _ in 0..args_config.display_periodicity.get() {
            summary.update(socket.send(packet)?, 1);

            if check_end_cond(args_config, &summary) {
                return Ok(summary);
            }

            thread::sleep(args_config.send_periodicity);
        }

        info!("Running with >>> {}.", summary);
    }
}

fn check_end_cond(args_config: &ArgsConfig, summary: &TestSummary) -> bool {
    if summary.time_passed() >= args_config.duration {
        info!(
            "The test is stopping because \
             the allotted time has passed. The total result is >>> {}.",
            summary
        );
        return true;
    }
    if summary.packets_sent() == args_config.packets.get() {
        info!(
            "The test is stopping because \
             all the required packets were sent. The total result is >>> {}.",
            summary
        );
        return true;
    }

    false
}
