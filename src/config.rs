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

use std::net::SocketAddr;
use std::num::{NonZeroUsize, ParseIntError};
use std::path::PathBuf;
use std::time::Duration;

use humantime::parse_duration;
use structopt::StructOpt;
use time::ParseError;

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
        parse(try_from_str = "parse_duration")
    )]
    pub wait: Duration,

    #[structopt(flatten)]
    pub logging_config: LoggingConfig,

    #[structopt(flatten)]
    pub tester_config: TesterConfig,

    #[structopt(flatten)]
    pub packets_config: PacketsConfig,

    #[structopt(flatten)]
    pub sockets_config: SocketsConfig,
}

#[derive(StructOpt, Debug, Clone, Eq, PartialEq)]
pub struct TesterConfig {
    /// A time interval between sendmmsg system calls. This option can be used
    /// to modify test intensity
    #[structopt(
        long = "send-periodicity",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "0secs",
        parse(try_from_str = "parse_duration")
    )]
    pub send_periodicity: Duration,

    /// A count of packets which the program will send using only one system
    /// call. After the operation completed, a test summary will have been
    /// printed
    #[structopt(
        long = "packets-per-syscall",
        takes_value = true,
        value_name = "POSITIVE-INTEGER",
        default_value = "600",
        parse(try_from_str = "parse_non_zero_usize")
    )]
    pub packets_per_syscall: NonZeroUsize,

    #[structopt(flatten)]
    pub exit_config: ExitConfig,
}

#[derive(StructOpt, Debug, Clone, Eq, PartialEq)]
pub struct SocketsConfig {
    /// A receiver of generated traffic, specified as an IP-address and a port
    /// number, separated by a colon.
    ///
    /// This option can be specified several times to identically test multiple
    /// receivers in parallel mode.
    #[structopt(
        short = "r",
        long = "receiver",
        takes_value = true,
        value_name = "SOCKET-ADDRESS",
        required = true
    )]
    pub receivers: Vec<SocketAddr>,

    /// A sender of generated traffic, specified as an IP-address and a port
    /// number, separated by a colon
    #[structopt(
        short = "s",
        long = "sender",
        takes_value = true,
        value_name = "SOCKET-ADDRESS",
        default_value = "0.0.0.0:0"
    )]
    pub sender: SocketAddr,

    /// Displays an interactive menu of network interfaces to use. If unset, a
    /// default one will be used
    #[structopt(
        long = "select-interface",
        takes_value = false,
        conflicts_with = "sender"
    )]
    pub select_interface: bool,

    /// A timeout of sending every single packet. If a timeout is reached, then
    /// a packet will be sent later
    #[structopt(
        short = "t",
        long = "send-timeout",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "10secs",
        parse(try_from_str = "parse_duration")
    )]
    pub send_timeout: Duration,

    /// Specifies the IP_TTL value for all future sockets. Usually this value
    /// equals a number of routers that a packet can go through
    #[structopt(long = "ip-ttl", takes_value = true, value_name = "UNSIGNED-INTEGER")]
    pub ip_ttl: Option<u32>,

    /// Allow sockets to send packets to a broadcast address specified using the
    /// `--receiver` option
    #[structopt(short = "b", long = "allow-broadcast", takes_value = false)]
    pub broadcast: bool,
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
        parse(try_from_str = "parse_time_format")
    )]
    pub date_time_format: String,
}

#[derive(StructOpt, Debug, Clone, Eq, PartialEq)]
pub struct ExitConfig {
    /// A count of packets for sending. When this limit is reached, then the
    /// program will exit
    #[structopt(
        short = "p",
        long = "packets-count",
        takes_value = true,
        value_name = "POSITIVE-INTEGER",
        default_value = "18446744073709551615",
        parse(try_from_str = "parse_non_zero_usize")
    )]
    pub packets_count: NonZeroUsize,

    /// A whole test duration. When this limit is reached, then the program will
    /// exit.
    ///
    /// Exit might occur a few seconds later because of long sendmmsg system
    /// calls. For more precision, decrease the `--packets-per-syscall`
    /// value.
    #[structopt(
        short = "d",
        long = "test-duration",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "64years 64hours 64secs",
        parse(try_from_str = "parse_duration")
    )]
    pub test_duration: Duration,
}

#[derive(StructOpt, Debug, Clone, Eq, PartialEq)]
pub struct PacketsConfig {
    /// Repeatedly send a random-generated packet with a specified bytes length.
    /// The default is 32768
    #[structopt(
        short = "l",
        long = "random-packet",
        takes_value = true,
        value_name = "POSITIVE-INTEGER",
        parse(try_from_str = "parse_non_zero_usize")
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

impl ArgsConfig {
    /// Use it to setup the current structure. It does special additional stuff
    /// unlike the typical `StructOpt::from_args()`.
    pub fn setup() -> ArgsConfig {
        let mut matches = ArgsConfig::from_args();

        // If a user hasn't specified both a file, a text message, and a packet length,
        // then set the default packet length
        if matches.packets_config.send_files.is_empty()
            && matches.packets_config.random_packets.is_empty()
            && matches.packets_config.send_messages.is_empty()
        {
            matches.packets_config.random_packets =
                vec![unsafe { NonZeroUsize::new_unchecked(32768) }];
        }

        matches
    }
}

fn parse_time_format(format: &str) -> Result<String, ParseError> {
    // If this call succeeds, then the format is also correctly
    time::strftime(format, &time::now()).map(|_| format.to_string())
}

fn parse_non_zero_usize(number: &str) -> Result<NonZeroUsize, ParseIntError> {
    number.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Check that ordinary formats are passed correctly
    #[test]
    fn parses_valid_time_format() {
        let check = |format| {
            assert_eq!(
                parse_time_format(format),
                Ok(String::from(format)),
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
    fn parses_invalid_time_format() {
        let check = |format| {
            assert!(
                parse_time_format(format).is_err(),
                "Parses invalid time correctly"
            )
        };

        check("%_=-%vbg=");
        check("yufb%44htv");
        check("sf%jhei9%990");
    }

    // Check that ordinary values are parsed correctly
    #[test]
    fn parses_valid_non_zero_usize() {
        let check = |num| {
            assert_eq!(
                parse_non_zero_usize(num),
                Ok(NonZeroUsize::new(num.parse().unwrap()).unwrap()),
                "Parses valid NonZeroUsize incorrectly"
            )
        };

        check("1");
        check("3");
        check("26655");
        check("+75");
    }

    // Invalid numbers must produce the invalid format error
    #[test]
    fn parses_invalid_non_zero_usize() {
        let check = |num| {
            assert!(
                parse_non_zero_usize(num).is_err(),
                "Parses invalid NonZeroUsize correctly"
            )
        };

        check("   ");
        check("abc5653odr!");
        check("6485&02hde");
        check("-565642");
        check(&"2178".repeat(50));
    }
}
