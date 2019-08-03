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
use std::sync::Arc;
use std::time::Duration;

use termion::color;

use crate::config::{ArgsConfig, Endpoints};
use crate::core::statistics::TestSummary;
use crate::core::udp_sender::{CreateUdpSenderError, SupplyResult, UdpSender};

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
    for (datagram, _) in datagrams
        .iter()
        .cycle()
        .zip(0..config.exit_config.packets_count.get())
    {
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
        match resend_packets(
            &mut sender,
            &mut summary,
            &datagrams
                .iter()
                .cycle()
                .take(unsent)
                .map(|packet| packet.as_slice())
                .collect::<Vec<&[u8]>>(),
            config.exit_config.test_duration,
        ) {
            ResendPacketsResult::Completed => display_packets_sent(),
            ResendPacketsResult::TimeExpired => display_expired_time(),
        }
    } else {
        display_packets_sent();
    }

    Ok(summary)
}

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

/// Sends `count` packets using the given `summary`. If the `limit` is reached,
/// it will return `ResendPacketsResult::TimeExpired`, otherwise,
/// `ResendPacketsResult::Completed`.
fn resend_packets(
    sender: &mut UdpSender,
    summary: &mut TestSummary,
    datagrams: &[&[u8]],
    limit: Duration,
) -> ResendPacketsResult {
    info!(
        "trying to resend {cyan}{count}{reset} packets to {receiver} from {sender} that haven't \
         been sent yet...",
        count = datagrams.len(),
        receiver = super::current_receiver(),
        sender = super::current_sender(),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );

    for &packet in datagrams {
        if summary.time_passed() >= limit {
            return ResendPacketsResult::TimeExpired;
        }

        while let Err(error) = sender.send_one(summary, packet) {
            error!(
                "failed to send a packet to {receiver} from {sender} >>> {error}! Retrying the \
                 operation...",
                receiver = super::current_receiver(),
                sender = super::current_sender(),
                error = error,
            );
        }
    }

    info!(
        "{cyan}{count}{reset} packets have been resent to {receiver} from {sender}.",
        count = datagrams.len(),
        receiver = super::current_receiver(),
        sender = super::current_sender(),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );

    ResendPacketsResult::Completed
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ResendPacketsResult {
    Completed,
    TimeExpired,
}

fn display_expired_time() {
    info!(
        "the allotted time has passed for {receiver} receiver and {sender} sender.",
        receiver = super::current_receiver(),
        sender = super::current_sender(),
    );
}

fn display_packets_sent() {
    info!(
        "all the packets have been sent to {receiver} from {sender}.",
        receiver = super::current_receiver(),
        sender = super::current_sender(),
    );
}

fn display_summary(summary: &TestSummary) {
    info!(
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
    error!(
        "failed to send packets to {receiver} from {sender} >>> {error}!",
        receiver = super::current_receiver(),
        sender = super::current_sender(),
        error = error,
    );
}
