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

use std::io::IoSlice;

use select_interface::select_interface;
pub use select_interface::SelectInterfaceError;
pub use udp_sender::UdpSender;

use crate::config::TesterConfig;

mod select_interface;
mod sendmmsg;
mod udp_sender;

/// A type alias that represents a portion to be sent. The first item is a
/// number of bytes sent, and the second item is a packet to be sent.
pub type Portion<'a> = (usize, IoSlice<'a>);

pub fn create_udp_senders(config: &TesterConfig) -> Result<Vec<UdpSender>, SelectInterfaceError> {
    let interface_address = if config.sockets_config.select_if {
        Some(select_interface()?)
    } else {
        None
    };

    let mut senders = Vec::with_capacity(config.sockets_config.receivers.len());

    Ok(senders)
}
