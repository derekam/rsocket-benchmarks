use crate::server::run_server;
use crate::single_client::run_benchmarks;
use crate::ring_iter::PayloadRing;
use rsocket_rust::prelude::Payload;
use std::fs::File;
use std::io::{BufReader, Read};
use std::{time, thread};
use std::process::Output;
use std::future::Future;
use std::borrow::Borrow;
use futures::{channel::oneshot, executor::block_on, join};
use tokio::runtime::Runtime;
use bytes::Bytes;
#[macro_use]
extern crate log;

mod benchmark_socket;
mod ring_iter;
mod client;
mod single_client;
mod server;

#[tokio::main]
async fn main() {
    println!("Starting Rust rSocket benchmarks.");
    let counts: [i32; 6] = [ 10_000_000, 1_000_000, 100_000, 10_000, 1_000, 100 ];


    let file = File::open("../resources/PayloadA.json").unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    let val: &str = buffer.as_ref();
    let req : Payload = Payload::builder()
        .set_data_utf8(val)
        .build();

    let req2: Payload = req.clone();

    let payloads1: PayloadRing<Payload> = PayloadRing {
        count: counts[3].clone(),
        payload: req
    };

    let payloads2: PayloadRing<Payload> = PayloadRing {
        count: counts[3].clone(),
        payload: req2.clone()
    };

    let mut runtime = Runtime::new().unwrap();
    runtime.spawn(run_server(payloads1));
    let ten_millis = time::Duration::from_secs(3);
    let now = time::Instant::now();

    thread::sleep(ten_millis);

    runtime.spawn(run_benchmarks(payloads2.clone()));
    thread::sleep(time::Duration::from_secs(20));
    runtime.shutdown_timeout(time::Duration::from_secs(10));
}
