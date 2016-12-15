#[macro_use]
extern crate log;
extern crate pcap;
extern crate peel;
extern crate time;

use log::LogLevelFilter;
use pcap::Capture;
use std::env;
use peel::prelude::*;
use time::precise_time_ns;

fn main() {
    // Get the pcap file from the command line arguments
    let arg1 = env::args().nth(1).expect("No trace file specified.");
    let mut cap = Capture::from_file(arg1.clone()).expect("Could not open pcap file.");

    // Get a fresh Peel instance
    let mut peel = default_peel();
    peel.set_log_level(LogLevelFilter::Warn);

    // Account some stats
    let mut packets = 0;
    let mut bytes = 0;

    let now = precise_time_ns();
    while let Ok(packet) = cap.next() {
        bytes += packet.data.len();
        match peel.traverse(packet.data, Vec::with_capacity(4)) {
            Ok(layers) => info!("{:?}", layers),
            Err(err) => warn!("{}", err),
        }
        packets += 1;
    }
    let duration = precise_time_ns() - now;
    let mib_s = (bytes as f64 / 1024f64 / 1024f64) / (duration as f64 / 1_000_000_000f64);

    warn!("Done processing {} packets.", packets);
    warn!("{} bytes / {} nanoseconds = {:.2} MiB/s ({:.2} MBit/s)",
          bytes,
          duration,
          mib_s,
          mib_s as u64 * 8);
}
