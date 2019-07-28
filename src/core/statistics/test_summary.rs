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

use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use std::time::{Duration, Instant};

use crate::core::statistics::SummaryPortion;

/// The structure which represents a whole test execution result by
/// concatenating `SummaryPortion` instances.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TestSummary {
    bytes_expected: usize,
    bytes_sent: usize,
    packets_expected: usize,
    packets_sent: usize,
    initial_time: Instant,

    /// Incoming ICMP error-messages. A key is represented as `(type, code)` and
    /// a value is a number of occurrences.
    incoming_icmp: HashMap<(u8, u8), usize>,
}

impl TestSummary {
    /// Updates the test summary by an performing an addition of the specified
    /// `SummaryPortion` to itself. You can also consider the addition operators
    /// defined as `summary += portion` and `summary + portion`.
    pub fn update(&mut self, portion: SummaryPortion) {
        self.bytes_expected += portion.bytes_expected();
        self.bytes_sent += portion.bytes_sent();

        self.packets_expected += portion.packets_expected();
        self.packets_sent += portion.packets_sent();
    }

    #[inline]
    pub fn update_icmp(&mut self, icmp_type: u8, icmp_code: u8) {
        let key = (icmp_type, icmp_code);
        self.incoming_icmp
            .insert(key, self.incoming_icmp.get(&key).unwrap_or(&0usize) + 1);
    }

    #[inline]
    #[allow(dead_code)]
    pub fn megabytes_expected(&self) -> usize {
        self.bytes_expected / 1024 / 1024
    }

    #[inline]
    pub fn megabytes_sent(&self) -> usize {
        self.bytes_sent / 1024 / 1024
    }

    #[inline]
    pub fn packets_expected(&self) -> usize {
        self.packets_expected
    }

    #[inline]
    pub fn packets_sent(&self) -> usize {
        self.packets_sent
    }

    #[inline]
    pub fn megabites_per_sec(&self) -> usize {
        let secs_passed = self.time_passed().as_secs() as usize;

        if secs_passed == 0 {
            0
        } else {
            (self.megabytes_sent() * 8) / secs_passed
        }
    }

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
    fn default() -> TestSummary {
        TestSummary {
            bytes_expected: 0,
            bytes_sent: 0,
            packets_expected: 0,
            packets_sent: 0,
            initial_time: Instant::now(),
            incoming_icmp: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use super::*;

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

        assert!(summary.incoming_icmp.is_empty());
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
    fn update_icmp_works() {
        let mut summary = TestSummary::default();

        summary.update_icmp(13, 54);
        summary.update_icmp(13, 54);

        summary.update_icmp(44, 11);

        summary.update_icmp(5, 21);
        summary.update_icmp(5, 21);
        summary.update_icmp(5, 21);

        assert_eq!(
            summary
                .incoming_icmp
                .get(&(13, 54))
                .expect("The key (13, 54) doesn't exist"),
            &2usize
        );
        assert_eq!(
            summary
                .incoming_icmp
                .get(&(44, 11))
                .expect("The key (44, 11) doesn't exist"),
            &1usize
        );
        assert_eq!(
            summary
                .incoming_icmp
                .get(&(5, 21))
                .expect("The key (5, 21) doesn't exist"),
            &3usize
        );
    }
}
