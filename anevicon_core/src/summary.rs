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

//! The module containing abstractions to analyse test execution results.
//!
//! # Examples
//!
//! ```rust
//! use anevicon_core::summary::{SummaryPortion, TestSummary};
//!
//! // Create the default TestSummary object with zero generated
//! // traffic
//! let mut summary = TestSummary::default();
//!
//! // Update our TestSummary with 59 packets sent containing
//! // 52364 bytes totally
//! summary.update(SummaryPortion::new(72678, 52364, 79, 59));
//!
//! println!(
//!     "{} packets were sent in {} seconds with the average speed of {} packets/sec.",
//!     summary.packets_sent(),
//!     summary.time_passed().as_secs(),
//!     summary.packets_per_sec()
//! );
//! ```

use std::ops::{Add, AddAssign};
use std::time::{Duration, Instant};

/// The structure which represents a whole test execution result by
/// concatenating `SummaryPortion` instances.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct TestSummary {
    bytes_expected: usize,
    bytes_sent: usize,
    packets_expected: usize,
    packets_sent: usize,
    initial_time: Instant,
}

impl TestSummary {
    /// Updates the test summary by an performing an addition of the specified
    /// `SummaryPortion` to itself. You can also consider the addition operators
    /// defined as `summary += portion` and `summary + portion`.
    #[inline]
    pub fn update(&mut self, portion: SummaryPortion) {
        self.bytes_expected += portion.bytes_expected;
        self.bytes_sent += portion.bytes_sent;

        self.packets_expected += portion.packets_expected;
        self.packets_sent += portion.packets_sent;
    }

    /// Returns a count of megabytes you were trying to send.
    #[inline]
    pub fn megabytes_expected(&self) -> usize {
        self.bytes_expected / 1024 / 1024
    }

    /// Returns a count of megabytes sent totally.
    #[inline]
    pub fn megabytes_sent(&self) -> usize {
        self.bytes_sent / 1024 / 1024
    }

    /// Returns a count of packets you were trying to send.
    #[inline]
    pub fn packets_expected(&self) -> usize {
        self.packets_expected
    }

    /// Returns a count of packets sent totally.
    #[inline]
    pub fn packets_sent(&self) -> usize {
        self.packets_sent
    }

    /// Returns an average speeed, specified in Mbps (Megabites Per Second).
    #[inline]
    pub fn megabites_per_sec(&self) -> usize {
        let secs_passed = self.time_passed().as_secs() as usize;

        if secs_passed == 0 {
            0
        } else {
            (self.megabytes_sent() * 8) / secs_passed
        }
    }

    /// Returns an average speeed, specified in packets sent per second.
    #[inline]
    pub fn packets_per_sec(&self) -> usize {
        let secs_passed = self.time_passed().as_secs() as usize;

        if secs_passed == 0 {
            0
        } else {
            self.packets_sent() / secs_passed
        }
    }

    /// Returns a passed time interval since a test summary creation. Note
    /// that this method uses the monotonically non-decreasing time
    /// structure [`Instant`].
    ///
    /// [`Instant`]: https://doc.rust-lang.org/std/time/struct.Instant.html
    #[inline]
    pub fn time_passed(&self) -> Duration {
        self.initial_time.elapsed()
    }
}

impl Add<SummaryPortion> for TestSummary {
    type Output = TestSummary;

    #[inline]
    fn add(mut self, portion: SummaryPortion) -> TestSummary {
        self.update(portion);
        self
    }
}

impl AddAssign<SummaryPortion> for TestSummary {
    #[inline]
    fn add_assign(&mut self, portion: SummaryPortion) {
        self.update(portion);
    }
}

impl Default for TestSummary {
    /// Returns the default test summary with current time specified.
    #[inline]
    fn default() -> TestSummary {
        TestSummary {
            bytes_expected: 0,
            bytes_sent: 0,
            packets_expected: 0,
            packets_sent: 0,
            initial_time: Instant::now(),
        }
    }
}

