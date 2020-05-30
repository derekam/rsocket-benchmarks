use rsocket_rust::prelude::*;
use rsocket_rust_transport_tcp::TcpServerTransport;
use std::env;
use std::error::Error;
use rsocket_rust::error::RSocketError;
use crate::ring_iter::*;
use std::fs::File;
use std::io::BufReader;
use crate::benchmark_socket::{BenchmarkSocket, BenchmarkSocket2};
use std::cell::RefCell;
use std::borrow::Borrow;
use futures::{TryFutureExt, TryFuture};
use std::sync::Mutex;

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
    // run_server(payloads1.clone()).await;
}

pub fn run_server(payloads: Mutex<PayloadRing<Payload>>) -> impl TryFuture {
    ///    fn(SetupPayload, Box<dyn RSocket>) -> Result<Box<dyn RSocket>, Box<dyn Error>>;
    println!("dfadfdadsfsadfsda");

/*        .acceptor(Box::new(|setup, _socket| {
            info!("accept setup: {:?}", setup);
            Ok(Box::new(EchoRSocket))
            // Or you can reject setup
            // Err(From::from("SETUP_NOT_ALLOW"))
        }))

 */
    //let socket: Box<BenchmarkSocket> = ;
    env_logger::builder().format_timestamp_millis().init();
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:7878".to_string());
     RSocketFactory::receive()
       .transport(TcpServerTransport::from(addr))
       .acceptor(Box::new(|setup, _socket| {
           Ok(Box::new(BenchmarkSocket2 {
               payload: payloads
           }))
       }))
         /*.acceptor(move |payload: SetupPayload, sockets: Box<(dyn RSocket )>| ->
       Result<Box<(dyn RSocket)>, Box<(dyn Error)>> {
           Ok(socket.clone() )
       })*/
        .on_start(Box::new(|| info!("+++++++ echo server started! +++++++")))
        .serve().into_future()
}




