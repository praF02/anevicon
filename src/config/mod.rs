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

//! A module containing command-line configurations such as receivers, date-time
//! format and so on.

use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::time::Duration;

use structopt::StructOpt;

pub use endpoints::{Endpoints, EndpointsV4, EndpointsV6, ParseEndpointsError};

const DEFAULT_RANDOM_PACKET_SIZE: usize = 1024;

mod endpoints;

#[derive(Debug, Clone, Eq, PartialEq, StructOpt)]
#[structopt(
    author = "Temirkhan Myrzamadi <gymmasssorla@gmail.com>",
    about = "A high-performant UDP-based load generator, written in Rust.",
    after_help = "The `--send-file`, `--random-packet`, and `--send-message` options can be \
                  specified several times to send multiple messages to a server. But there are \
                  not guarantees about sending order because UDP is unreliable protocol.\n\nSome \
                  options accept time spans. If you want to read the specification, see \
                  <https://docs.rs/humantime/1.2.0/humantime/fn.parse_duration.html>.\n\nFor more \
                  information see <https://github.com/Gymmasssorla/anevicon>.",
    set_term_width = 90
)]
pub struct ArgsConfig {
    /// A waiting time span before a test execution used to prevent a
    /// launch of an erroneous (unwanted) test
    #[structopt(
        short = "w",
        long = "wait",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "5secs",
        parse(try_from_str = "humantime::parse_duration")
    )]
    pub wait: Duration,

    /// A maximum number of packets transmitted per a second. It's guaranteed
    /// that a number of packets sent per a second will never exceed this value
    #[structopt(
        long = "test-intensity",
        takes_value = true,
        value_name = "PACKETS",
        default_value = "1000"
    )]
    pub test_intensity: NonZeroUsize,

    #[structopt(flatten)]
    pub sockets_config: SocketsConfig,

    #[structopt(flatten)]
    pub packets_config: PacketsConfig,

    #[structopt(flatten)]
    pub logging_config: LoggingConfig,

    #[structopt(flatten)]
    pub exit_config: ExitConfig,
}

#[derive(StructOpt, Debug, Clone, Eq, PartialEq)]
pub struct SocketsConfig {
    /// Allow sockets to send packets to a broadcast address specified using the
    /// `--endpoints` option
    #[structopt(short = "b", long = "allow-broadcast", takes_value = false)]
    pub broadcast: bool,
}

#[derive(StructOpt, Debug, Clone, Eq, PartialEq)]
pub struct PayloadConfig {
    /// Repeatedly send a random-generated packet with a specified bytes length.
    /// The default is 1024
    #[structopt(
        short = "l",
        long = "random-packet",
        takes_value = true,
        value_name = "POSITIVE-INTEGER"
    )]
    pub random_packets: Vec<NonZeroUsize>,

    /// Interpret the specified file content as a single packet and repeatedly
    /// send it to each receiver
    #[structopt(
        short = "f",
        long = "send-file",
        takes_value = true,
        value_name = "FILENAME"
    )]
    pub send_files: Vec<PathBuf>,

    /// Interpret the specified UTF-8 encoded text message as a single packet
    /// and repeatedly send it to each receiver
    #[structopt(
        short = "m",
        long = "send-message",
        takes_value = true,
        value_name = "STRING"
    )]
    pub send_messages: Vec<String>,
}

