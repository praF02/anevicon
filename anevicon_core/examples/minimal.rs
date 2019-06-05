// This example demonstrates sending a couple of messages to the example.com
// domain (just for an example, you should enter here your server):

use std::net::UdpSocket;

use anevicon_core::{TestSummary, Tester};

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
