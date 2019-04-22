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

//! A module containing all the available options for packets sending.
//!
//! # Examples
//! ```rust
//! use anevicon_core::SendOptions;
//!
//! // Create your custom options which tell to don't update a test summary after sending packets
//! let options = SendOptions::default().update(false);
//! ```

/// The main structure describing all the available options for sending packets
/// via `Tester`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SendOptions {
    /// Tells a tester to do not update a specified test summary after sending
    /// packets.
    pub update: bool,
}

impl SendOptions {
    /// Updates the `update` field with your custom value.
    pub fn update(mut self, update: bool) -> SendOptions {
        self.update = update;
        self
    }
}

impl Default for SendOptions {
    fn default() -> SendOptions {
        SendOptions { update: true }
    }
}
