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

use std::sync::{Arc, RwLock};
use std::thread;

use log::{error, info};
use structopt::StructOpt;

use config::ArgsConfig;
use helpers::construct_packet;
use logging::setup_logging;
use summary::TestSummary;
use tester::Tester;

mod config;
mod helpers;
mod logging;
mod summary;
mod tester;

fn main() {
    let config = ArgsConfig::from_args();
    setup_logging(config.debug);

    info!("The test is starting with {}.", config);
    thread::sleep(config.wait);

    setup_threads(config);
}

fn setup_threads(args_config: ArgsConfig) {
    let threads_count = args_config.threads.get();
    let mut threads = Vec::with_capacity(threads_count);

    let packet = match construct_packet(&args_config) {
        Err(error) => {
            error!("Cannot construct a packet >>> {}!", error);
            std::process::exit(1);
        }
        Ok(packet) => Arc::new(RwLock::new(packet)),
    };

    let lock_config = Arc::new(RwLock::new(args_config));
    let summary = Arc::new(RwLock::new(TestSummary::new()));

    for _ in 0..threads_count {
        let lock_config = lock_config.clone();
        let packet = packet.clone();
        let summary = summary.clone();

        threads.push(thread::spawn(move || {
            let tester = match Tester::new(lock_config, packet) {
                Err(error) => {
                    error!("Cannot setup the tester >>> {}!", error);
                    std::process::exit(1);
                }
                Ok(tester) => tester,
            };

            if let Err(error) = tester.execute(summary) {
                error!("An error occurred during the test >>> {}!", error);
                std::process::exit(1);
            }
        }));
    }

    threads
        .into_iter()
        .for_each(|thread| thread.join().expect("The thread has panicked"));
}
