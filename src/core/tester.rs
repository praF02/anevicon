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
use std::num::NonZeroUsize;
use std::sync::Arc;

use termion::color;

use crate::config::{ArgsConfig, Endpoints};
use crate::core::statistics::TestSummary;
use crate::core::udp_sender::{CreateUdpSenderError, SupplyResult, UdpSender};

#[derive(Debug)]
pub enum RunTesterError {
    UdpSenderError(CreateUdpSenderError),
}

impl Display for RunTesterError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            RunTesterError::UdpSenderError(err) => err.fmt(fmt),
        }
    }
}

impl Error for RunTesterError {}

pub fn run_tester(
    config: Arc<ArgsConfig>,
    datagrams: Vec<Vec<u8>>,
    endpoints: Endpoints,
) -> Result<TestSummary, RunTesterError> {
    let mut summary = TestSummary::default();
    let current_receiver = endpoints.receiver();
    let mut sender = UdpSender::new(
        config.test_intensity,
        &current_receiver,
        config.sockets_config.broadcast,
    )
    .map_err(RunTesterError::UdpSenderError)?;

    // Run the main cycle for the current worker, and exit if the allotted time
    // expires or all required packets will be sent (whichever happens first)
    let mut packets_to_send = config.exit_config.packets_count.get();
    loop {
        for (datagram, _) in datagrams.iter().cycle().zip(0..packets_to_send) {
            match sender.supply(&mut summary, datagram) {
                Err(err) => send_multiple_error(err),
                Ok(res) => {
                    if res == SupplyResult::Flushed {
                        display_summary(&summary);
                    }
                }
            }

            if summary.time_passed() >= config.exit_config.test_duration {
                display_expired_time();
                return Ok(summary);
            }
        }

        if let Err(err) = sender.flush(&mut summary) {
            send_multiple_error(err);
        }

        // We might have a situation when not all the required packets are sent, so
        // resend them again
        let unsent = summary.packets_expected() - summary.packets_sent();
        if unsent != 0 {
            packets_to_send = unsent;
        } else {
            display_packets_sent(config.exit_config.packets_count);
            break;
        }
    }

    Ok(summary)
}

fn display_expired_time() {
    log::info!(
        "the allotted time has passed for {receiver} receiver and {sender} sender.",
        receiver = super::current_receiver(),
        sender = super::current_sender(),
    );
}

fn display_packets_sent(packets_count: NonZeroUsize) {
    log::info!(
        "{cyan}{packets_count}{reset} packets have been sent to {receiver} from {sender}.",
        packets_count = packets_count,
        receiver = super::current_receiver(),
        sender = super::current_sender(),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );
}

fn display_summary(summary: &TestSummary) {
    log::info!(
        "stats for {endpoints}:\n\tData Sent:     {cyan}{data_sent}{reset}\n\tAverage Speed: \
         {cyan}{average_speed}{reset}\n\tTime Passed:   {cyan}{time_passed}{reset}",
        endpoints = super::current_endpoints(),
        data_sent = format!(
            "{packets} packets ({megabytes} MB)",
            packets = summary.packets_sent(),
            megabytes = summary.megabytes_sent(),
        ),
        average_speed = format!(
            "{packets_per_sec} packets/sec ({mbps} Mbps)",
            packets_per_sec = summary.packets_per_sec(),
            mbps = summary.megabites_per_sec(),
        ),
        time_passed = humantime::format_duration(summary.time_passed()),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );
}

fn send_multiple_error<E: Error>(error: E) {
    log::error!(
        "failed to send packets to {receiver} from {sender} {red}>>>{reset} {error}!",
        receiver = super::current_receiver(),
        sender = super::current_sender(),
        error = error,
        red = color::Fg(color::Red),
        reset = color::Fg(color::Reset),
    );
}

#[cfg(test)]
mod tests {
    use std::net::UdpSocket;

    use structopt::StructOpt;

    use crate::core::craft_datagrams;

    use super::*;

    #[test]
    fn test_run_tester() {
        let socket = UdpSocket::bind("127.0.0.1:0").expect("UdpSocket::bind(...) failed");

        let config = ArgsConfig::from_iter(&[
            "anevicon",
            "--endpoints",
            &format!("{0}&{0}", socket.local_addr().unwrap()),
            "--packets-count",
            "1000",
            "--test-intensity",
            "42",
            "--send-message",
            "My first message",
            "--send-message",
            "My second message",
            "--send-message",
            "My third message",
            "--send-file",
            "files/packet.txt",
            "--wait",
            "0secs",
        ]);

        let packets_expected = config.exit_config.packets_count.get();
        let datagrams = craft_datagrams::craft_all(&config.packets_config)
            .expect("Cannot construct datagarms")
            .remove(0)
            .collect::<Vec<Vec<u8>>>();

        let endpoints = config.packets_config.endpoints[0];
        let summary =
            run_tester(Arc::new(config), datagrams, endpoints).expect("Failed to run a tester");

        assert_eq!(summary.packets_expected(), packets_expected);
        assert_eq!(summary.packets_sent(), packets_expected);
    }
}
