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
    <img src="https://img.shields.io/badge/crates.io-v5.2.2-orange.svg">
  </a>
  <a href="https://semver.org">
    <img src="https://img.shields.io/badge/semver-follows-green.svg">
  </a>
  
  <img src="https://github.com/Gymmasssorla/anevicon/raw/master/DEMO.png"><br>
  
A high-performant traffic generator, designed to be as convenient and reliable as it is possible. It sends
numerous UDP packets to a server, thereby simulating an activity that can be produced by your end users or a
group of hackers.

  <h4>
    <a href="https://github.com/Gymmasssorla/anevicon/pulse">Pulse</a> &middot;
    <a href="https://github.com/Gymmasssorla/anevicon/stargazers">Stargazers</a> &middot;
    <a href="https://github.com/Gymmasssorla/anevicon/releases">Releases</a> &middot;
    <a href="https://github.com/Gymmasssorla/anevicon/blob/master/CONTRIBUTING.md">Contributing</a>
  </h4>
</div>

----------

## Table of contents
 - [Advantages](https://github.com/Gymmasssorla/anevicon#advantages)
 - [Disadvantages](https://github.com/Gymmasssorla/anevicon#disadvantages)
 - [Installation](https://github.com/Gymmasssorla/anevicon#installation)
   - [Building from crates.io](https://github.com/Gymmasssorla/anevicon#building-from-cratesio)
   - [Building from sources](https://github.com/Gymmasssorla/anevicon#building-from-sources)
   - [Pre-compiled binaries](https://github.com/Gymmasssorla/anevicon#pre-compiled-binaries)
 - [Usage](https://github.com/Gymmasssorla/anevicon#usage)
   - [Flags](https://github.com/Gymmasssorla/anevicon#flags)
   - [Options](https://github.com/Gymmasssorla/anevicon#options)
 - [Overview](https://github.com/Gymmasssorla/anevicon#overview)
   - [Minimal command](https://github.com/Gymmasssorla/anevicon#minimal-command)
   - [Test intensity](https://github.com/Gymmasssorla/anevicon#test-intensity)
   - [Multiple receivers](https://github.com/Gymmasssorla/anevicon#multiple-receivers)
   - [Network interfaces](https://github.com/Gymmasssorla/anevicon#network-interfaces)
   - [Exit conditions](https://github.com/Gymmasssorla/anevicon#exit-conditions)
   - [Custom message](https://github.com/Gymmasssorla/anevicon#custom-message)
   - [Logging options](https://github.com/Gymmasssorla/anevicon#logging-options)
   - [Multiple messages](https://github.com/Gymmasssorla/anevicon#multiple-messages)
 - [Going deeper](https://github.com/Gymmasssorla/anevicon#going-deeper)
 - [Using as a library](https://github.com/Gymmasssorla/anevicon#using-as-a-library)
 - [Contributing](https://github.com/Gymmasssorla/anevicon#contributing)
 - [Legal disclaimer](https://github.com/Gymmasssorla/anevicon#legal-disclaimer)
 - [Contacts](https://github.com/Gymmasssorla/anevicon#contacts)

----------

## Advantages
 - **Linux-accelerated.** Anevicon uses the [`sendmmsg`](http://man7.org/linux/man-pages/man2/sendmmsg.2.html) system call which is specific to Linux. It simply sends large data sets with the single kernel call, thereby reducing CPU load.

 - **Functional.** I've tried to implement as many things to make a multi-functional tool and stay simple at the same time. Such features as multiple tests, verbosity levels, and even the [API](https://crates.io/crates/anevicon_core) are supported.
 
 - **Written in Rust.** How you can see, all the logic is written completely in [Rust](https://www.rust-lang.org/), which means that it leverages bare-metal performance and high-level safety (no SIGSEGV, SIGILL, and other "funny" stuff).

----------

## Disadvantages
 - **Platform-dependend.** Like most of pentesting utilities, this project is developed for only Linux-based systems. If you are a Windows user, you probably need a [virtual machine](https://en.wikipedia.org/wiki/Virtual_machine) or another computer with Linux.

----------

## Installation

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
$ wget https://github.com/Gymmasssorla/anevicon/releases/download/vX.X.X/anevicon-x86_64-linux
$ chmod a+x anevicon-x86_64-linux
```

----------

## Usage

### Flags
Name | Explanation
-----|------------
`-b, --allow-broadcast`| Allow sockets to send packets to a broadcast address specified using the `--receiver` option
`-h, --help` | Prints help information
`--select-if` | Displays an interactive menu of network interfaces to use. If unset, a default one will be used
`-V, --version` | Prints version information

### Options
Name | Value | Default | Explanation
-----|-------|---------|------------
`--date-time-format` | String | `%X` | A format for displaying local date and time in log messages. Type `man strftime` to see the format specification
`--ip-ttl` | Unsigned integer | None | Specifies the `IP_TTL` value for all future sockets. Usually this value equals a number of routers that a packet can go through
`--random-packet` | Positive integer | `32768` | Repeatedly send a random-generated packet with a specified bytes length
`-p, --packets-count` | Positive integer | `18 '446 '744 '073 '709 '551 '615` | A count of packets for sending. When this limit is reached, then the program will exit
`--packets-per-syscall` | Positive integer | `600` | A count of packets which the program will send using only one system call. After the operation completed, a test summary will have been printed
`-r, --receiver` | Socket address | None | A receiver of generated traffic, specified as an IP-address and a port number, separated by a colon.<br><br>This option can be specified several times to identically test multiple receivers in parallel mode.
`-f, --send-file` | Filename | None | Interpret the specified file content as a single packet and repeatedly send it to each receiver
`-m, --send-message` | String | None | Interpret the specified UTF-8 encoded text message as a single packet and repeatedly send it to each receiver
`--send-periodicity` | Time span | `0secs` | A time interval between `sendmmsg` system calls. This option can be used to modify test intensity
`-t, --send-timeout` | Time span | `10secs` | A timeout of sending every single packet. If a timeout is reached, then a packet will be sent later
`-s, --sender` | Socket address | `0.0.0.0:0` | A sender of generated traffic, specified as an IP-address and a port number, separated by a colon
`-d, --test-duration` | Time span | `64years 64hours 64secs` | A whole test duration. When this limit is reached, then the program will exit.<br><br>Exit might occur a few seconds later because of long `sendmmsg` system calls. For more precision, decrease the `--packets-per-syscall` value.
`-v, --verbosity` | From 0 to 5 | `3` | Enable one of the possible verbosity levels. The zero level doesn't print anything, and the last level prints everything.<br><br>Note that specifying the 4 and 5 verbosity levels might decrease performance, do it only for debugging.
`-w, --wait` | Time span | `5secs` | A waiting time span before a test execution used to prevent a launch of an erroneous (unwanted) test

----------

## Overview

### Minimal command
All you need is to provide the testing server address, which consists of an IP address and a port number, separated by the colon character. By default, all sending sockets will have your local address:

```bash
# Test the 80 port of the example.com site using your local address
$ anevicon --receiver=93.184.216.34:80
```

### Test intensity
In some situations, you don't need to transmit the maximum possible amount of packets, you might want to decrease the intensity of packets sending. To do so, there is one more straightforward option called `--send-periodicity`.

```bash
# Test the example.com waiting for 270 microseconds after each sendmmsg syscall
$ anevicon --receiver=93.184.216.34:80 --send-periodicity=270us
```

### Multiple receivers
Anevicon also has the functionality to test multiple receivers in parallel mode, thereby distributing the load on your processor cores. To do so, just specify the `--receiver` option several times.

```bash
# Test the 80 port of example.com and the 13 port of google.com in parallel
$ anevicon --receiver=93.184.216.34:80 --receiver=216.58.207.78:13
```

### Network interfaces
There is also an ability to bind all future sockets to a specific network interface. Consider the `--select-if` flag, which displays an interactive menu of network interfaces in a command line:

```bash
# Test example.com with a custom network interface using `--select-if`
$ anevicon --receiver=93.184.216.34:80 --select-if
```

### Exit conditions
Note that the command above might not work on your system due to the security reasons. To make your test deterministic, there are two end conditions called `--test-duration` and `--packets-count` (a test duration and a packets count, respectively):

```bash
# Test the 80 port of the example.com site with the two limit options
$ anevicon --receiver=93.184.216.34:80 --test-duration=3min --packets-count=7000
```

### Custom message
By default, Anevicon will generate a random packet with a specified size. In some kinds of UDP-based tests, packet content makes sense, and this is how you can specify it using the `--send-file` or `--send-message` options:

```bash
# Test the 80 port of example.com with the custom file 'message.txt'
$ anevicon --receiver=93.184.216.34:80 --send-file="message.txt"

# Test the 80 port of example.com with the custom text message
$ anevicon --receiver=93.184.216.34:80 --send-message="How do you do?"
```

### Logging options
Consider specifying a custom verbosity level from 0 to 5 (inclusively), which is done by the `--verbosity` option. There is also the `--date-time-format` option which tells Anevicon to use your custom date-time format.

```bash
# Use a custom date-time format and the last verbosity level
$ anevicon --receiver=64.233.165.113:80 --date-time-format="%F" --verbosity=5
```

Different verbosity levels print different logging types. As you can see in the table below, the zero verbosity level prints nothing, and the last one prints everything. The levels in the middle print logs selectively:

| | Errors | Warnings | Notifications | Debugs | Traces |
|-|--------|----------|---------------|--------|--------|
| Zero (0) | ❌ | ❌ | ❌ | ❌ | ❌ |
| First (1) | ✔ | ❌ | ❌ | ❌ | ❌ |
| Second (2) | ✔ | ✔ | ❌ | ❌ | ❌ |
| Third (3) | ✔ | ✔ | ✔ | ❌ | ❌ |
| Fourth (4) | ✔ | ✔ | ✔ | ✔ | ❌ |
| Fifth (5) | ✔ | ✔ | ✔ | ✔ | ✔ |


### Multiple messages
[v5.2.0](https://github.com/Gymmasssorla/anevicon/releases/tag/v5.2.0) introduced the multiple messages functionality, which means that you can specify several messages to be sent to a tested web server (but order is not guaranteed).

```bash
# Test the 80 port of example.com with these messages:
#   1) A custom file "file.txt";
#   2) A text message "Hello, Pitty! You're my worst friend.";
#   3) A text message "Hello, Scott! This is just a test.";
#   4) A text message "Goodbye, Albret! You're my best friend.";
#   5) A random packet of 5355 bytes;
#   6) A random packet of 2222 bytes.
$ anevicon --receiver=93.184.216.34:80 \
--send-file="file.txt" \
--send-message "Hello, Pitty! You're my worst friend." \
--send-message "Hello, Scott! This is just a test." \
--send-message "Goodbye, Albert! You're my best friend." \
--random-packet=5355 \
--random-packet=2222
```

----------

## Going deeper
Well, it's time to understand the internals of Anevicon. First, it constructs an iterator of N messages (specified by both `--send-message`, `--send-file`, and `--random-packet`), where N is a number of packets specified by `--packets-count`. Each of these packets is accepted by an optimized sending buffer.

An optimized sending buffer is a data structure representing a sending buffer which can contain M messages, where M is a number of packets transmitted per a [`sendmmsg`](http://man7.org/linux/man-pages/man2/sendmmsg.2.html) system call. When this buffer is full, it flushes all its messages by (surprise!) `sendmmsg`, thereby providing much better performance than an ordinary buffer.

That is, Anevicon has been designed to minimize a number of system calls to your Linux kernel. Yes, we can instead use such libraries as [netmap](https://www.freebsd.org/cgi/man.cgi?query=netmap)/[PF_RING](https://www.ntop.org/products/packet-capture/pf_ring/)/[DPDK](https://www.dpdk.org/), but then users might be confused with running Anevicon on their systems. Anyway, I think that `sendmmsg` provides pretty well performance for all needs.

Here is a visual demonstration of the described process. You enter `anevicon --receiver=93.184.216.34:80 --packets-count=7 --send-message="First" --send-message="Second" --send-message="Third" --packets-per-syscall=3` and the program generates an iterator over ten messages that will be processed by an optimized sending buffer with the capacity of three:

----------

## Using as a library
This program simply sends four packets to http://example.com/. Now you can follow [the official documentation](https://docs.rs/anevicon_core) to learn more about the `anevicon_core` abstractions.

([`examples/minimal.rs`](https://github.com/Gymmasssorla/anevicon/blob/master/anevicon_core/examples/minimal.rs))
```rust
use std::net::UdpSocket;

use anevicon_core::{Tester, TestSummary};

fn main() {
    // Setup the socket connected to the example.com domain
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("93.184.216.34:80").unwrap();

    // Setup all the I/O vectors (messages) we want to send
    let payload = &mut [
        (0, "Generals gathered in their masses".as_bytes()),
        (0, "Just like witches at black masses".as_bytes()),
        (0, "Evil minds that plot destruction".as_bytes()),
        (0, "Sorcerers of death's construction".as_bytes()),
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

----------

## Contributing
You are always welcome for any contribution to this project! But before you start, you should read [the appropriate document](https://github.com/Gymmasssorla/anevicon/blob/master/CONTRIBUTING.md) to know about the preferred development process and the basic communication rules.

----------

## Legal disclaimer
Anevicon was developed as a means of testing stress resistance of web servers, and not for hacking, that is, the author of the project **IS NOT RESPONSIBLE** for any damage caused by your use of his program.

----------

## Contacts
[Temirkhan Myrzamadi](https://github.com/Gymmasssorla) <[gymmasssorla@gmail.com](mailto:gymmasssorla@gmail.com)> (the author)
