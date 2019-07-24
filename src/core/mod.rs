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

//! A module containing the key function `run` which does the main work.

use std::cell::RefCell;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use termion::color;

use construct_payload::construct_payload;
use statistics::TestSummary;
use udp_sender::{SupplyResult, UdpSender};

use crate::config::{ArgsConfig, Endpoints};

mod construct_packets;
mod construct_payload;
mod statistics;
mod udp_sender;

thread_local! {
    /// A colored sender name for this thread.
    static SENDER: RefCell<String> = RefCell::new(
        format!("{cyan}Undefined{reset}", cyan = color::Fg(color::Cyan),
            reset = color::Fg(color::Reset)));

    /// A colored receiver name for this thread.
    static RECEIVER: RefCell<String> = RefCell::new(
        format!("{cyan}Undefined{reset}", cyan = color::Fg(color::Cyan),
            reset = color::Fg(color::Reset)));
}

fn init_endpoints(value: Endpoints) {
    SENDER.with(|sender| {
        *sender.borrow_mut() = format!(
            "{cyan}{sender}{reset}",
            sender = value.sender(),
            cyan = color::Fg(color::Cyan),
            reset = color::Fg(color::Reset),
        )
    });

    RECEIVER.with(|receiver| {
        *receiver.borrow_mut() = format!(
            "{cyan}{receiver}{reset}",
            receiver = value.receiver(),
            cyan = color::Fg(color::Cyan),
            reset = color::Fg(color::Reset),
        )
    });
}

fn current_sender() -> String {
    SENDER.with(|string| string.borrow().clone())
}

fn current_receiver() -> String {
    RECEIVER.with(|string| string.borrow().clone())
}

fn current_endpoints() -> String {
    format!(
        "{sender} ===> {receiver}",
        sender = current_sender(),
        receiver = current_receiver(),
    )
}

/// This is the key function which accepts a whole `ArgsConfig` and returns
/// `Result<(), ()>` that needs to be returned out of `main()`.
pub fn run(config: ArgsConfig) -> Result<(), ()> {
    let payload = match construct_payload(&config.packets_config.payload_config) {
        Err(err) => {
            error!("failed to construct payload >>> {}!", err);
            return Err(());
        }
        Ok(payload) => payload,
    };

    wait(&config);

    let payload = Arc::new(payload);
    let config = Arc::new(config);
    let mut workers = Vec::with_capacity(config.packets_config.endpoints.len());

    for &endpoints in &config.packets_config.endpoints {
        let payload = payload.clone();
        let config = config.clone();

        workers.push(thread::spawn(move || {
            init_endpoints(endpoints);
            run_tester(config, payload, endpoints)?;
            Ok(())
        }));
    }

    workers
        .into_iter()
        .for_each(|worker: JoinHandle<Result<_, RunTesterError>>| {
            if let Err(err) = worker.join().expect("A child thread has panicked") {
                error!("a tester exited unexpectedly >>> {}!", err);
            }
        });
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RunTesterError {
    NixError(nix::Error),
}

impl Display for RunTesterError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            RunTesterError::NixError(err) => err.fmt(fmt),
        }
    }
}

impl Error for RunTesterError {}

fn run_tester(
    config: Arc<ArgsConfig>,
    payload: Arc<Vec<Vec<u8>>>,
    endpoints: Endpoints,
) -> Result<TestSummary, RunTesterError> {
    let packets = payload
        .iter()
        .map(|payload| {
            construct_packets::ip_udp_packet(&endpoints, payload, config.packets_config.ip_ttl)
        })
        .collect::<Vec<Vec<u8>>>();
    let mut summary = TestSummary::default();
    let mut sender = UdpSender::new(
        &endpoints.receiver(),
        config.buffer_capacity,
        &config.sockets_config,
    )
    .map_err(RunTesterError::NixError)?;

    // Run the main cycle for the current worker, and exit if the allotted time
    // expires or all required packets will be sent (whichever happens first)
    for (packet, _) in packets
        .iter()
        .cycle()
        .zip(0..config.exit_config.packets_count.get())
    {
        match sender.supply(&mut summary, packet) {
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

        thread::sleep(config.send_periodicity);
    }

    if let Err(err) = sender.flush(&mut summary) {
        send_multiple_error(err);
    }

    // We might have a situation when not all the required packets are sent, so
    // resend them again
    let unsent =
        unsafe { NonZeroUsize::new_unchecked(summary.packets_expected() - summary.packets_sent()) };

    if unsent.get() != 0 {
        match resend_packets(
            &mut sender,
            &mut summary,
            &packets
                .iter()
                .cycle()
                .take(unsent.get())
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ResendPacketsResult {
    Completed,
    TimeExpired,
}

/// Sends `count` packets using the given `summary`. If the `limit` is reached,
/// it will return `ResendPacketsResult::TimeExpired`, otherwise,
/// `ResendPacketsResult::Completed`.
fn resend_packets(
    sender: &mut UdpSender,
    summary: &mut TestSummary,
    packets: &[&[u8]],
    limit: Duration,
) -> ResendPacketsResult {
    info!(
        "trying to resend {cyan}{count}{reset} packets to {receiver} from {sender} that haven't \
         been sent yet...",
        count = packets.len(),
        receiver = current_receiver(),
        sender = current_sender(),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );

    for &packet in packets {
        if summary.time_passed() >= limit {
            return ResendPacketsResult::TimeExpired;
        }

        while let Err(error) = sender.send_one(summary, packet) {
            error!(
                "failed to send a packet to {receiver} from {sender} >>> {error}! Retrying the \
                 operation...",
                receiver = current_receiver(),
                sender = current_sender(),
                error = error,
            );
        }
    }

    info!(
        "{cyan}{count}{reset} packets have been resent to {receiver} from {sender}.",
        count = packets.len(),
        receiver = current_receiver(),
        sender = current_sender(),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );

    ResendPacketsResult::Completed
}

fn wait(config: &ArgsConfig) {
    warn!(
        "waiting {cyan}{time}{reset} and then starting to execute the tests until \
         {cyan}{packets}{reset} packets will be sent or {cyan}{duration}{reset} duration will be \
         passed...",
        time = humantime::format_duration(config.wait),
        packets = config.exit_config.packets_count,
        duration = humantime::format_duration(config.exit_config.test_duration),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );
    thread::sleep(config.wait);
}

fn display_expired_time() {
    info!(
        "the allotted time has passed for {receiver} receiver and {sender} sender.",
        receiver = current_receiver(),
        sender = current_sender(),
    );
}

fn display_packets_sent() {
    info!(
        "all the packets have been sent to {receiver} from {sender}.",
        receiver = current_receiver(),
        sender = current_sender(),
    );
}

fn display_summary(summary: &TestSummary) {
    info!(
        "stats for {endpoints}:\n\tData Sent:     {cyan}{data_sent}{reset}\n\tAverage Speed: \
         {cyan}{average_speed}{reset}\n\tTime Passed:   {cyan}{time_passed}{reset}",
        endpoints = current_endpoints(),
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

fn send_multiple_error<E: Display>(error: E) {
    error!(
        "failed to send packets to {receiver} from {sender} >>> {error}!",
        receiver = current_receiver(),
        sender = current_sender(),
        error = error,
    );
}
