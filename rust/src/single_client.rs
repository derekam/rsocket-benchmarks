use rsocket_rust::prelude::*;
use rsocket_rust_transport_tcp::TcpClientTransport;
use histogram::Histogram;
use std::time::Instant;
use std::u64;

use crate::ring_iter::{PayloadRing, ResultRing};
use rsocket_rust::runtime::DefaultSpawner;
use std::fs::File;
use std::io::{BufReader, Write, BufWriter};
use futures::{FutureExt, TryFutureExt};
use rsocket_rust::error::RSocketError;
use std::borrow::Borrow;

pub async fn run_benchmarks(mut payloads: &mut PayloadRing<Payload>, writer: &BufWriter<File>) {
    let client: Client<DefaultSpawner> = RSocketFactory::connect()
        .acceptor(Box::new(|| -> Box<dyn RSocket>  {Box::new(EchoRSocket{})}))
        .transport(TcpClientTransport::from("127.0.0.1:7878"))
        .start()
        .await
        .unwrap();

    request_response(&client, payloads, writer).await;
    fire_and_forget(&client, payloads, writer).await;
    channel(&client, payloads, writer).await;
    stream(&client, payloads, writer).await;
}

async fn request_response(client: &Client<DefaultSpawner>, mut payloads: &mut PayloadRing<Payload>, mut writer: &BufWriter<File>) {
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

async fn fire_and_forget(client: &Client<DefaultSpawner>, mut payloads: &mut PayloadRing<Payload>, mut writer: &BufWriter<File>) {
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

async fn channel(client: &Client<DefaultSpawner>, mut payloads: &mut PayloadRing<Payload>, mut writer: &BufWriter<File>) {
    println!("Starting Channel benchmark.");
    let mut times: Vec<u128> = Vec::new();
    let start = Instant::now();
    let mut last = start.clone();

    client.request_channel(Box::pin(ResultRing {
        ring: payloads.into_iter()
    }.into_iter()))
        .map(|result: Result<Payload, RSocketError>| {
            result.unwrap();
            times.push(last.elapsed().as_nanos());
            last = Instant::now();
        });

    write_statistics(start, times, writer);
}

async fn stream(client: &Client<DefaultSpawner>, mut payloads: &mut PayloadRing<Payload>, mut writer: &BufWriter<File>) {
    println!("Starting Stream benchmark.");
    let mut times: Vec<u128> = Vec::new();
    let start = Instant::now();
    let mut last = start.clone();
    //let payload = Payload::builder().set_data(payloads.clone().payload.data())
    //  .set_metadata(payloads.count).build();
    client.request_stream(payloads.clone().payload)
        .map(|result: Result<Payload, RSocketError>| {
            result.unwrap();
            times.push(last.elapsed().as_nanos());
            last = Instant::now();
        });

    write_statistics(start, times, writer);
}

fn write_statistics(start: Instant, times: Vec<u128>, mut writer: &BufWriter<File>) {
    let time: u128 = start.elapsed().as_nanos();
    let count: usize = times.len();
    for i in 0..count {
        writer.write(&times.get(i).unwrap().to_be_bytes());
        if i != count - 1 {
            writer.write(b",");
        }
    }
    let time_seconds: f64 = time as f64 / 1_000_000_000.0 as f64;
    let reqs_per_second: f64 = count as f64 / time_seconds;
    println!("Test complete in {:?} seconds.", time_seconds);
    println!("Test averaged {:?} requests per second.", reqs_per_second);
    writer.write(b"\n");
    writer.write(time_seconds.to_be_bytes().as_ref());
    writer.write(b"\n");
    writer.write(reqs_per_second.to_be_bytes().as_ref());
    writer.write(b"\n");
    writer.flush();
}