/// The abstraction which encapsulates a result of sending a data (one or
/// multiple packets) to a target server.
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
    #[inline]
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

    /// Returns a number of bytes you were trying to send (see the
    /// `SummaryPortion::new()` associated function).
    #[inline]
    pub fn bytes_expected(&self) -> usize {
        self.bytes_expected
    }

    /// Returns a number of bytes you actually sent (see the
    /// `SummaryPortion::new()` associated function).
    #[inline]
    pub fn bytes_sent(&self) -> usize {
        self.bytes_sent
    }

    /// Returns a number of packets you were trying to send (see the
    /// `SummaryPortion::new()` associated function).
    #[inline]
    pub fn packets_expected(&self) -> usize {
        self.packets_expected
    }

    /// Returns a number of packets you actually sent (see the
    /// `SummaryPortion::new()` associated function).
    #[inline]
    pub fn packets_sent(&self) -> usize {
        self.packets_sent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;

    #[test]
    fn is_nondecreasing_clock() {
        let summary = TestSummary::default();

        let mut last_time = summary.time_passed();
        for _ in 0..50 {
            // Wait for about 30 milliseconds to see real results
            sleep(Duration::from_millis(30));

            let current_time = summary.time_passed();

            // Check that out clock is monotonically nondecreasing
            assert!(last_time <= current_time);

            last_time = current_time;
        }
    }

    #[test]
    fn is_correct_initial_value() {
        let summary = TestSummary::default();

        assert_eq!(summary.megabytes_expected(), 0);
        assert_eq!(summary.megabytes_sent(), 0);

        assert_eq!(summary.packets_expected(), 0);
        assert_eq!(summary.packets_sent(), 0);
    }

    #[test]
    fn ordinary_updates_work() {
        let mut summary = TestSummary::default();

        summary.update(SummaryPortion::new(
            1024 * 1024 * 24,
            1024 * 1024 * 23,
            3000,
            2698,
        ));

        assert_eq!(summary.megabytes_expected(), 24);
        assert_eq!(summary.megabytes_sent(), 23);

        assert_eq!(summary.packets_expected(), 3000);
        assert_eq!(summary.packets_sent(), 2698);

        summary.update(SummaryPortion::new(
            1024 * 1024 * 85,
            1024 * 1024 * 85,
            4258,
            4258,
        ));
        assert_eq!(summary.megabytes_sent(), 85 + 23);
        assert_eq!(summary.packets_sent(), 2698 + 4258);
    }

    #[test]
    fn truncates_megabytes_correctly() {
        let mut summary = TestSummary::default();

        summary.update(SummaryPortion::new(1024 * 1023, 1024 * 1023, 5338, 5338));
        assert_eq!(
            summary.megabytes_sent(),
            0,
            "'TestSummary' truncates megabytes incorrectly"
        );
        assert_eq!(summary.packets_sent(), 5338);

        // However, we must have one megabyte sent after this update
        summary.update(SummaryPortion::new(1024, 1024, 19, 19));
        assert_eq!(summary.megabytes_sent(), 1);
    }

    #[test]
    fn zero_update_works() {
        let mut summary = TestSummary::default();
        summary.update(SummaryPortion::new(
            1024 * 1024 * 85,
            1024 * 1024 * 58,
            2698,
            2000,
        ));

        summary.update(SummaryPortion::new(0, 0, 0, 0));

        assert_eq!(
            summary.megabytes_expected(),
            85,
            "'TestSummary' hasn't the same megabytes after zero-update"
        );
        assert_eq!(
            summary.megabytes_sent(),
            58,
            "'TestSummary' hasn't the same megabytes after zero-update"
        );

        assert_eq!(
            summary.packets_expected(),
            2698,
            "'TestSummary' hasn't the same megabytes after zero-update"
        );
        assert_eq!(
            summary.packets_sent(),
            2000,
            "'TestSummary' hasn't the same packets after zero-update"
        );
    }

    #[test]
    fn time_passed_works() {
        let mut summary = TestSummary::default();

        // Wait for a little time because the test fails without it
        sleep(Duration::from_millis(10));

        let initial_time = Instant::now();

        // Do an arbitrary updates and sleep that take some time
        for _ in 0..100 {
            summary.update(SummaryPortion::new(
                1024 * 1024 * 563,
                1024 * 1024 * 563,
                54138,
                54138,
            ));
            sleep(Duration::from_millis(20));
        }

        assert!(summary.time_passed() >= initial_time.elapsed());
    }

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
    fn summary_portion_panics_if_invalid_bytes() {
        SummaryPortion::new(145, 2456, 544, 544);
    }

    #[test]
    #[should_panic(expected = "packets_sent cannot be higher than packets_expected")]
    fn summary_portion_panics_if_invalid_packets() {
        SummaryPortion::new(457, 456, 8778, 10999);
    }
}
