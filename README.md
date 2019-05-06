<div align="center">
  <h1>anevicon</h1>
  
  <a href="https://gitter.im/Gymmasssorla/anevicon">
    <img src="https://img.shields.io/badge/chat-on%20gitter-pink.svg">
  </a>
  <a href="https://travis-ci.com/Gymmasssorla/anevicon">
    <img src="https://travis-ci.com/Gymmasssorla/anevicon.svg?branch=master">
  </a>
  <a href="https://github.com/Gymmasssorla/anevicon/blob/master/LICENSE">
    <img src="https://img.shields.io/badge/license-GPLv3-blue.svg">
  </a>
  <a href="https://crates.io/crates/anevicon">
    <img src="https://img.shields.io/badge/crates.io-v5.0.3-orange.svg">
  </a>
  <a href="https://semver.org">
    <img src="https://img.shields.io/badge/semver-follows-green.svg">
  </a>
  
  <img src="media/MAIN.png"><br>
  
A high-performant traffic generator, designed to be as convenient and reliable as it is possible. It sends
numerous UDP packets to a server, thereby simulating an activity that can be produced by your end users or a
group of hackers.

This tool can be also used as a bot to build a botnet for simulating [UDP flood attacks](https://en.wikipedia.org/wiki/UDP_flood_attack) (but only for educational and pentesting purposes). This is achieved by the [Anevicon Core Library](https://crates.io/crates/anevicon_core) with which this program depends on.
  <h4>
    <a href="https://github.com/Gymmasssorla/anevicon/pulse">Pulse</a> &middot;
    <a href="https://github.com/Gymmasssorla/anevicon/stargazers">Stargazers</a> &middot;
    <a href="https://github.com/Gymmasssorla/anevicon/releases">Releases</a> &middot;
    <a href="https://github.com/Gymmasssorla/anevicon/blob/master/CONTRIBUTING.md">Contributing</a>
  </h4>
</div>

----------

## Installation
Currently, this project requires unstable standard library features, so this is why you must switch to the nightly channel to avoid compilation errors:

```bash
$ rustup override set nightly-2019-04-11
```

### Building from crates.io
```bash
$ cargo install anevicon
```

### Building from sources
```bash
$ git clone https://github.com/Gymmasssorla/anevicon.git
$ cd anevicon
$ cargo build --release
```

### Pre-compiled binaries
```bash
$ wget https://github.com/Gymmasssorla/anevicon/releases/download/<VERSION>/anevicon-x86_64-linux
$ chmod a+x anevicon-x86_64-linux
$ ./anevicon-x86_64-linux
```

----------

## Options
```
anevicon 5.0.3
Temirkhan Myrzamadi <gymmasssorla@gmail.com>
A high-performant UDP-based load generator, written in Rust.

USAGE:
    anevicon [FLAGS] [OPTIONS] --receiver <SOCKET-ADDRESS>...

FLAGS:
    -b, --allow-broadcast    Allow sockets to send packets to a broadcast
                             address
    -h, --help               Prints help information
        --select-if          Displays an interactive menu of network interfaces
                             to use. If unset, a default one will be used.
                             
                             This option conflicts with the `--sender` because
                             it will automatically bind an appropriate
                             interface's IP.
    -V, --version            Prints version information

OPTIONS:
        --date-time-format <STRING>
            A format for displaying local date and time in log messages. Type
            `man strftime` to see the format specification.
            
            Specifying a different format with days of weeks might be helpful
            when you want to test a server more than one day. [default: %X]
        --ip-ttl <UNSIGNED-INTEGER>
            Specifies the IP_TTL value for all future sockets. Usually this
            value equals a number of routers that a packet can go through.
    -l, --packet-length <POSITIVE-INTEGER>
            Repeatedly send a random-generated packet with a specified bytes
            length. The default is 32768
    -p, --packets-count <POSITIVE-INTEGER>
            A count of packets for sending. When this limit is reached, then the
            program will exit [default: 18446744073709551615]
        --packets-per-syscall <POSITIVE-INTEGER>
            A count of packets which the program will send using only one
            syscall. After the operation completed, a test summary will have
            been printed.
            
            It is not recommended to set this option to a low value for some
            performance reasons. [default: 600]
    -r, --receiver <SOCKET-ADDRESS>...
            A receiver of generated traffic, specified as an IP-address and a
            port number, separated by a colon.
            
            This option can be specified several times to test multiple
            receivers in parallel mode.
            
            All receivers will be tested identically. Run multiple instances of
            this program to describe specific characteristics for each receiver.
    -f, --send-file <FILENAME>
            Interpret the specified file content as a single packet and
            repeatedly send it to each receiver
    -m, --send-message <STRING>
            Interpret the specified UTF-8 encoded text message as a single
            packet and repeatedly send it to each receiver
        --send-periodicity <TIME-SPAN>
            A time interval between sendmmsg syscalls. This option can be used
            to decrease test intensity [default: 0secs]
    -t, --send-timeout <TIME-SPAN>
            A timeout of sending every single packet. If a timeout is reached,
            then a packet will be sent later [default: 10secs]
    -s, --sender <SOCKET-ADDRESS>
            A sender of generated traffic, specified as an IP-address and a port
            number, separated by a colon [default: 0.0.0.0:0]
    -d, --test-duration <TIME-SPAN>
            A whole test duration. When this limit is reached, then the program
            will exit.
            
            Exit might occur a few seconds later because of long sendmmsg
            syscalls. For more precision, decrease the `--packets-per-syscall`
            value. [default: 64years 64hours 64secs]
    -v, --verbosity <LEVEL>
            Enable one of the possible verbosity levels. The zero level doesn't
            print anything, and the last level prints everything [default: 3]
            [possible values: 0, 1, 2, 3, 4, 5]
    -w, --wait <TIME-SPAN>
            A waiting time span before a test execution used to prevent a launch
            of an erroneous (unwanted) test [default: 5secs]

For more information see <https://github.com/Gymmasssorla/anevicon>.
```

----------

## Using as a library
Just copy this code into your `main.rs` file and launch the compiled program, which simply sends one thousand empty packets to the `example.com` site:

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

----------

## Gallery

<div align="center">
  <h3>Statistics</h3>
  <img src="media/STATS.png">
  
  <h3>Network interfaces</h3>
  <img src="media/INTERFACES.png">
  
  <h3>Being verbose</h3>
  <img src="media/VERBOSE.png">
</div>

----------

## Contributing
You are always welcome for any contribution to this project! But before you start, you should read [the appropriate document](https://github.com/Gymmasssorla/anevicon/blob/master/CONTRIBUTING.md) to know about the preferred development process and the basic communication rules.

----------

## Legal disclaimer
Anevicon was developed as a means of testing stress resistance of web servers, and not for hacking, that is, the author of the project **IS NOT RESPONSIBLE** for any damage caused by your use of his program.

----------

## Contacts
[Temirkhan Myrzamadi](https://github.com/Gymmasssorla) <[gymmasssorla@gmail.com](mailto:gymmasssorla@gmail.com)> (the author)
