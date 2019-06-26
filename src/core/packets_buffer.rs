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
        let res = if self.buffer.len() == self.buffer.capacity() {
            tester.send_multiple(&mut self.buffer).map(|_| ())?;
            self.buffer.clear();
            SupplyResult::Flushed
        } else {
            SupplyResult::NotFlushed
        };

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

#[cfg(test)]
mod tests {
    use anevicon_core::TestSummary;

    use super::super::test_utils::loopback_socket;
    use super::*;

    #[test]
    fn constructs_buffer_correctly() {
        let buffer = PacketsBuffer::new(NonZeroUsize::new(354).unwrap());

        assert_eq!(buffer.buffer.capacity(), 354);
        assert_eq!(buffer.buffer.len(), 0);
    }

    #[test]
    fn test_packets_buffer() {
        // Initialise random stuff for PacketsBuffer
        let socket = loopback_socket();
        let mut summary = TestSummary::default();
        let mut tester = Tester::new(&socket, &mut summary);

        let mut buffer = PacketsBuffer::new(NonZeroUsize::new(4).unwrap());

        let check = |buffer: &PacketsBuffer| {
            assert_eq!(buffer.buffer.capacity(), 4);
            assert_eq!(buffer.buffer.last().unwrap().1, "Our packet".as_bytes());
        };

        let mut supply = |buffer: &mut PacketsBuffer| {
            buffer
                .supply(&mut tester, (0, "Our packet".as_bytes()))
                .expect("buffer.supply() failed");
        };

        supply(&mut buffer);
        assert_eq!(buffer.buffer.len(), 1);
        check(&buffer);

        supply(&mut buffer);
        assert_eq!(buffer.buffer.len(), 2);
        check(&buffer);

        supply(&mut buffer);
        assert_eq!(buffer.buffer.len(), 3);
        check(&buffer);

        supply(&mut buffer);
        assert_eq!(buffer.buffer.len(), 4);
        check(&buffer);

        // At this moment our buffer must flush itself
        supply(&mut buffer);
        assert_eq!(buffer.buffer.len(), 1);
        check(&buffer);

        supply(&mut buffer);
        assert_eq!(buffer.buffer.len(), 2);
        check(&buffer);

        buffer
            .complete(&mut tester)
            .expect("buffer.complete() failed");
        assert_eq!(buffer.buffer.len(), 0);
        assert_eq!(buffer.buffer.capacity(), 4);
    }
}
