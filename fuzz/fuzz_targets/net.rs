#![no_main]
use libfuzzer_sys::fuzz_target;
use laminar::DatagramSocket;

use std::{collections::HashSet, net::SocketAddr, time::Instant};

#[cfg(feature = "tester")]
use laminar::LinkConditioner;
use laminar::{Config, Packet, Socket, SocketEvent};


fuzz_target!(|payload: Vec<u8>| {

    let cfg = Config::default();

    let mut client = Socket::bind_any_with_config(cfg.clone()).unwrap();
    let mut server = Socket::bind_any_with_config(Config {
        blocking_mode: true,
        ..cfg
    })
    .unwrap();

    let server_addr = server.local_addr().unwrap();

    let time = Instant::now();

    client
        .send(Packet::unreliable(server_addr, payload))
        .unwrap();

    client.manual_poll(time);
    server.manual_poll(time);


});