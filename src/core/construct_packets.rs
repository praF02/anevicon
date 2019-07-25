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

use etherparse::PacketBuilder;

use crate::config::{Endpoints, EndpointsV4, EndpointsV6};

pub fn ip_udp_packet(endpoints: &Endpoints, payload: &[u8], time_to_live: u8) -> Vec<u8> {
    match endpoints {
        Endpoints::V4(endpoints_v4) => ipv4_udp_packet(endpoints_v4, payload, time_to_live),
        Endpoints::V6(endpoints_v6) => ipv6_udp_packet(endpoints_v6, payload, time_to_live),
    }
}

pub fn ipv4_udp_packet(endpoints: &EndpointsV4, payload: &[u8], time_to_live: u8) -> Vec<u8> {
    let builder = PacketBuilder::ipv4(
        endpoints.sender.ip().octets(),
        endpoints.receiver.ip().octets(),
        time_to_live,
    )
    .udp(endpoints.sender.port(), endpoints.receiver.port());
    let mut serialized = Vec::<u8>::with_capacity(builder.size(payload.len()));
    builder
        .write(&mut serialized, payload)
        .expect("Failed to serialize a UDP/IPv4 packet into Vec<u8>");
    serialized
}

pub fn ipv6_udp_packet(endpoints: &EndpointsV6, payload: &[u8], time_to_live: u8) -> Vec<u8> {
    let builder = PacketBuilder::ipv6(
        endpoints.sender.ip().octets(),
        endpoints.receiver.ip().octets(),
        time_to_live,
    )
    .udp(endpoints.sender.port(), endpoints.receiver.port());
    let mut serialized = Vec::<u8>::with_capacity(builder.size(payload.len()));
    builder
        .write(&mut serialized, payload)
        .expect("Failed to serialize a UDP/IPv6 packet into Vec<u8>");
    serialized
}
