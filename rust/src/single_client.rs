use rsocket_rust::prelude::*;
use rsocket_rust_transport_tcp::TcpClientTransport;
use std::time::Instant;

use crate::ring_iter::{PayloadRing, ResultRing};
use rsocket_rust::runtime::DefaultSpawner;
use std::io::{ Write };
use futures::{ TryFutureExt};

pub async fn run_benchmarks(payloads: PayloadRing<Payload>) -> Vec<u8> {
    let client: Client<DefaultSpawner> = RSocketFactory::connect()
        .acceptor(Box::new(|| -> Box<dyn RSocket>  {Box::new(EchoRSocket{})}))
        .transport(TcpClientTransport::from("127.0.0.1:7878"))
        .start()
        .await
        .unwrap();
    let mut response: Vec<u8> = Vec::new();
    request_response(&client, payloads.clone(), &mut response).await;
    fire_and_forget(&client, payloads.clone(), &mut response).await;
    channel(&client, payloads.clone(), &mut response).await;
    stream(&client, payloads.clone(), &mut response).await;
    response
}

async fn request_response(client: &Client<DefaultSpawner>, payloads: PayloadRing<Payload>, writer: &mut Vec<u8>) {
    println!("Starting Request-Response benchmark.");
    let mut times: Vec<u128> = Vec::new();
    let mut time;
    let start = Instant::now();

    for payload in payloads.into_iter() {
        time = Instant::now();
        client.request_response(payload).into_future().await;
        times.push(time.elapsed().as_nanos());
    }

    write_statistics(start, times, writer);
}

async fn fire_and_forget(client: &Client<DefaultSpawner>, payloads: PayloadRing<Payload>, writer: &mut Vec<u8>) {
    println!("Starting Fire-and-Forget benchmark.");
    let mut times: Vec<u128> = Vec::new();
    let mut time;
    let start = Instant::now();

    for payload in payloads.into_iter() {
        time = Instant::now();
        client.fire_and_forget(payload).await;
        times.push(time.elapsed().as_nanos());
    }

    write_statistics(start, times, writer);
}

async fn channel(client: &Client<DefaultSpawner>, payloads: PayloadRing<Payload>, writer: &mut Vec<u8>) {
    println!("Starting Channel benchmark.");
    let mut times: Vec<u128> = Vec::new();
    let start = Instant::now();
    let mut last = start.clone();

    let mut results = client.request_channel(Box::pin(ResultRing {
        ring: payloads.into_iter()
    }.into_iter()));
    loop {
        match results.next().await {
            Some(Ok(_v)) => {
                times.push(last.elapsed().as_nanos());
                last = Instant::now();
            },
            Some(Err(e)) => {
                error!("CHANNEL_RESPONSE FAILED: {:?}", e);
            }
            None => break,
        }
    }

    write_statistics(start, times, writer);
}

async fn stream(client: &Client<DefaultSpawner>, payloads: PayloadRing<Payload>, writer: &mut Vec<u8>) {
    println!("Starting Stream benchmark.");
    let mut times: Vec<u128> = Vec::new();
    let start = Instant::now();
    let mut last = start.clone();

    let mut results =client.request_stream(payloads.clone().payload);
    loop {
        match results.next().await {
            Some(Ok(_v)) => {
                times.push(last.elapsed().as_nanos());
                last = Instant::now();
            },
            Some(Err(e)) => {
                error!("CHANNEL_RESPONSE FAILED: {:?}", e);
            }
            None => break,
        }
    }

    write_statistics(start, times, writer);
}

fn write_statistics(start: Instant, times: Vec<u128>, writer: &mut Vec<u8>) {
    let time: u128 = start.elapsed().as_nanos();
    let count: usize = times.len();
    for i in 0..count {
        writer.write(&times.get(i).unwrap().to_string().as_bytes());
        if i != count - 1 {
            writer.write(b",");
        }
    }
    let time_seconds: f64 = time as f64 / 1_000_000_000.0 as f64;
    let reqs_per_second: f64 = count as f64 / time_seconds;
    println!("Test complete in {:?} seconds.", time_seconds);
    println!("Test averaged {:?} requests per second.", reqs_per_second);
    writer.write(b"\n");
    writer.write(time_seconds.to_string().as_bytes());
    writer.write(b"\n");
    writer.write(reqs_per_second.to_string().as_bytes());
    writer.write(b"\n");
}