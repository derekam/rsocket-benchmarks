use crate::server::run_server;
use crate::single_client::run_benchmarks;
use crate::ring_iter::PayloadRing;
use rsocket_rust::prelude::Payload;
use std::fs::{File, ReadDir};
use std::io::{BufReader, Read, BufWriter, Write};
use std::fs;
use tokio::runtime::Runtime;
use std::time::Duration;
use std::path::Path;

#[macro_use]
extern crate log;

mod benchmark_socket;
mod ring_iter;
mod single_client;
mod server;

fn main() {
    env_logger::builder().format_timestamp_millis().init();

    println!("Starting Rust rSocket benchmarks.");
    const COUNTS: [i32; 6] = [ 10_000_000, 1_000_000, 100_000, 10_000, 1_000, 100 ];
    let mut counter = 0;
    let files: Vec<String> = get_filenames("../resources");
    println!("Running benchmarks for payloads {:?}", files);

    let output: File = File::create("rust_results.csv").unwrap();
    let mut writer = BufWriter::new(output);

    for file in files.iter() {
        let file = File::open("../resources/".to_owned() + file).unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap();
        let val: &str = buffer.as_ref();
        let req : Payload = Payload::builder()
            .set_data_utf8(val)
            .build();

        let count = COUNTS[counter];
        counter += 1;
        let payloads: PayloadRing<Payload> = PayloadRing {
            count: count.clone(),
            payload: req.clone()
        };

        let mut runtime = Runtime::new().unwrap();
        runtime.spawn(run_server(payloads.clone()));
        let client = runtime.spawn(run_benchmarks(payloads.clone()));
        let res: Vec<u8> = runtime.block_on(client).unwrap();
        writer.write(res.as_slice());
        writer.flush();
        runtime.shutdown_timeout(Duration::from_secs(2));
    }

}

fn get_filenames(path: &str) -> Vec<String> {
    let files: ReadDir = fs::read_dir(Path::new(path)).unwrap();
    let mut names: Vec<String> = files.map(|entry| {entry.unwrap().file_name().into_string().unwrap()}).collect();
    names.sort();
    names
}