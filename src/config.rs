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

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::net::SocketAddr;
use std::num::{NonZeroUsize, ParseIntError};
use std::path::PathBuf;
use std::time::Duration;

use humantime::parse_duration;
use structopt::clap::ArgGroup;
use structopt::StructOpt;
use time::ParseError;

#[derive(Debug, Clone, Eq, PartialEq, StructOpt)]
#[structopt(
    author = "Temirkhan Myrzamadi <gymmasssorla@gmail.com>",
    about = "An UDP-based server stress-testing tool, written in Rust.",
    after_help = "For more information see <https://github.com/Gymmasssorla/anevicon>.",
    set_term_width = 80
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

    /// A time interval between sendmmsg syscalls. This option can be used to
    /// decrease test intensity
    #[structopt(
        long = "send-periodicity",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "0secs",
        parse(try_from_str = "parse_duration")
    )]
    pub send_periodicity: Duration,

    #[structopt(flatten)]
    pub logging_config: LoggingConfig,

    #[structopt(flatten)]
    pub exit_config: ExitConfig,

    #[structopt(flatten)]
    pub packet_config: PacketConfig,

    #[structopt(flatten)]
    pub network_config: NetworkConfig,
}

#[derive(StructOpt, Debug, Clone, Eq, PartialEq)]
pub struct NetworkConfig {
    /// A receiver of generated traffic, specified as an IP-address and a port
    /// number, separated by a colon.
    ///
    /// This option can be specified several times to test multiple receivers in
    /// parallel mode.
    ///
    /// All receivers will be tested identically. Run multiple instances of this
    /// program to describe specific characteristics for each receiver.
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

    /// A timeout of sending every single packet. If a timeout is reached, then
    /// a packet will be sent later.
    #[structopt(
        short = "t",
        long = "send-timeout",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "10secs",
        parse(try_from_str = "parse_duration")
    )]
    pub send_timeout: Duration,

    /// A count of packets which the program will send using only one syscall.
    /// After the operation completed, a test summary will have been
    /// printed.
    ///
    /// It is not recommended to set this option to a low value for some
    /// performance reasons.
    #[structopt(
        long = "packets-per-syscall",
        takes_value = true,
        value_name = "POSITIVE-INTEGER",
        default_value = "800",
        parse(try_from_str = "parse_non_zero_usize")
    )]
    pub packets_per_syscall: NonZeroUsize,

    /// Allow sockets to send packets to a broadcast address
    #[structopt(short = "b", long = "allow-broadcast", takes_value = false)]
    pub broadcast: bool,
}

#[derive(StructOpt, Debug, Clone, Eq, PartialEq)]
pub struct LoggingConfig {
    /// Enable one of the possible verbosity levels. The zero level doesn't
    /// print anything, and the last level prints everything
    #[structopt(
        short = "v",
        long = "verbosity",
        takes_value = true,
        value_name = "LEVEL",
        default_value = "3",
        possible_value = "0",
        possible_value = "1",
        possible_value = "2",
        possible_value = "3",
        possible_value = "4",
        possible_value = "5"
    )]
    pub verbosity: i32,

    /// A format for displaying local date and time in log messages. Type `man
    /// strftime` to see the format specification.
    ///
    /// Specifying a different format with days of weeks might be helpful when
    /// you want to test a server more than one day.
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
    /// Exit might occur a few seconds later because of long sendmmsg syscalls.
    /// For more precision, decrease the `--packets-per-syscall` value.
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
pub struct PacketConfig {
    /// Repeatedly send a random-generated packet with a specified bytes length.
    /// The default is 32768
    #[structopt(
        short = "l",
        long = "packet-length",
        takes_value = true,
        value_name = "POSITIVE-INTEGER",
        parse(try_from_str = "parse_non_zero_usize")
    )]
    pub packet_length: Option<NonZeroUsize>,

    /// Interpret the specified file content as a single packet and repeatedly
    /// send it to each receiver
    #[structopt(
        short = "f",
        long = "send-file",
        takes_value = true,
        value_name = "FILENAME"
    )]
    pub send_file: Option<PathBuf>,

    /// Interpret the specified UTF-8 encoded text message as a single packet
    /// and repeatedly send it to each receiver
    #[structopt(
        short = "m",
        long = "send-message",
        takes_value = true,
        value_name = "STRING"
    )]
    pub send_message: Option<String>,
}

