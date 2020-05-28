use rsocket_rust::prelude::*;
use rsocket_rust_transport_tcp::TcpClientTransport;
use histogram::Histogram;
use std::time::Instant;
use std::u64;

use crate::ring_iter::PayloadRing;
use rsocket_rust::runtime::DefaultSpawner;
use std::fs::File;
use std::io::BufReader;
use futures::{FutureExt, TryFutureExt};
use rsocket_rust::error::RSocketError;

#[tokio::main]
async fn main() {
    println!("Starting Rust rSocket benchmarks.");
    let COUNT:i32 = 5_000;
    let file = File::open("../resources/Payload.json").unwrap();
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u: serde_json::Value = serde_json::from_reader(reader).unwrap();
    println!("{}", u.to_string());
    let req = Payload::builder()
        .set_data_utf8(&u.to_string())
        .build();

    let payloads1: PayloadRing<Payload> = PayloadRing {
        count: COUNT.clone(),
        payload: req
    };
    run_benchmarks(payloads1).await;
}

pub async fn run_benchmarks(payloads: PayloadRing<Payload>) -> bool {
    let mut histogram = Histogram::new();
    let cli: Client<DefaultSpawner> = RSocketFactory::connect()
        .acceptor(Box::new(|| -> Box<dyn RSocket>  {Box::new(EchoRSocket{})}))
        .transport(TcpClientTransport::from("127.0.0.1:7878"))
        .start()
        .await
        .unwrap();


    let mut time;

    let start = Instant::now();

    for payload in payloads {
        time = Instant::now();
        cli.request_response(payload).await;
        histogram.increment( time.elapsed().as_nanos() as u64);
    }

    println!("Time elapsed: {} ms", start.elapsed().as_millis());

    // print percentiles from the histogram
    println!("Percentiles: p50: {} ns p90: {} ns p99: {} ns p999: {}",
             histogram.percentile(50.0).unwrap(),
             histogram.percentile(90.0).unwrap(),
             histogram.percentile(99.0).unwrap(),
             histogram.percentile(99.9).unwrap(),
    );

    // print additional statistics
    println!("Latency (ns): Min: {} Avg: {} Max: {} StdDev: {}",
             histogram.minimum().unwrap(),
             histogram.mean().unwrap(),
             histogram.maximum().unwrap(),
             histogram.stddev().unwrap(),
    );

    let req = Payload::builder().build();
    time = Instant::now();

    let mut results = cli.request_stream(req);
    loop {
        match results.next().await {
            Some(Ok(v)) => info!("STREAM_RESPONSE OK: {:?}", v),
            Some(Err(e)) => error!("STREAM_RESPONSE FAILED: {:?}", e),
            None => break,
        }
    }

    println!("Time elapsed: {} ms", time.elapsed().as_millis());
    true

}

/*
        socket.requestStream(EmptyPayload.INSTANCE)
                .map(Payload::getDataUtf8)
                .doFinally(res -> {
                    double completedMillis = (nanoTime() - start) / 1_000_000d;
                    double rps = count / ((nanoTime() - start) / 1_000_000_000d);
                    logger.info("test complete in {} ms", completedMillis);
                    logger.info("test rps {}", rps);
                })
                .blockLast();*/