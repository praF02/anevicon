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

use std::io;
use std::num::NonZeroUsize;

use anevicon_core::{Portion, Tester};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SupplyResult {
    Flushed,
    NotFlushed,
}

/// This is a structure which represents a buffer of UDP packets. The buffer
/// capacity equals to a number of packets transmitted per a system call.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PacketsBuffer<'a> {
    buffer: Vec<Portion<'a>>,
}

impl<'a> PacketsBuffer<'a> {
    pub fn new(capacity: NonZeroUsize) -> Self {
        PacketsBuffer {
            buffer: {
                let mut packets = Vec::new();
                packets.reserve_exact(capacity.get());
                packets
            },
        }
    }

    pub fn supply(&mut self, tester: &mut Tester, packet: Portion<'a>) -> io::Result<SupplyResult> {
        let res;

        if self.buffer.len() == self.buffer.capacity() {
            tester.send_multiple(&mut self.buffer).map(|_| ())?;
            self.buffer.clear();
            res = SupplyResult::Flushed;
        } else {
            res = SupplyResult::NotFlushed;
        }

        self.buffer.push(packet);
        Ok(res)
    }

    pub fn complete(&mut self, tester: &mut Tester) -> io::Result<()> {
        if !self.buffer.is_empty() {
            tester.send_multiple(&mut self.buffer).map(|_| ())?;
            self.buffer.clear();
        }

        Ok(())
    }
}
