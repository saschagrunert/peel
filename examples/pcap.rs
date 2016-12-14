#[macro_use]
extern crate log;
extern crate pcap;
extern crate peel;

use log::LogLevelFilter;
use pcap::Capture;
use std::env;
use peel::prelude::*;

fn main() {
    // Get the pcap file from the command line arguments
    let arg1 = env::args().nth(1).expect("No trace file specified.");
    let mut cap = Capture::from_file(arg1.clone()).expect("Could not open pcap file.");

    // Get a fresh Peel instance
    let mut peel = get_packet_peel();
    peel.set_log_level(LogLevelFilter::Info);
    let mut packets = 0;

    while let Ok(packet) = cap.next() {
        match peel.traverse(packet.data, Vec::with_capacity(4)) {
            Ok(layers) => info!("{:?}", layers),
            Err(err) => warn!("{}", err),
        }
        packets += 1;
    }

    info!("Done processing {} packets", packets);
}
