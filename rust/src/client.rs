use rsocket_rust::prelude::*;
use rsocket_rust_transport_tcp::TcpClientTransport;
use histogram::Histogram;
use std::time::Instant;
use std::thread;
use std::u64;
use std::error::Error;


#[tokio::main]
async fn main() {
    let mut children = vec![];
    static threads: i32 = 3;


    for _i in 0..threads {
        // Spin up another thread
        children.push(thread::spawn(move || async {
            let mut histogram = Histogram::new();
            static count:i32 = 5_000;
            static iter:i32 = count / threads;
            async {
            let cli = RSocketFactory::connect()
                .acceptor(Box::new(|| -> Box<dyn RSocket>  {Box::new(EchoRSocket{})}))
                .transport(TcpClientTransport::from("127.0.0.1:7878"))
                .setup(Payload::from("READY!"))
                .mime_type("text/plain", "text/plain")
                .start()
                .await
                .unwrap();
                println!("here, dion {}", iter);
            for msg in 0..iter {

                let time = Instant::now();
                let req = Payload::builder()
                    .set_data_utf8("hello")
                    .build();

                cli.request_response(req).await.unwrap();
                histogram.increment( time.elapsed().as_nanos() as u64);
            }
            cli.close();
                0
             }.await;
            return histogram;
        }));
    }

    let mut histogram = Histogram::new();
    for child in children {
        // Wait for the thread to finish. Returns a result.
        let h : Histogram = child.join().unwrap().await;
        histogram.merge(&h);
    }
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
}