#[derive(StructOpt, Debug, Clone, Eq, PartialEq)]
pub struct PacketsConfig {
    /// Two endpoints specified as `<SENDER-ADDRESS>&<RECEIVER-ADDRESS>`, where
    /// address is a string of a `<IP>:<PORT>` format.
    ///
    /// A sender and a receiver can be absolutely any valid IPv4/IPv6 addresses
    /// (which is used to send spoofed packets sometimes).
    ///
    /// This option can be specified several times to identically test multiple
    /// web servers in concurrent mode.
    #[structopt(
        short = "e",
        long = "endpoints",
        takes_value = true,
        value_name = "SENDER&RECEIVER",
        multiple = true,
        number_of_values = 1,
        required = true
    )]
    pub endpoints: Vec<Endpoints>,

    /// Specifies the IP_TTL value for all future sockets. Usually this value
    /// equals a number of routers that a packet can go through
    #[structopt(
        long = "ip-ttl",
        takes_value = true,
        default_value = "64",
        value_name = "UNSIGNED-INTEGER"
    )]
    pub ip_ttl: u8,

    #[structopt(flatten)]
    pub payload_config: PayloadConfig,
}

#[derive(StructOpt, Debug, Clone, Eq, PartialEq)]
pub struct LoggingConfig {
    /// Enable one of the possible verbosity levels. The zero level doesn't
    /// print anything, and the last level prints everything.
    ///
    /// Note that specifying the 4 and 5 verbosity levels might decrease
    /// performance, do it only for debugging.
    #[structopt(
        short = "v",
        long = "verbosity",
        takes_value = true,
        value_name = "LEVEL",
        default_value = "3",
        raw(possible_values = r#"&["0", "1", "2", "3", "4", "5"]"#)
    )]
    pub verbosity: i32,

    /// A format for displaying local date and time in log messages. Type `man
    /// strftime` to see the format specification
    #[structopt(
        long = "date-time-format",
        takes_value = true,
        value_name = "STRING",
        default_value = "%X",
        raw(validator = "validate_date_time_format")
    )]
    pub date_time_format: String,
}

#[derive(StructOpt, Debug, Clone, Eq, PartialEq)]
pub struct ExitConfig {
    /// A count of packets for sending. When this limit is reached, then the
    /// program will immediately stop its execution
    #[structopt(
        short = "p",
        long = "packets-count",
        takes_value = true,
        value_name = "POSITIVE-INTEGER",
        default_value = "18446744073709551615"
    )]
    pub packets_count: NonZeroUsize,

    /// A whole test duration. When this limit is reached, then the program will
    /// immediately stop its execution
    #[structopt(
        short = "d",
        long = "test-duration",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "64years 64hours 64secs",
        parse(try_from_str = "humantime::parse_duration")
    )]
    pub test_duration: Duration,
}

impl ArgsConfig {
    /// Use it to setup the current structure. It does special additional stuff
    /// unlike the typical `StructOpt::from_args()`.
    pub fn setup() -> ArgsConfig {
        let mut matches = ArgsConfig::from_args();

        // If a user hasn't specified both a file, a text message, and a packet length,
        // then set the default packet length
        if matches.packets_config.payload_config.send_files.is_empty()
            && matches
                .packets_config
                .payload_config
                .random_packets
                .is_empty()
            && matches
                .packets_config
                .payload_config
                .send_messages
                .is_empty()
        {
            matches.packets_config.payload_config.random_packets =
                vec![NonZeroUsize::new(DEFAULT_RANDOM_PACKET_SIZE).unwrap()];
        }

        matches
    }
}

fn validate_date_time_format(format: String) -> Result<(), String> {
    // If this call succeeds, `format` is correct
    time::strftime(&format, &time::now())
        .map(|_| ())
        .map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Check that ordinary formats are passed correctly
    #[test]
    fn validates_valid_time_format() {
        let check = |format| {
            assert_eq!(
                validate_date_time_format(String::from(format)),
                Ok(()),
                "Parses valid time incorrectly"
            )
        };

        check("%x %X %e");
        check("%H %a %G");
        check("something");
        check("flower %d");
    }

    // Invalid formats must produce the invalid format error
    #[test]
    fn validates_invalid_time_format() {
        let check = |format| {
            assert!(
                validate_date_time_format(String::from(format)).is_err(),
                "Parses invalid time correctly"
            )
        };

        check("%_=-%vbg=");
        check("yufb%44htv");
        check("sf%jhei9%990");
    }
}
