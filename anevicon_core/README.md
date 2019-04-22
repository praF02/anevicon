<div align="center">
  <h1>Anevicon Core</h2>
  
  <a href="https://gitter.im/Gymmasssorla/anevicon">
    <img src="https://img.shields.io/badge/chat-on%20gitter-pink.svg">
  </a>
  <a href="https://travis-ci.com/Gymmasssorla/anevicon">
    <img src="https://travis-ci.com/Gymmasssorla/anevicon.svg?branch=master">
  </a>
  <a href="https://github.com/Gymmasssorla/anevicon/blob/master/LICENSE">
    <img src="https://img.shields.io/badge/license-GPLv3-blue.svg">
  </a>
  <a href="https://crates.io/crates/anevicon_core">
    <img src="https://img.shields.io/badge/crates.io-v0.4.5-orange.svg">
  </a>
  <a href="https://docs.rs/anevicon_core">
    <img src="https://img.shields.io/badge/docs.rs-link-blue.svg">
  </a>
  <a href="https://semver.org">
    <img src="https://img.shields.io/badge/semver-follows-green.svg">
  </a>
  
  This crate can be used as a bot to build a [botnet](https://en.wikipedia.org/wiki/Botnet) for simulating [UDP-based DDoS attacks](https://en.wikipedia.org/wiki/UDP_flood_attack) (but only for educational and pentesting purposes, see [the GPLv3 license](https://github.com/Gymmasssorla/anevicon/blob/master/LICENSE), under which the library is distributed).
  
This library was designed to be as convenient and reliable as it is possible, and without any external dependencies (except of
the standard library). If you are just interested in one single program, please take a look at [this one](https://docs.rs/anevicon_core/0.1.0/anevicon_core/).
</div>

----------

## Usage
This example demonstrates sending a couple of messages to the `example.com` domain (just for an example, you should enter here your server):

([`examples/minimal.rs`](https://github.com/Gymmasssorla/anevicon/blob/master/anevicon_core/examples/minimal.rs))
```rust
#![feature(iovec)]

use std::io::IoVec;
use std::net::UdpSocket;

use anevicon_core::{TestSummary, Tester};

fn main() {
    // Setup the socket connected to the example.com domain
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("93.184.216.34:80").unwrap();

    // Setup all the I/O vectors (messages) we want to send
    let payload = &mut [
        (0, IoVec::new(b"Generals gathered in their masses")),
        (0, IoVec::new(b"Just like witches at black masses")),
        (0, IoVec::new(b"Evil minds that plot destruction")),
        (0, IoVec::new(b"Sorcerers of death's construction")),
    ];

    // Send all the created messages using only one system call
    let mut summary = TestSummary::default();
    let mut tester = Tester::new(&socket, &mut summary);

    println!(
        "The total packets sent: {}, the total seconds passed: {}",
        tester.send_multiple(payload).unwrap().packets_sent(),
        summary.time_passed().as_secs()
    );
}
```

This is how you are able to build your own stress-testing bot. Now you can follow [the official documentation](https://docs.rs/anevicon_core) to learn more about the `anevicon_core` abstractions.
