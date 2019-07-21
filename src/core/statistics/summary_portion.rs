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

/// The abstraction which encapsulates a result of sending a data (one or
/// multiple packets) to a target web server.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SummaryPortion {
    bytes_expected: usize,
    bytes_sent: usize,
    packets_expected: usize,
    packets_sent: usize,
}

impl SummaryPortion {
    /// Constructs an instance of `SummaryPortion` from the specified parts.
    ///
    /// The `bytes_expected` and `packets_expected` variables represent a number
    /// of bytes/packets you were trying to send respectively.
    ///
    /// The `bytes_sent` and `packets_sent` variables represent a number
    /// of bytes/packets you actually sent.
    ///
    /// # Panics
    /// This function panics if one of two conditions (`bytes_sent >
    /// bytes_expected` or `packets_sent > packets_expected`) becomes true.
    pub fn new(
        bytes_expected: usize,
        bytes_sent: usize,
        packets_expected: usize,
        packets_sent: usize,
    ) -> SummaryPortion {
        if bytes_sent > bytes_expected {
            panic!("bytes_sent cannot be higher than bytes_expected");
        }
        if packets_sent > packets_expected {
            panic!("packets_sent cannot be higher than packets_expected");
        }

        SummaryPortion {
            bytes_expected,
            bytes_sent,
            packets_expected,
            packets_sent,
        }
    }

    #[inline]
    pub fn bytes_expected(&self) -> usize {
        self.bytes_expected
    }

    #[inline]
    pub fn bytes_sent(&self) -> usize {
        self.bytes_sent
    }

    #[inline]
    pub fn packets_expected(&self) -> usize {
        self.packets_expected
    }

    #[inline]
    pub fn packets_sent(&self) -> usize {
        self.packets_sent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_portion_valid_works() {
        let (bytes_expected, bytes_sent, packets_expected, packets_sent) = (18394, 1223, 94, 74);
        let portion =
            SummaryPortion::new(bytes_expected, bytes_sent, packets_expected, packets_sent);

        assert_eq!(portion.bytes_expected(), bytes_expected);
        assert_eq!(portion.bytes_sent(), bytes_sent);
        assert_eq!(portion.packets_expected(), packets_expected);
        assert_eq!(portion.packets_sent(), packets_sent);
    }

    #[test]
    #[should_panic(expected = "bytes_sent cannot be higher than bytes_expected")]
    fn summary_portion_panics_bytes() {
        SummaryPortion::new(145, 2456, 544, 544);
    }

    #[test]
    #[should_panic(expected = "packets_sent cannot be higher than packets_expected")]
    fn summary_portion_panics_packets() {
        SummaryPortion::new(457, 456, 8778, 10999);
    }
}