impl ArgsConfig {
    pub fn setup() -> ArgsConfig {
        let matches = ArgsConfig::clap()
            .group(ArgGroup::with_name("message").args(&[
                "send_file",
                "packet_length",
                "send_message",
            ]))
            .get_matches();

        let mut args_config = ArgsConfig::from_clap(&matches);

        // If a user hasn't specified both a file, a text message, and a packet length,
        // then set the default packet length
        if !matches.is_present("send_file")
            && !matches.is_present("send_message")
            && !matches.is_present("packet_length")
        {
            args_config.packet_config.packet_length =
                Some(unsafe { NonZeroUsize::new_unchecked(32768) });
        }

        args_config
    }
}

pub fn parse_time_format(format: &str) -> Result<String, ParseError> {
    // If the strftime function consumes the specified date-time format correctly,
    // then the format is also correctly
    time::strftime(format, &time::now())?;

    // Return the same date-time format, but as an owned String instance
    Ok(String::from(format))
}

pub fn parse_non_zero_usize(number: &str) -> Result<NonZeroUsize, NonZeroUsizeError> {
    let number: usize = number.parse().map_err(NonZeroUsizeError::InvalidFormat)?;

    NonZeroUsize::new(number).ok_or(NonZeroUsizeError::ZeroValue)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NonZeroUsizeError {
    InvalidFormat(ParseIntError),
    ZeroValue,
}

impl Display for NonZeroUsizeError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            NonZeroUsizeError::InvalidFormat(error) => write!(fmt, "{}", error),
            NonZeroUsizeError::ZeroValue => write!(fmt, "The value equals to zero"),
        }
    }
}

impl Error for NonZeroUsizeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_time_format() {
        // Check that ordinary formats are passed correctly
        assert_eq!(parse_time_format("%x %X %e"), Ok(String::from("%x %X %e")));
        assert_eq!(parse_time_format("%H %a %G"), Ok(String::from("%H %a %G")));

        assert_eq!(
            parse_time_format("something"),
            Ok(String::from("something"))
        );
        assert_eq!(
            parse_time_format("flower %d"),
            Ok(String::from("flower %d"))
        );
    }

    #[test]
    fn parses_invalid_time_format() {
        let panic_if_invalid = |format| {
            if let Ok(_) = parse_time_format(format) {
                panic!("Parses invalid date-time format correctly");
            }
        };

        // Invalid formats must produce the invalid format error
        panic_if_invalid("%_=-%vbg=");
        panic_if_invalid("yufb%44htv");
        panic_if_invalid("sf%jhei9%990");
    }

    #[test]
    fn parses_valid_non_zero_usize() {
        unsafe {
            // Check that ordinary values are parsed correctly
            assert_eq!(
                parse_non_zero_usize("1"),
                Ok(NonZeroUsize::new_unchecked(1))
            );
            assert_eq!(
                parse_non_zero_usize("3"),
                Ok(NonZeroUsize::new_unchecked(3))
            );
            assert_eq!(
                parse_non_zero_usize("26655"),
                Ok(NonZeroUsize::new_unchecked(26655))
            );
            assert_eq!(
                parse_non_zero_usize("+75"),
                Ok(NonZeroUsize::new_unchecked(75))
            );
        }
    }

    #[test]
    fn parses_invalid_non_zero_usize() {
        let panic_if_invalid = |string| {
            if let Ok(_) = parse_non_zero_usize(string) {
                panic!("Parses invalid formatted usize correctly");
            }
        };

        // Invalid numbers must produce the invalid format error
        panic_if_invalid("   ");

        panic_if_invalid("abc5653odr!");
        panic_if_invalid("6485&02hde");

        panic_if_invalid("-565642");
        panic_if_invalid(&"2178".repeat(50));

        // Check that the zero value is not allowed
        assert_eq!(parse_non_zero_usize("0"), Err(NonZeroUsizeError::ZeroValue));
    }
}
