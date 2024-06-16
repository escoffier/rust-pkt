use std::{env, thread};

use af_packet;
use nom::IResult;
use pktparse::{ethernet::{self, EtherType}, ipv4};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut settings = af_packet::RingSettings::default();
    let mut fds = Vec::<i32>::new();

    settings.fanout_method = af_packet::PACKET_FANOUT_LB;
    settings.ring_settings.tp_feature_req_word = 0;
    settings.if_name = args[1].clone();

    let mut ring = af_packet::Ring::new(settings).unwrap();
    fds.push(ring.fd);

    thread::spawn(move || {
        loop {
            let mut block = ring.get_block();
            for pkt in block.get_raw_packets() {
                if let IResult::Done(remainder, frame) = ethernet::parse_ethernet_frame(&pkt.data[82..]) {
                    if frame.ethertype == EtherType::IPv4  {
                        if let IResult::Done(remainder, ip_pkt) = ipv4::parse_ipv4_header(&remainder)  {
                            
                        }
                    }
                    
                }

            }
            block.mark_as_consumed();
        }
    });

}