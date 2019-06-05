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

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use std::convert::TryInto;

use termion::{color, style, terminal_size};

use config::ArgsConfig;

mod config;
mod core;
mod logging;

fn main() {
    setup_ctrlc_handler();

    let config = ArgsConfig::setup();
    title();

    logging::setup_logging(&config.logging_config);
    trace!("{:?}", config);

    std::process::exit(core::run(config));
}

fn setup_ctrlc_handler() {
    ctrlc::set_handler(move || {
        info!("cancellation has been received. Exiting the program...");
        std::process::exit(0);
    })
    .expect("Error while setting the Ctrl-C handler");

    trace!("the Ctrl-C handler has been configured.");
}

fn title() {
    let tab = " ".repeat(
        ((terminal_size().expect("Failed to get the terminal size").0 - 66) / 2)
            .try_into()
            .unwrap(),
    );

    #[rustfmt::skip]
    println!(
        "{cyan}{}{reset}",
        format!("
{tab}+----------------------------------------------------------------+\n\
{tab}|        \\\\\\  ///     wWw    wWwwW  Ww   c  c     .-.   \\\\\\  /// |\n\
{tab}|    /)  ((O)(O)) wWw (O)    (O)(O)(O)   (OO)   c(O_O)c ((O)(O)) |\n\
{tab}|  (o)(O) | \\ ||  (O)_( \\    / ) (..)  ,'.--.) ,'.---.`, | \\ ||  |\n\
{tab}|   //\\\\  ||\\\\|| .' __)\\ \\  / /   ||  / //_|_\\/ /|_|_|\\ \\||\\\\||  |\n\
{tab}|  |(__)| || \\ |(  _)  /  \\/  \\  _||_ | \\___  | \\_____/ ||| \\ |  |\n\
{tab}|  /,-. | ||  || `.__) \\ `--' / (_/\\_)'.    ) '. `---' .`||  ||  |\n\
{tab}| -'   ''(_/  \\_)       `-..-'          `-.'    `-...-' (_/  \\_) |\n\
{tab}+----------------------------------------------------------------+", tab = tab),
        cyan = color::Fg(color::Cyan),
        reset = color::Fg(color::Reset));

    println!(
        "                           {tab}{red}{bold}version {version}{reset_color}{reset_style}",
        version = structopt::clap::crate_version!(),
        tab = tab,
        bold = style::Bold,
        red = color::Fg(color::Red),
        reset_style = style::Reset,
        reset_color = color::Fg(color::Reset),
    );

    println!(
        "            {tab}{underline}{green}A high-performant UDP-based load \
         generator{reset_style}{reset_color}\n",
        tab = tab,
        underline = style::Underline,
        green = color::Fg(color::Green),
        reset_style = style::Reset,
        reset_color = color::Fg(color::Reset),
    );
}
