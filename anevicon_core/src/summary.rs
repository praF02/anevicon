/* anevicon: The most powerful UDP-based load generator, written in Rust.
 * Copyright (C) 2019  Temirkhan Myrzamadi <gymmasssorla@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * For more information see <https://github.com/Gymmasssorla/anevicon>.
 */

/*!
 * The module containing abstractions to analyse test execution results.
 *
 * # Examples
 *
 * ```rust
 * use anevicon_core::summary::TestSummary;
 *
 * // Create the default TestSummary object with zero generated traffic
 * let mut summary = TestSummary::default();
 *
 * // Update our TestSummary with 59 packets sent containing 52364 bytes
 * // totally
 * summary.update(59, 52364);
 *
 * println!("The total result is: {:?}", summary);
 * ```
 */

use std::time::{Duration, Instant};

/// The test summary abstraction to analyse test execution results.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TestSummary {
    bytes_sent: usize,
    packets_sent: usize,
    initial_time: Instant,
}

impl TestSummary {
    /// Updates the test summary by an adding additional bytes and an
    /// additional packets sent count.
    pub fn update(&mut self, additional_bytes: usize, additional_packets: usize) {
        self.bytes_sent += additional_bytes;
        self.packets_sent += additional_packets;
    }

    /// Returns a count of megabytes sent totally.
    pub fn megabytes_sent(&self) -> usize {
        self.bytes_sent / 1024 / 1024
    }

    /// Returns a count of packets sent totally.
    pub fn packets_sent(&self) -> usize {
        self.packets_sent
    }

    /// Returns an average speeed, specified in Mbps (Megabites Per Second).
    pub fn megabites_per_sec(&self) -> usize {
        let secs_passed = self.time_passed().as_secs() as usize;

        if secs_passed == 0 {
            0
        } else {
            (self.megabytes_sent() * 8) / secs_passed
        }
    }

    /// Returns an average speeed, specified in packets sent per second.
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
    pub fn time_passed(&self) -> Duration {
        self.initial_time.elapsed()
    }
}

impl Default for TestSummary {
    /// Returns the default test summary with current time specified.
    fn default() -> TestSummary {
        TestSummary {
            bytes_sent: 0,
            packets_sent: 0,
            initial_time: Instant::now(),
        }
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

        assert_eq!(summary.megabytes_sent(), 0);
        assert_eq!(summary.packets_sent(), 0);
    }

    #[test]
    fn ordinary_updates_work() {
        let mut summary = TestSummary::default();

        summary.update(1024 * 1024 * 23, 2698);
        assert_eq!(summary.megabytes_sent(), 23);
        assert_eq!(summary.packets_sent(), 2698);

        summary.update(1024 * 1024 * 85, 4258);
        assert_eq!(summary.megabytes_sent(), 85 + 23);
        assert_eq!(summary.packets_sent(), 2698 + 4258);
    }

    #[test]
    fn truncates_megabytes_correctly() {
        let mut summary = TestSummary::default();

        summary.update(1024 * 1023, 5338);
        assert_eq!(
            summary.megabytes_sent(),
            0,
            "'TestSummary' truncates megabytes incorrectly"
        );
        assert_eq!(summary.packets_sent(), 5338);

        // However, we must have one megabyte sent after this update
        summary.update(1024, 19);
        assert_eq!(summary.megabytes_sent(), 1);
    }

    #[test]
    fn zero_update_works() {
        let mut summary = TestSummary::default();
        summary.update(1024 * 1024 * 85, 2698);

        summary.update(0, 0);
        assert_eq!(
            summary.megabytes_sent(),
            85,
            "'TestSummary' hasn't the same megabytes after zero-update"
        );
        assert_eq!(
            summary.packets_sent(),
            2698,
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
            summary.update(1024 * 1024 * 563, 54138);
            sleep(Duration::from_millis(20));
        }

        assert!(summary.time_passed() >= initial_time.elapsed());
    }
}
