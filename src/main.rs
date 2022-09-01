extern crate env_logger;

use clap::Parser;
use rustdds::DomainParticipant;
use std::{thread, time};

/// Simple program to print dds discovery info
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of seconds to wait for discovery
    #[clap(short, long, value_parser, default_value_t = 3)]
    timeout: u8,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let domain_participant = DomainParticipant::new(0).unwrap();
    thread::sleep(time::Duration::from_secs(args.timeout.into()));

    let discovered_topics = domain_participant.discovered_topics();
    for discovered_topic in discovered_topics.iter() {
        println!("discovered_topic: {}", discovered_topic.topic_name());
    }
}
