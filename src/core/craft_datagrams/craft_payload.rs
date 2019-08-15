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

//! This file is used to construct user's payload.

use std::cell::RefCell;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::io;
use std::num::NonZeroUsize;
use std::path::Path;

use rand::rngs::ThreadRng;
use rand::Rng;
use termion::color;

use crate::config::PayloadConfig;

/// Constructs a bytes packets from `PacketConfig`. Then it must be sent to all
/// receivers multiple times.
///
/// Note that this function constructs **ONLY** payload without
/// protocol-specific headers and etc. Just payload that a user has specified by
/// `--send-file`, `--send-message`, `--random-packet`.
pub fn craft_all(config: &PayloadConfig) -> Result<Vec<Vec<u8>>, CraftPayloadError> {
    let mut packets = Vec::with_capacity(
        config.send_messages.len() + config.send_files.len() + config.random_packets.len(),
    );

    for message in &config.send_messages {
        packets.push(message.as_bytes().to_owned());
    }

    for file in &config.send_files {
        packets.push(read_payload(file)?);
    }

    for length in &config.random_packets {
        packets.push(random_payload(*length));
    }

    Ok(packets)
}

fn random_payload(length: NonZeroUsize) -> Vec<u8> {
    thread_local! {
        static PRNG: RefCell<ThreadRng> = RefCell::new(rand::thread_rng());
    }

    let mut buffer = Vec::with_capacity(length.get());
    PRNG.with(|generator| {
        for _ in 0..length.get() {
            buffer.push(generator.borrow_mut().gen::<u8>());
        }
    });
    buffer
}

fn read_payload<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, CraftPayloadError> {
    let content = fs::read(path).map_err(CraftPayloadError::ReadFailed)?;

    if content.is_empty() {
        return Err(CraftPayloadError::ZeroSize);
    }

    Ok(content)
}

#[derive(Debug)]
pub enum CraftPayloadError {
    ReadFailed(io::Error),
    ZeroSize,
}

impl Display for CraftPayloadError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Self::ReadFailed(err) => write!(
                fmt,
                "Error while reading the file {red}>>>{reset} {error}",
                error = err,
                red = color::Fg(color::Red),
                reset = color::Fg(color::Reset),
            ),
            Self::ZeroSize => write!(fmt, "Zero packet size"),
        }
    }
}

impl Error for CraftPayloadError {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use lazy_static::lazy_static;

    use crate::config::PayloadConfig;

    use super::*;

    lazy_static! {
        static ref PACKET_FILE: PathBuf = PathBuf::from("files/packet.txt");
        static ref SECOND_PACKET_FILE: PathBuf = PathBuf::from("files/second_packet.txt");
        static ref ZERO_FILE: PathBuf = PathBuf::from("files/zero.txt");
        static ref PACKET_CONTENT: Vec<u8> =
            fs::read("files/packet.txt").expect("fs::read(...) failed");
        static ref SECOND_PACKET_CONTENT: Vec<u8> =
            fs::read("files/second_packet.txt").expect("fs::read(...) failed");
    }

    #[test]
    fn generates_random_payload() {
        let length = NonZeroUsize::new(35684).unwrap();
        let buffer = random_payload(length);

        // Check that we've got the correctly length and capacity
        assert_eq!(buffer.len(), length.get());
        assert!(buffer.capacity() >= length.get());
    }

    /// Check that the function must return the 'ZeroSize' error.
    #[test]
    #[should_panic(expected = "Zero packet size")]
    fn test_read_zero_file() {
        if let Err(CraftPayloadError::ZeroSize) = read_payload(ZERO_FILE.to_str().unwrap()) {
            panic!("Zero packet size");
        } else {
            panic!("Must return the 'ZeroSize' error");
        }
    }

    #[test]
    fn test_choose_random_payload() {
        let packet_length = NonZeroUsize::new(24550).unwrap();
        let packets = craft_all(&PayloadConfig {
            send_files: Vec::new(),
            random_packets: vec![packet_length],
            send_messages: Vec::new(),
        })
        .expect("Cannot construct a packet");
        assert_eq!(packets.len(), 1);

        // The function must generate a random set of bytes as a packet
        assert_eq!(packets[0].len(), packet_length.get());
    }

    #[test]
    fn test_choose_file_payload() {
        let packets = craft_all(&PayloadConfig {
            send_files: vec![PACKET_FILE.clone()],
            random_packets: Vec::new(),
            send_messages: Vec::new(),
        })
        .expect("Cannot construct a packet");
        assert_eq!(packets.len(), 1);

        // The function must return a valid file content that we have
        // already written
        assert_eq!(&packets[0], &PACKET_CONTENT.as_slice());
    }

    #[test]
    fn test_choose_text_message() {
        let message = String::from("Generals gathered in their masses");

        let packets = craft_all(&PayloadConfig {
            send_files: Vec::new(),
            random_packets: Vec::new(),
            send_messages: vec![message.clone()],
        })
        .expect("Cannot construct a packet");
        assert_eq!(packets.len(), 1);

        // The function must return the message that we specified above
        assert_eq!(packets[0], message.into_bytes(),);
    }

    /// The `construct_packets` function must generate multiple packets if they
    /// were specified
    #[test]
    fn test_multiple_packets() {
        let first_message = String::from("First message");
        let second_message = String::from("Second message");

        let random_first = NonZeroUsize::new(3566).unwrap();
        let random_second = NonZeroUsize::new(9385).unwrap();

        let packets = craft_all(&PayloadConfig {
            send_files: vec![PACKET_FILE.clone(), SECOND_PACKET_FILE.clone()],
            random_packets: vec![random_first, random_second],
            send_messages: vec![first_message.clone(), second_message.clone()],
        })
        .expect("Cannot construct multiple packets");

        assert_eq!(packets.len(), 6);

        assert_eq!(packets[0], first_message.into_bytes());
        assert_eq!(packets[1], second_message.into_bytes());

        assert_eq!(&packets[2], &PACKET_CONTENT.as_slice());
        assert_eq!(&packets[3], &SECOND_PACKET_CONTENT.as_slice());

        assert_eq!(packets[4].len(), random_first.get());
        assert_eq!(packets[5].len(), random_second.get());
    }
}
