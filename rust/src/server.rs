use rsocket_rust::prelude::*;
use rsocket_rust_transport_tcp::TcpServerTransport;
use std::env;
use std::error::Error;
use crate::ring_iter::{PayloadRing, ResultRing};
use crate::benchmark_socket::BenchmarkSocket;

pub async fn run_server(payloads: PayloadRing<Payload>) -> Result<(), Box<dyn Error + Send + Sync>> {

    let socket: Box<BenchmarkSocket> = Box::new(BenchmarkSocket {
        payloads: ResultRing {
            ring: payloads.clone().into_iter()
        }
    });
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:7878".to_string());
    RSocketFactory::receive()
        .transport(TcpServerTransport::from(addr))
        .acceptor(Box::new(move |_setup, _socket| {
            Ok(socket.clone())
        }))
        .on_start(Box::new(|| info!("+++++++ echo server started! +++++++")))
        .serve()
        .await
}




