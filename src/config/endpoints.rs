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

//! The structures representing user-specified communication endpoints.

use std::net::{AddrParseError, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct EndpointsV4 {
    pub sender: SocketAddrV4,
    pub receiver: SocketAddrV4,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct EndpointsV6 {
    pub sender: SocketAddrV6,
    pub receiver: SocketAddrV6,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum Endpoints {
    V4(EndpointsV4),
    V6(EndpointsV6),
}

#[derive(Debug, Clone, Eq, PartialEq, Fail)]
pub enum ParseEndpointsError {
    #[fail(
        display = "Endpoints must be specified as <SENDER-ADDRESS>&<RECEIVER-ADDRESS>, where \
                   address is defined as <IP>:<PORT>"
    )]
    InvalidFormat,

    #[fail(display = "{}", _0)]
    InvalidAddressFormat(#[fail(cause)] AddrParseError),

    #[fail(
        display = "Endpoints must be specified as <SENDER-ADDRESS>&<RECEIVER-ADDRESS>, where \
                   address is defined as <IP>:<PORT>"
    )]
    DifferentIpVersions,
}

impl Endpoints {
    pub fn sender(&self) -> SocketAddr {
        match self {
            Self::V4(v4) => SocketAddr::V4(v4.sender),
            Self::V6(v6) => SocketAddr::V6(v6.sender),
        }
    }

    pub fn receiver(&self) -> SocketAddr {
        match self {
            Self::V4(v4) => SocketAddr::V4(v4.receiver),
            Self::V6(v6) => SocketAddr::V6(v6.receiver),
        }
    }
}

impl FromStr for Endpoints {
    type Err = ParseEndpointsError;

    fn from_str(format: &str) -> Result<Self, ParseEndpointsError> {
        let addresses = format.split('&').collect::<Vec<&str>>();
        if addresses.len() != 2 {
            return Err(ParseEndpointsError::InvalidFormat);
        }

        let sender = addresses[0]
            .parse::<SocketAddr>()
            .map_err(ParseEndpointsError::InvalidAddressFormat)?;
        let receiver = addresses[1]
            .parse::<SocketAddr>()
            .map_err(ParseEndpointsError::InvalidAddressFormat)?;

        match sender {
            SocketAddr::V4(sender_v4) => match receiver {
                SocketAddr::V4(receiver_v4) => Ok(Endpoints::V4(EndpointsV4 {
                    sender: sender_v4,
                    receiver: receiver_v4,
                })),
                _ => Err(ParseEndpointsError::DifferentIpVersions),
            },
            SocketAddr::V6(sender_v6) => match receiver {
                SocketAddr::V6(receiver_v6) => Ok(Endpoints::V6(EndpointsV6 {
                    sender: sender_v6,
                    receiver: receiver_v6,
                })),
                _ => Err(ParseEndpointsError::DifferentIpVersions),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use super::*;

    #[test]
    fn check_endpoints_v4() {
        let v4 = EndpointsV4 {
            sender: SocketAddrV4::new(Ipv4Addr::new(32, 43, 35, 211), 1921),
            receiver: SocketAddrV4::new(Ipv4Addr::new(63, 222, 66, 14), 1939),
        };
        let endpoints = Endpoints::V4(v4);

        assert_eq!(endpoints.sender(), SocketAddr::V4(v4.sender));
        assert_eq!(endpoints.receiver(), SocketAddr::V4(v4.receiver));
    }

    #[test]
    fn check_endpoints_v6() {
        let v6 = EndpointsV6 {
            sender: SocketAddrV6::new(Ipv6Addr::new(32, 43, 35, 211, 53, 25, 9, 213), 1921, 0, 0),
            receiver: SocketAddrV6::new(
                Ipv6Addr::new(63, 222, 66, 14, 66, 24, 111, 20),
                1939,
                0,
                0,
            ),
        };
        let endpoints = Endpoints::V6(v6);

        assert_eq!(endpoints.sender(), SocketAddr::V6(v6.sender));
        assert_eq!(endpoints.receiver(), SocketAddr::V6(v6.receiver));
    }

    #[test]
    fn parses_valid_v4() {
        assert_eq!(
            Endpoints::from_str("233.43.24.53:34&29.32.45.111:9191"),
            Ok(Endpoints::V4(EndpointsV4 {
                sender: SocketAddrV4::from_str("233.43.24.53:34").unwrap(),
                receiver: SocketAddrV4::from_str("29.32.45.111:9191").unwrap(),
            }))
        );
    }

    #[test]
    fn parses_valid_v6() {
        assert_eq!(
            Endpoints::from_str(
                "[2001:db8:85a3:0:0:8a2e:370:7334]:18281&[2001:0db8:85a3:0000:0000:8a2e:0370:\
                 7334]:9191"
            ),
            Ok(Endpoints::V6(EndpointsV6 {
                sender: SocketAddrV6::from_str("[2001:db8:85a3:0:0:8a2e:370:7334]:18281").unwrap(),
                receiver: SocketAddrV6::from_str("[2001:0db8:85a3:0000:0000:8a2e:0370:7334]:9191")
                    .unwrap(),
            }))
        );
    }

    #[test]
    fn check_invalid_versions() {
        assert_eq!(
            Endpoints::from_str("233.43.24.53:34&[2001:0db8:85a3:0000:0000:8a2e:0370:7334]:9191"),
            Err(ParseEndpointsError::DifferentIpVersions)
        );
    }

    #[test]
    fn check_invalid_format() {
        assert_eq!(
            Endpoints::from_str("233.43.24.1:8181---92.52.113.18:9191"),
            Err(ParseEndpointsError::InvalidFormat)
        );
        assert_eq!(
            Endpoints::from_str("233.43.24.1:8181^92.52.113.18:9191"),
            Err(ParseEndpointsError::InvalidFormat)
        );
    }

    #[test]
    fn check_invalid_address_format() {
        let check = |format| {
            if let Err(ParseEndpointsError::InvalidAddressFormat(_)) = Endpoints::from_str(format) {
                // Good
            } else {
                panic!("ParseEndpointsError::InvalidAddressFormat must be returned");
            }
        };

        check("233.43.24.53:34&92.52.113:9191");
        check("233.43.24:34&92.52.113.43:9191");
        check("hello:51919&world:21342");
    }
}
