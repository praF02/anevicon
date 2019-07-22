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
use std::hint::unreachable_unchecked;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

pub fn ip_udp_packet(
    source: &SocketAddr,
    dest: &SocketAddr,
    payload: &[u8],
    time_to_live: u8,
) -> Vec<u8> {
    match source {
        SocketAddr::V4(ipv4_source) => match dest {
            SocketAddr::V4(ipv4_dest) => {
                ipv4_udp_packet(ipv4_source, ipv4_dest, payload, time_to_live)
            }
            _ => unsafe { unreachable_unchecked() },
        },
        SocketAddr::V6(ipv6_source) => match dest {
            SocketAddr::V6(ipv6_dest) => {
                ipv6_udp_packet(ipv6_source, ipv6_dest, payload, time_to_live)
            }
            _ => unsafe { unreachable_unchecked() },
        },
    }
}

pub fn ipv4_udp_packet(
    source: &SocketAddrV4,
    dest: &SocketAddrV4,
    payload: &[u8],
    time_to_live: u8,
) -> Vec<u8> {
    let builder = PacketBuilder::ipv4(source.ip().octets(), dest.ip().octets(), time_to_live)
        .udp(source.port(), dest.port());
    let mut serialized = Vec::<u8>::with_capacity(builder.size(payload.len()));
    builder
        .write(&mut serialized, payload)
        .expect("Failed to serialize a UDP/IPv4 packet into Vec<u8>");
    serialized
}

pub fn ipv6_udp_packet(
    source: &SocketAddrV6,
    dest: &SocketAddrV6,
    payload: &[u8],
    time_to_live: u8,
) -> Vec<u8> {
    let builder = PacketBuilder::ipv6(source.ip().octets(), dest.ip().octets(), time_to_live)
        .udp(source.port(), dest.port());
    let mut serialized = Vec::<u8>::with_capacity(builder.size(payload.len()));
    builder
        .write(&mut serialized, payload)
        .expect("Failed to serialize a UDP/IPv4 packet into Vec<u8>");
    serialized
}
