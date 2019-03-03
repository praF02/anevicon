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

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::io;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};

use rand::{thread_rng, RngCore};

pub fn construct_packet(
    send_file: &Option<PathBuf>,
    packet_length: &Option<NonZeroUsize>,
) -> Result<Vec<u8>, ReadPacketError> {
    // If an user has specified a file, then use file content as a packet.
    // Otherwise, generate a random set of bytes
    if let Some(ref filename) = send_file {
        read_packet(filename)
    } else {
        // If a file wasn't specified, then at least packet length must
        // be explicitly or implicitly specified
        Ok(random_packet(packet_length.unwrap()))
    }
}

pub fn random_packet(length: NonZeroUsize) -> Vec<u8> {
    // Create a packet without an unnecessary initialization because
    // we'll fill this buffer with random values next
    let mut buffer = Vec::with_capacity(length.get());
    unsafe {
        buffer.set_len(length.get());
    }

    thread_rng().fill_bytes(buffer.as_mut_slice());
    buffer
}

pub fn read_packet<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, ReadPacketError> {
    let content = fs::read(path).map_err(|error| ReadPacketError::ReadFailed(error))?;

    if content.len() == 0 {
        return Err(ReadPacketError::ZeroSize);
    }

    Ok(content)
}

#[derive(Debug)]
pub enum ReadPacketError {
    ReadFailed(io::Error),
    ZeroSize,
}

impl Display for ReadPacketError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            ReadPacketError::ReadFailed(error) => {
                write!(fmt, "Error while reading the file >>> {}", error)
            }
            ReadPacketError::ZeroSize => write!(fmt, "Zero packet size"),
        }
    }
}

impl Error for ReadPacketError {}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Write;
    use std::path::PathBuf;

    use tempfile::NamedTempFile;

    fn test_file() -> NamedTempFile {
        NamedTempFile::new().expect("Cannot create a temp file")
    }

    #[test]
    fn generates_random_packet() {
        let length = unsafe { NonZeroUsize::new_unchecked(35684) };
        let buffer = random_packet(length);

        // Check that we've got the correctly length and capacity
        assert_eq!(buffer.len(), length.get());
        assert!(buffer.capacity() >= length.get());
    }

    #[test]
    #[should_panic(expected = "Zero packet size")]
    fn test_read_zero_file() {
        let temp_file = test_file();

        // Check that the function must return the 'ZeroSize' error
        if let Err(ReadPacketError::ZeroSize) = read_packet(temp_file.path()) {
            panic!("Zero packet size");
        } else {
            panic!("Must return the 'ZeroSize' error");
        }
    }

    #[test]
    fn test_read_valid_file() {
        let mut temp_file = test_file();

        let content = vec![26; 4096];
        temp_file.write_all(&content).unwrap();

        let read_file = read_packet(temp_file.path()).expect("Cannot read a temp file");
        assert_eq!(read_file, content);
    }

    #[test]
    fn test_choose_random_packet() {
        let packet_length = unsafe { NonZeroUsize::new_unchecked(24550) };

        // The function must generate a random set of bytes as a packet
        assert_eq!(
            construct_packet(&None, &Some(packet_length))
                .expect("Cannot construct a packet")
                .len(),
            packet_length.get()
        );
    }

    #[test]
    fn test_choose_file_packet() {
        let mut temp_file = test_file();

        let content = vec![165; 4096];
        temp_file.write_all(&content).unwrap();

        // Now we have a file specified, and the function must read it
        // even with the existing '--length' option (just ignore it)
        assert_eq!(
            construct_packet(
                &Some(PathBuf::from(temp_file.path().to_str().unwrap())),
                &None
            )
            .expect("Cannot construct a packet"),
            content
        );
    }
}
