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
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use failure::Fallible;
use termion::color;

use crate::config::{ArgsConfig, Endpoints};
use crate::errors_utils;

mod craft_datagrams;
mod statistics;
mod tester;
mod udp_sender;

thread_local! {
    /// A sender for this thread.
    static SENDER: RefCell<String> = RefCell::new(String::from("Undefined"));

    /// A receiver for this thread.
    static RECEIVER: RefCell<String> = RefCell::new(String::from("Undefined"));
}

fn current_sender() -> String {
    SENDER.with(|string| string.borrow().clone())
}

fn current_receiver() -> String {
    RECEIVER.with(|string| string.borrow().clone())
}

fn init_endpoints(value: Endpoints) {
    SENDER.with(|sender| *sender.borrow_mut() = format!("{sender}", sender = value.sender(),));

    RECEIVER.with(|receiver| {
        *receiver.borrow_mut() = format!("{receiver}", receiver = value.receiver(),)
    });
}

fn current_endpoints_colored() -> String {
    format!(
        "{sender} {yellow}~~~>{reset_color} {receiver}",
        sender = current_sender(),
        receiver = current_receiver(),
        yellow = color::Fg(color::Yellow),
        reset_color = color::Fg(color::Reset),
    )
}

/// This is the key function which accepts a whole `ArgsConfig` and returns
/// `Result<(), ()>` that needs to be returned out of `main()`.
pub fn run(config: ArgsConfig) -> Result<(), ()> {
    let datagrams = match craft_datagrams::craft_all(&config.packets_config) {
        Err(err) => {
            log::error!(
                "failed to construct datagrams!\n{causes}",
                causes = errors_utils::display_error_causes(&err),
            );
            return Err(());
        }
        Ok(datagrams) => datagrams,
    };

    wait(&config);

    let config = Arc::new(config);
    let mut workers =
        Vec::<JoinHandle<Fallible<()>>>::with_capacity(config.packets_config.endpoints.len());

    for (&endpoints, datagrams) in config
        .packets_config
        .endpoints
        .iter()
        .zip(datagrams.into_iter())
    {
        let config = config.clone();

        workers.push(thread::spawn(move || {
            init_endpoints(endpoints);
            tester::run_tester(config, datagrams.collect(), endpoints)?;
            Ok(())
        }));
    }

    workers
        .into_iter()
        .for_each(|worker: JoinHandle<Result<_, failure::Error>>| {
            if let Err(err) = worker.join().expect("A child thread has panicked") {
                log::error!(
                    "a tester exited unexpectedly!\n{causes}",
                    causes = errors_utils::display_error_causes(&err),
                );
            }
        });
    Ok(())
}

fn wait(config: &ArgsConfig) {
    log::warn!(
        "waiting {time} and then starting to execute the tests until {packets} packets will be \
         sent or {duration} duration will be passed...",
        time = humantime::format_duration(config.wait),
        packets = config.exit_config.packets_count,
        duration = humantime::format_duration(config.exit_config.test_duration)
    );
    thread::sleep(config.wait);
}
