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

use log::error;

use config::ArgsConfig;
use helpers::construct_packet;
use logging::setup_logging;
use testing::execute;

mod config;
mod helpers;
mod logging;
mod summary;
mod testing;

fn main() {
    let config = ArgsConfig::setup();

    if let Err(error) = setup_logging(config.debug, &config.output) {
        logging::raw_fatal(format_args!(
            "Opening the output file failed >>> {}!",
            error
        ));
    }

    let packet = match construct_packet(&config) {
        Err(error) => {
            error!("Constructing the packet failed >>> {}!", error);
            std::process::exit(1);
        }
        Ok(packet) => packet,
    };

    if let Err(error) = execute(&config, &packet) {
        error!("Testing the server failed >>> {}!", error);
        std::process::exit(1);
    }
}
