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

use std::convert::TryInto;
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

use pnet_packet::ip::IpNextHeaderProtocols;
use pnet_packet::ipv4::{Ipv4Packet, MutableIpv4Packet};
use pnet_packet::ipv6::{Ipv6Packet, MutableIpv6Packet};
use pnet_packet::udp::MutableUdpPacket;
use pnet_packet::{Packet, PacketSize};
use std::hint::unreachable_unchecked;

const IPV4_HEADER_LENGTH: usize = 20;
const IPV6_HEADER_LENGTH: usize = 40;
const UDP_HEADER_LENGTH: usize = 8;

pub fn construct_ip_udp_packet(
    source: &SocketAddr,
    dest: &SocketAddr,
    payload: &[u8],
    ip_ttl: u8,
) -> Vec<u8> {
    match source {
        SocketAddr::V4(ipv4_source) => match dest {
            SocketAddr::V4(ipv4_dest) => {
                construct_ipv4_udp_packet(ipv4_source, ipv4_dest, payload, ip_ttl)
                    .packet()
                    .to_owned()
            }
            _ => unsafe { unreachable_unchecked() },
        },
        SocketAddr::V6(ipv6_source) => match dest {
            SocketAddr::V6(ipv6_dest) => {
                construct_ipv6_udp_packet(ipv6_source, ipv6_dest, payload, ip_ttl)
                    .packet()
                    .to_owned()
            }
            _ => unsafe { unreachable_unchecked() },
        },
    }
}

pub fn construct_ipv4_udp_packet(
    source: &SocketAddrV4,
    dest: &SocketAddrV4,
    payload: &[u8],
    ip_ttl: u8,
) -> Ipv4Packet<'static> {
    let mut udp_packet = construct_udp_packet_without_checksum(
        SocketAddr::V4(*source),
        SocketAddr::V4(*dest),
        payload,
    );
    udp_packet.set_checksum(pnet_packet::udp::ipv4_checksum_adv(
        &udp_packet.to_immutable(),
        payload,
        &source.ip(),
        &dest.ip(),
    ));

    let total_ipv4_packet_length = IPV4_HEADER_LENGTH + udp_packet.packet_size();

    let mut ipv4_packet = MutableIpv4Packet::owned(vec![0u8; total_ipv4_packet_length]).unwrap();
    ipv4_packet.set_version(4);
    ipv4_packet.set_header_length((IPV4_HEADER_LENGTH / 4).try_into().unwrap());
    ipv4_packet.set_dscp(0);
    ipv4_packet.set_ecn(0);
    ipv4_packet.set_total_length(total_ipv4_packet_length.try_into().unwrap());
    // Linux will care about the identification field
    ipv4_packet.set_identification(0);
    ipv4_packet.set_flags(0);
    ipv4_packet.set_fragment_offset(0);
    ipv4_packet.set_ttl(ip_ttl);
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Udp);
    ipv4_packet.set_source(*source.ip());
    ipv4_packet.set_destination(*dest.ip());
    ipv4_packet.set_payload(udp_packet.packet());
    ipv4_packet.set_checksum(0);
    ipv4_packet.set_checksum(pnet_packet::ipv4::checksum(&ipv4_packet.to_immutable()));

    ipv4_packet.consume_to_immutable()
}

pub fn construct_ipv6_udp_packet(
    source: &SocketAddrV6,
    dest: &SocketAddrV6,
    payload: &[u8],
    ip_ttl: u8,
) -> Ipv6Packet<'static> {
    let mut udp_packet = construct_udp_packet_without_checksum(
        SocketAddr::V6(*source),
        SocketAddr::V6(*dest),
        payload,
    );
    udp_packet.set_checksum(pnet_packet::udp::ipv6_checksum_adv(
        &udp_packet.to_immutable(),
        payload,
        &source.ip(),
        &dest.ip(),
    ));

    let total_ipv6_packet_length = IPV6_HEADER_LENGTH + udp_packet.packet_size();

    let mut ipv6_packet = MutableIpv6Packet::owned(vec![0u8; total_ipv6_packet_length]).unwrap();
    ipv6_packet.set_version(6);
    ipv6_packet.set_traffic_class(0);
    ipv6_packet.set_flow_label(0);
    ipv6_packet.set_hop_limit(ip_ttl);
    ipv6_packet.set_next_header(IpNextHeaderProtocols::Udp);
    ipv6_packet.set_source(*source.ip());
    ipv6_packet.set_destination(*dest.ip());
    ipv6_packet.set_payload(udp_packet.packet());
    ipv6_packet.set_payload_length(udp_packet.packet_size().try_into().unwrap());

    ipv6_packet.consume_to_immutable()
}

fn construct_udp_packet_without_checksum(
    source: SocketAddr,
    dest: SocketAddr,
    payload: &[u8],
) -> MutableUdpPacket<'static> {
    let udp_packet_length = UDP_HEADER_LENGTH + payload.len();

    let mut udp_packet = MutableUdpPacket::owned(vec![0u8; udp_packet_length]).unwrap();
    udp_packet.set_source(source.port());
    udp_packet.set_destination(dest.port());
    udp_packet.set_length(udp_packet_length.try_into().unwrap());
    udp_packet.set_payload(payload);
    udp_packet.set_checksum(0);

    udp_packet
}
