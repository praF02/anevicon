// This example demonstrates sending a couple of messages to the example.com
// domain (just for an example, you should enter here your server):

#![feature(iovec)]

use std::io::IoVec;
use std::net::UdpSocket;

use anevicon_core::{SendOptions, TestSummary, Tester};

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
        tester
            .send_multiple(payload, SendOptions::default())
            .unwrap()
            .packets_sent(),
        summary.time_passed().as_secs()
    );
}
