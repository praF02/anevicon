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

use pnet::packet::udp::UdpPacket;

use construct_payload::construct_payload;
pub use construct_payload::ConstructPayloadError;

use crate::config::PacketsConfig;

mod construct_payload;

/// Returns a new instance of `UdpPacketsIterator` (an iterator of UDP packets
/// each constructed from specified user's payload, i.e by `--random-packet`,
/// `--send-message`, `--send-file`).
pub fn packets_iterator(
    config: &PacketsConfig,
) -> Result<UdpPacketsIterator, ConstructPayloadError> {
    Ok(UdpPacketsIterator {
        user_packets: construct_payload(config)?,
    })
}

pub struct UdpPacketsIterator {
    user_packets: Vec<Vec<u8>>,
}

//impl Iterator for UdpPacketsIterator {
//   type Item = UdpPacket;

//   fn next(&mut self) -> Option<Self::Item> {}
//}
