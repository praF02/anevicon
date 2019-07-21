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

use std::cell::RefCell;
use std::fmt::Display;
use std::thread;

use humantime::format_duration;
use termion::color;

use crate::config::ArgsConfig;
use crate::core::statistics::TestSummary;

mod construct_packets;
mod construct_payload;
mod select_interface;
mod statistics;
mod udp_sender;

thread_local! {
    /// A receiver name for this thread.
    static RECEIVER: RefCell<String> = RefCell::new(String::from("Undefined"))
}

/// Initializes the `RECEIVER` thread-local variable.
fn init_receiver(value: String) {
    RECEIVER.with(|receiver| *receiver.borrow_mut() = value);
}

/// Returns the thread-local value of a current receiver.
#[inline]
fn current_receiver() -> String {
    RECEIVER.with(|string| string.borrow().clone())
}

/// This is the key function which accepts a whole `ArgsConfig` and returns an
/// exit code (either 1 on failure or 0 on success).
pub fn run(config: ArgsConfig) -> Result<(), ()> {
    Ok(())
}

fn wait(config: &ArgsConfig) {
    warn!(
        "waiting {cyan}{time}{reset} and then starting to execute the tests until \
         {cyan}{packets}{reset} packets will be sent or {cyan}{duration}{reset} duration will be \
         passed...",
        time = format_duration(config.wait),
        packets = config.exit_config.packets_count,
        duration = format_duration(config.exit_config.test_duration),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );
    thread::sleep(config.wait);
}

fn display_expired_time() {
    info!(
        "the allotted time has passed for {cyan}{receiver}{reset}.",
        receiver = current_receiver(),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );
}

fn display_packets_sent() {
    info!(
        "all the packets have been sent to {cyan}{receiver}{reset}.",
        receiver = current_receiver(),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );
}

fn display_summary(summary: &TestSummary) {
    info!(
        "stats for {cyan}{receiver}{reset}:\n\tData Sent:     {cyan}{data_sent}{reset}\n\tAverage \
         Speed: {cyan}{average_speed}{reset}\n\tTime Passed:   {cyan}{time_passed}{reset}",
        receiver = current_receiver(),
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
        time_passed = format_duration(summary.time_passed()),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );
}

fn send_multiple_error<E: Display>(error: E) {
    error!(
        "failed to send packets to {cyan}{receiver}{reset} >>> {error}!",
        receiver = current_receiver(),
        error = error,
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset),
    );
}
