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

## Contents
 - [Ass-kicking features](https://github.com/Gymmasssorla/anevicon#ass-kicking-features)
 - [Installation](https://github.com/Gymmasssorla/anevicon#installation)
   - [Building from crates.io](https://github.com/Gymmasssorla/anevicon#building-from-cratesio)
   - [Building from sources](https://github.com/Gymmasssorla/anevicon#building-from-sources)
   - [Pre-compiled binaries](https://github.com/Gymmasssorla/anevicon#pre-compiled-binaries)
 - [Options](https://github.com/Gymmasssorla/anevicon#options)
 - [Using as a program](https://github.com/Gymmasssorla/anevicon#using-as-a-program)
   - [Minimal command](https://github.com/Gymmasssorla/anevicon#minimal-command)
   - [Multiple receivers](https://github.com/Gymmasssorla/anevicon#multiple-receivers)
   - [IP spoofing](https://github.com/Gymmasssorla/anevicon#ip-spoofing)
   - [End conditions](https://github.com/Gymmasssorla/anevicon#end-conditions)
   - [Packet size](https://github.com/Gymmasssorla/anevicon#packet-size)
   - [Custom message](https://github.com/Gymmasssorla/anevicon#custom-message)
   - [Test intensity](https://github.com/Gymmasssorla/anevicon#test-intensity)
   - [Verbosity levels](https://github.com/Gymmasssorla/anevicon#verbosity-levels)
   - [Date-time format](https://github.com/Gymmasssorla/anevicon#date-time-format)
   - [Packets per one syscall](https://github.com/Gymmasssorla/anevicon#packets-per-one-syscall)
   - [Send timeout](https://github.com/Gymmasssorla/anevicon#send-timeout)
   - [Waiting before a test](https://github.com/Gymmasssorla/anevicon#waiting-before-a-test)
   - [Network interfaces](https://github.com/Gymmasssorla/anevicon#network-interfaces)
 - [Using as a library](https://github.com/Gymmasssorla/anevicon#using-as-a-library)
 - [Gallery](https://github.com/Gymmasssorla/anevicon#gallery)
   - [Statistics](https://github.com/Gymmasssorla/anevicon#statistics)
   - [Network interfaces](https://github.com/Gymmasssorla/anevicon#network-interfaces-1)
   - [Being verbose](https://github.com/Gymmasssorla/anevicon#being-verbose)
 - [Contributing](https://github.com/Gymmasssorla/anevicon#contributing)
 - [Cautions](https://github.com/Gymmasssorla/anevicon#cautions)
 - [Contacts](https://github.com/Gymmasssorla/anevicon#contacts)
 

----------

## Ass-kicking features

 - **Written in Rust.** How you can see, Anevicon is completely written in [Rust](https://www.rust-lang.org/), which means that it leverages high-level safeness and bare-metal performance, achieved by zero-cost abstractions.
 
 - **Linux acceleration.** Anevicon is developed for Linux-based operating systems not just like that. It interacts with the kernel using the [sendmmsg](http://man7.org/linux/man-pages/man2/sendmmsg.2.html) system call, transmitting multiple messages at once.
 
 - **Functionality.** I've tried to implement as many things to make a multi-functional tool and stay simple at the same time. Such features as multiple tests, verbosity levels, IP spoofing and many more are supported.

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
$ wget https://github.com/Gymmasssorla/anevicon/releases/download/<VERSION>/anevicon.AppImage
$ chmod a+x anevicon.AppImage
$ ./anevicon.AppImage
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

## Using as a program

### Minimal command
All you need is to provide the testing server address, which consists of an IP address and a port number, separated by the colon character. By default, all sending sockets will have your local address:

```bash
# Test the 80 port of the example.com site using your local address
$ anevicon --receiver 93.184.216.34:80
```

### Multiple receivers
Anevicon also has the functionality to test multiple receivers in parallel mode, thereby distributing the load on your processor cores. To do so, just specify the `--receiver` option several times.

```bash
# Test the 80 port of example.com and the 13 port of google.com in parallel
$ anevicon --receiver 93.184.216.34:80 --receiver 216.58.207.78:13
```

### IP spoofing
Using the IP spoofing technique, hackers can protect their bandwidth from server response messages and hide their real IP address. You can imitate it via the `--sender` command-line option, as described below:

```bash
# Test the 80 port of the example.com site using its own IP address
$ anevicon --receiver 93.184.216.34:80 --sender 93.184.216.34:80
```

### End conditions
Note that the command above might not work on your system due to the security reasons. To make your test deterministic, there are two end conditions called `--test-duration` and `--packets-count` (a test duration and a packets count, respectively):

```bash
# Test the 80 port of the example.com site with the two limit options
$ anevicon --receiver 93.184.216.34:80 --test-duration 3min --packets-count 7000
```

### Packet size
Note that the test below will end when, and only when one of two specified end conditions become true. And what is more, you can specify a global packet length in bytes:

```bash
# Test the 80 port of example.com with the packet length of 4092 bytes
$ anevicon --receiver 93.184.216.34:80 --packet-length 4092
```

### Custom message
By default, Anevicon will generate a random packet with a specified size. In some kinds of UDP-based tests, packet content makes sense, and this is how you can specify it using the `--send-file` or `--send-message` options:

```bash
# Test the 80 port of example.com with the custom file 'message.txt'
$ anevicon --receiver 93.184.216.34:80 --send-file message.txt

# Test the 80 port of example.com with the custom text message
$ anevicon --receiver 93.184.216.34:80 --send-message "How do you do?"
```

### Test intensity
In some situations, you don't need to transmit the maximum possible amount of packets, you might want to decrease the intensity of packets sending. To do so, there is one more straightforward option called `--send-periodicity`.

```bash
# Test the example.com waiting for 270 microseconds after each sendmmsg syscall
$ anevicon --receiver 93.184.216.34:80 --send-periodicity 270us
```

### Verbosity levels
Anevicon supports a few verbosity levels from zero to five inclusively. Zero level prints nothing, first level prints only errors, second level adds warnings, third adds notifications, fourth adds debugs, and fifth - traces.

```bash
# Test the 80 port of example.com using the fourth verbosity level
$ anevicon --receiver 93.184.216.34:80 --verbosity 4
```

### Date-time format
You can explicitly specify your custom date-time format that is used for displaying every log message. Setting a format with days and weeks might be helpful if you want to test something more than one day:

```bash
# Test with the format displaying months, days, years, hours, minutes, and seconds
$ anevicon --receiver 93.184.216.34:80 --date-time-format "%D %X"
```

### Packets per one syscall
For performance reasons, Anevicon uses the [sendmmsg](http://man7.org/linux/man-pages/man2/sendmmsg.2.html) syscall by default, reducing CPU usage significantly. Specifying a number of packets being sent per a syscall is also supported.

```bash
# Test the 80 port of the example.com site with 1200 packets per one syscall
$ anevicon --receiver 93.184.216.34:80 --packets-per-syscall 1200
```

### Send timeout
Network operations sometimes are not performed momentarily. This is why, the program supports the `--send-timeout` option which represents a duration, after which an error will be printed if a packet isn't sent.

```bash
# Test the 80 port of the example.com site using the timeout of 200 milliseconds
$ anevicon --receiver 93.184.216.34:80 --send-timeout 200ms
```

### Waiting before a test
The most vulnerable element of a system is an object sitting between a computer and a chair. So to prevent executing an erroneous test, there is the `--wait` option which waits five seconds by default:

```bash
# Test the example.com site waiting 30 seconds before the execution
$ anevicon --receiver 93.184.216.34:80 --wait 30seconds
```

### Network interfaces
Anevicon automatically uses a default network interface for your system, but you can also choose yours custom. Consider the `--select-if` option which tells the program to display interactive menu of network interfaces.

```bash
# Choose a custom network interface to which bind all future sockets
$ anevicon --receiver 93.184.216.34:80 --select-if
```

----------

## Using as a library
Just copy this code into your `main.rs` file and launch the compiled program, which simply sends one thousand empty packets to the `example.com` site:

([`examples/minimal.rs`](https://github.com/Gymmasssorla/anevicon/blob/master/anevicon_core/examples/minimal.rs))
```rust
#![feature(iovec)]

use std::io::IoVec;
use std::net::UdpSocket;

use anevicon_core::summary::TestSummary;
use anevicon_core::tester::Tester;

fn main() {
    // Setup the socket connected to the example.com domain
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("93.184.216.34:80").unwrap();

    // Setup all the I/O vectors (messages) we want to send
    let paylod = &mut [
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
        tester.send_multiple(paylod).unwrap(),
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

## Cautions
- Anevicon was developed as a means of testing stress resistance of web servers, and not for hacking, that is, the author of the project **IS NOT RESPONSIBLE** for any damage caused by your use of my program.
 
 - Despite the fact that Anevicon is heavily tested both automatically and manually, does not mean that the author is responsible for any bug in his work because the program comes with **ABSOLUTELY NO WARRANTY**.

----------

## Contacts
[Temirkhan Myrzamadi](https://github.com/Gymmasssorla) <[gymmasssorla@gmail.com](mailto:gymmasssorla@gmail.com)> (the author)
