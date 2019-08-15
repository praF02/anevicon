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

//! Some functions to construct raw UDP/IP packets (headers + data).

use etherparse::PacketBuilder;

use crate::config::{Endpoints, EndpointsV4, EndpointsV6};

pub fn ip_udp_packet(endpoints: &Endpoints, payload: &[u8], time_to_live: u8) -> Vec<u8> {
    match endpoints {
        Endpoints::V4(endpoints_v4) => ipv4_udp_packet(endpoints_v4, payload, time_to_live),
        Endpoints::V6(endpoints_v6) => ipv6_udp_packet(endpoints_v6, payload, time_to_live),
    }
}

fn ipv4_udp_packet(endpoints: &EndpointsV4, payload: &[u8], time_to_live: u8) -> Vec<u8> {
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

fn ipv6_udp_packet(endpoints: &EndpointsV6, payload: &[u8], time_to_live: u8) -> Vec<u8> {
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

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

    use super::*;

    #[test]
    fn test_construct_ipv4_first() {
        let packet = ipv4_udp_packet(
            &EndpointsV4 {
                sender: SocketAddrV4::new(Ipv4Addr::BROADCAST, 3838),
                receiver: SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 17172),
            },
            b"I wanna hold you in my arms, yeah",
            9,
        );

        assert_eq!(
            packet,
            vec![
                69, 0, 0, 61, 0, 0, 64, 0, 9, 17, 113, 177, 255, 255, 255, 255, 0, 0, 0, 0, 14,
                254, 67, 20, 0, 41, 200, 222, 73, 32, 119, 97, 110, 110, 97, 32, 104, 111, 108,
                100, 32, 121, 111, 117, 32, 105, 110, 32, 109, 121, 32, 97, 114, 109, 115, 44, 32,
                121, 101, 97, 104,
            ]
        );
    }

    #[test]
    fn test_construct_ipv4_second() {
        let packet = ipv4_udp_packet(
            &EndpointsV4 {
                sender: SocketAddrV4::new(Ipv4Addr::new(53, 76, 0, 112), 3838),
                receiver: SocketAddrV4::new(Ipv4Addr::new(84, 10, 8, 81), 17172),
            },
            b"Havin' a nervous breakdown, a-drive me insane, yeah",
            134,
        );

        assert_eq!(
            packet,
            vec![
                69, 0, 0, 79, 0, 0, 64, 0, 134, 17, 98, 135, 53, 76, 0, 112, 84, 10, 8, 81, 14,
                254, 67, 20, 0, 59, 27, 25, 72, 97, 118, 105, 110, 39, 32, 97, 32, 110, 101, 114,
                118, 111, 117, 115, 32, 98, 114, 101, 97, 107, 100, 111, 119, 110, 44, 32, 97, 45,
                100, 114, 105, 118, 101, 32, 109, 101, 32, 105, 110, 115, 97, 110, 101, 44, 32,
                121, 101, 97, 104,
            ]
        );
    }

    #[test]
    fn test_construct_ipv6_first() {
        let packet = ipv6_udp_packet(
            &EndpointsV6 {
                sender: SocketAddrV6::new(Ipv6Addr::LOCALHOST, 18273, 0, 0),
                receiver: SocketAddrV6::new(Ipv6Addr::LOCALHOST, 9492, 0, 0),
            },
            b"Communication breakdown, it's always the same",
            61,
        );

        assert_eq!(
            packet,
            vec![
                96, 0, 0, 0, 0, 53, 17, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 71, 97, 37, 20, 0, 53, 239, 209, 67, 111,
                109, 109, 117, 110, 105, 99, 97, 116, 105, 111, 110, 32, 98, 114, 101, 97, 107,
                100, 111, 119, 110, 44, 32, 105, 116, 39, 115, 32, 97, 108, 119, 97, 121, 115, 32,
                116, 104, 101, 32, 115, 97, 109, 101,
            ]
        );
    }

    #[test]
    fn test_construct_ipv6_second() {
        let packet = ipv6_udp_packet(
            &EndpointsV6 {
                sender: SocketAddrV6::new(Ipv6Addr::new(64, 0, 0, 23, 111, 213, 4, 44), 7475, 0, 0),
                receiver: SocketAddrV6::new(
                    Ipv6Addr::new(244, 1, 44, 63, 92, 18, 91, 5),
                    16392,
                    0,
                    0,
                ),
            },
            b"I wanna hold you in my arms, yeah",
            250,
        );

        assert_eq!(
            packet,
            vec![
                96, 0, 0, 0, 0, 41, 17, 250, 0, 64, 0, 0, 0, 0, 0, 23, 0, 111, 0, 213, 0, 4, 0, 44,
                0, 244, 0, 1, 0, 44, 0, 63, 0, 92, 0, 18, 0, 91, 0, 5, 29, 51, 64, 8, 0, 41, 185,
                188, 73, 32, 119, 97, 110, 110, 97, 32, 104, 111, 108, 100, 32, 121, 111, 117, 32,
                105, 110, 32, 109, 121, 32, 97, 114, 109, 115, 44, 32, 121, 101, 97, 104,
            ]
        );
    }
}
