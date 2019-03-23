use anevicon_core::summary::TestSummary;
use anevicon_core::testing::send;

fn main() {
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
}
