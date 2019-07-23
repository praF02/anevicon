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
    <img src="https://img.shields.io/badge/crates.io-v6.0.5-orange.svg">
  </a>
  <a href="https://semver.org">
    <img src="https://img.shields.io/badge/semver-follows-green.svg">
  </a>
  
  <img src="https://github.com/Gymmasssorla/anevicon/raw/master/DEMO.png"><br>
  
An open-source, high-performant traffic generator, designed to be as convenient and reliable as it is possible. It generates
numerous UDP packets which lets you test your server against the abnormaly high activity.

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
   - [Exit conditions](https://github.com/Gymmasssorla/anevicon#exit-conditions)
   - [Custom messages](https://github.com/Gymmasssorla/anevicon#custom-messages)
   - [Logging options](https://github.com/Gymmasssorla/anevicon#logging-options)
   - [Multiple messages](https://github.com/Gymmasssorla/anevicon#multiple-messages)
 - [Going deeper](https://github.com/Gymmasssorla/anevicon#going-deeper)
 - [Contributing](https://github.com/Gymmasssorla/anevicon#contributing)
 - [Legal disclaimer](https://github.com/Gymmasssorla/anevicon#legal-disclaimer)
 - [Contacts](https://github.com/Gymmasssorla/anevicon#contacts)

----------

## Advantages
 - **Linux-accelerated.** Anevicon communicates with a Linux kernel by a few specific system calls to reduce the CPU load significantly. However, it makes the program platform-dependent.

 - **Functional.** I've tried to implement as many things to make a multi-functional tool and stay simple at the same time. Such features as multiple tests, verbosity levels, and even [IP spoofing](https://en.wikipedia.org/wiki/IP_address_spoofing) are supported.
 
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
`-V, --version` | Prints version information

### Options
Name | Value | Default | Explanation
-----|-------|---------|------------
`--date-time-format` | String | `%X` | A format for displaying local date and time in log messages. Type `man strftime` to see the format specification
`-e, --endpoints` | String | None | Two endpoints specified as `<SENDER-ADDRESS>&<RECEIVER-ADDRESS>`, where address is a string of a `<IP>:<PORT>` format.<br><br>A sender and a receiver can be absolutely any valid IPv4/IPv6 addresses (which is used to send spoofed packets sometimes).<br><br>This option can be specified several times to identically test multiple web servers in concurrent mode.
`--ip-ttl` | Unsigned integer | `64` | Specifies the `IP_TTL` value for all future sockets. Usually this value equals a number of routers that a packet can go through
`--random-packet` | Positive integer | `32768` | Repeatedly send a random-generated packet with a specified bytes length
`-p, --packets-count` | Positive integer | `18 '446 '744 '073 '709 '551 '615` | A count of packets for sending. When this limit is reached, then the program will exit
`--buffer-capacity` | Positive integer | `600` | A count of packets which the program will send using only one system call. After the operation completed, a test summary will have been printed
`-f, --send-file` | Filename | None | Interpret the specified file content as a single packet and repeatedly send it to each receiver
`-m, --send-message` | String | None | Interpret the specified UTF-8 encoded text message as a single packet and repeatedly send it to each receiver
`--send-periodicity` | Time span | `0secs` | A time interval between `sendmmsg` system calls. This option can be used to modify test intensity
`-t, --send-timeout` | Time span | `10secs` | A timeout of sending every single packet. If a timeout is reached, then a packet will be sent later
`-d, --test-duration` | Time span | `64years 64hours 64secs` | A whole test duration. When this limit is reached, then the program will exit.<br><br>Exit might occur a few seconds later because of long `sendmmsg` system calls. For more precision, decrease the `--buffer-capacity` value.
`-v, --verbosity` | From 0 to 5 | `3` | Enable one of the possible verbosity levels. The zero level doesn't print anything, and the last level prints everything.<br><br>Note that specifying the 4 and 5 verbosity levels might decrease performance, do it only for debugging.
`-w, --wait` | Time span | `5secs` | A waiting time span before a test execution used to prevent a launch of an erroneous (unwanted) test

----------

## Overview
First of all, please remember that Anevicon uses [raw sockets](https://en.wikipedia.org/wiki/Network_socket#Raw_socket) that require root permissions, so in order to run Anevicon you must already have them. Just type the commands below before running Anevicon:

```bash
$ sudo -s
$ PATH+=":/home/gymmasssorla/.cargo/bin"
```

### Minimal command
All you need is to provide a source address and a server address, each of which consists of an IP address and a port number, separated by the colon character. You must specify them as `<SENDER-ADDRESS>&<RECEIVER-ADDRESS>`:

```bash
# Test example.com:80 on a local network interface 192.168.1.41
$ anevicon --endpoints="192.168.1.41:0&93.184.216.34:80"
```

Here we have `192.168.1.41:0` as a source address (my local network interface given by [ifconfig](https://en.wikipedia.org/wiki/Ifconfig)) and `93.184.216.34:80` (port 80 of http://example.com/) as a receiver of all generated traffic.

You can specify as many `--endpoints` options as you want to test multiple web servers concurrently, Anevicon will spawn one thread for each of endpoints. Also it's possible to [spoof UDP packets](https://en.wikipedia.org/wiki/IP_address_spoofing), in other words, fake a source address so that your receiver will be thinking that someone else sends packets to it, not you!

### Test intensity
In some situations, you don't need to transmit the maximum possible amount of packets, you might want to decrease the intensity of packets sending. To do so, there is one more straightforward option called `--send-periodicity`.

```bash
# Test example.com:80 waiting for 270 microseconds after each sendmmsg call
$ anevicon --endpoints="192.168.1.41:0&93.184.216.34:80" --send-periodicity=270us
```


### Exit conditions
Note that the command above might not work on your system due to the security reasons. To make your test deterministic, there are two end conditions called `--test-duration` and `--packets-count` (a test duration and a packets count, respectively):

```bash
# Test example.com:80 with the two limit options
$ anevicon --endpoints="192.168.1.41:0&93.184.216.34:80" --test-duration=3min --packets-count=7000
```

### Custom messages
By default, Anevicon will generate a random packet with a default size (32768). In some kinds of UDP-based tests, packet content makes sense, and this is how you can specify it using the `--send-file` or `--send-message` options:

```bash
# Test example.com:80 with the custom file 'message.txt'
$ anevicon --endpoints="192.168.1.41:0&93.184.216.34:80" --send-file="message.txt"

# Test example.com:80 with the custom text message
$ anevicon --endpoints="192.168.1.41:0&93.184.216.34:80" --send-message="How do you do?"
```

Also, you are able to specify one or more random packets with your own lengths using the `--random-packet` option. This example specifies two random-generated packets with the sizes 1454 and 29400:

```bash
# Test example.com:80 with two random packets
$ anevicon --endpoints="192.168.1.41:0&93.184.216.34:80" --random-packet=1454 --random-packet=29400
```

### Logging options
Consider specifying a custom verbosity level from 0 to 5 (inclusively), which is done by the `--verbosity` option. There is also the `--date-time-format` option which tells Anevicon to use your custom date-time format.

```bash
# Use a custom date-time format and the last verbosity level
$ anevicon --endpoints="192.168.1.41:0&93.184.216.34:80" --date-time-format="%F" --verbosity=5
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
# Test example.com:80 with these messages
#   1) A custom file "file.txt";
#   2) A text message "Hello, Pitty! You're my worst friend.";
#   3) A text message "Hello, Scott! This is just a test.";
#   4) A text message "Goodbye, Albret! You're my best friend.";
#   5) A random packet of 5355 bytes;
#   6) A random packet of 2222 bytes.
$ anevicon ---endpoints="192.168.1.41:0&93.184.216.34:80" \
--send-file="file.txt" \
--send-message="Hello, Pitty! You're my worst friend." \
--send-message="Hello, Scott! This is just a test." \
--send-message="Goodbye, Albert! You're my best friend." \
--random-packet=5355 \
--random-packet=2222
```

----------

## Going deeper
Well, it's time to understand the internals of Anevicon. First, it constructs an iterator of N messages (specified by both `--send-message`, `--send-file`, and `--random-packet`), where N is a number of packets specified by `--packets-count`. Each of these packets is accepted by an optimized sending buffer.

An optimized sending buffer is a data structure representing a sending buffer which can contain M messages, where M (`--buffer-capacity=M`) is a number of packets transmitted per a [`sendmmsg`](http://man7.org/linux/man-pages/man2/sendmmsg.2.html) system call. When this buffer is full, it flushes all its messages by (surprise!) `sendmmsg`, thereby providing much better performance than an ordinary buffer.

That is, Anevicon has been designed to minimize a number of system calls to your Linux kernel. Yes, we can instead use such libraries as [netmap](https://www.freebsd.org/cgi/man.cgi?query=netmap)/[PF_RING](https://www.ntop.org/products/packet-capture/pf_ring/)/[DPDK](https://www.dpdk.org/), but then users might be confused with running Anevicon on their systems. Anyway, I think that `sendmmsg` provides pretty well performance for all needs.

Here is a visual demonstration of the described process. You enter `anevicon --endpoints="192.168.1.41:0&93.184.216.34:80" --packets-count=7 --send-message="First" --send-message="Second" --send-message="Third" --buffer-capacity=3` and the program generates an iterator over ten messages that will be processed by an optimized sending buffer with the capacity of three:

<div align="center">
  <img src="https://github.com/Gymmasssorla/anevicon/raw/master/media/PROCESS.png">
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
