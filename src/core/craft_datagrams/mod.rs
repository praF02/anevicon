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

use failure::Fallible;

pub use craft_payload::CraftPayloadError;

use crate::config::PacketsConfig;

mod craft_packets;
mod craft_payload;

/// Constructs raw UDP/IP datagrams from `PacketsConfig`.
///
/// # Returns
/// This function returns a vector of iterators that return UDP/IP datagrams.
///
/// Each datagram consists of IP header + UDP header + user's payload, and the
/// resulting size of each iterator is equal to a total number of occurrences of
/// `--random-packet`, `--send-message`, and `--send-file` options.
pub fn craft_all(config: &PacketsConfig) -> Fallible<Vec<impl Iterator<Item = Vec<u8>>>> {
    let payload = craft_payload::craft_all(&config.payload_config)?;

    let mut result = Vec::with_capacity(config.endpoints.len());
    for next_endpoints in &config.endpoints {
        let mut datagrams = Vec::with_capacity(payload.len());
        for payload_portion in &payload {
            datagrams.push(craft_packets::ip_udp_packet(
                next_endpoints,
                payload_portion,
                config.ip_ttl,
            ));
        }

        result.push(datagrams.into_iter());
    }

    Ok(result)
}
