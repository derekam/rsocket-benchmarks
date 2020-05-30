use crate::server::run_server;
use crate::single_client::run_benchmarks;
use crate::ring_iter::PayloadRing;
use rsocket_rust::prelude::Payload;
use std::fs::{File, DirEntry, ReadDir};
use std::io::{BufReader, Read, BufWriter};
use std::{time, thread, env, fs};
use std::process::Output;
use std::future::Future;
use std::borrow::Borrow;
use futures::{channel::oneshot, executor::block_on, join};
use bytes::Bytes;
use std::path::{PathBuf, Path};
use std::convert::TryFrom;
use std::thread::JoinHandle;
use std::error::Error;
use std::cell::RefCell;
use crate::ring_iter::*;
use std::sync::Mutex;

#[macro_use]
extern crate log;

mod benchmark_socket;
mod ring_iter;
mod single_client;
mod server;

#[tokio::main]
async fn main() {
    println!("Starting Rust rSocket benchmarks.");
    const COUNTS: [i32; 6] = [ 10_000_000, 1_000_000, 100_000, 10_000, 1_000, 100 ];

    let FILES: Vec<String> = get_filenames("../resources");
    println!("Running benchmarks for payloads {:?}", FILES);

    let output: File = File::create("rust_results.csv").unwrap();
    let mut writer = BufWriter::new(output);

    let mut payloads1: Mutex<PayloadRing<Payload>> = Mutex::new(PayloadRing {
        count: 0,
        payload: Payload::builder().build()
    });



    let server = run_server(payloads1);

    let ten_millis = time::Duration::from_secs(3);

    thread::sleep(ten_millis);

    for file in FILES.iter() {
        let file = File::open("../resources/".to_owned() + file).unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap();
        let val: &str = buffer.as_ref();
        let req : Payload = Payload::builder()
            .set_data_utf8(val)
            .build();

        let req2: Payload = req.clone();
        for count in COUNTS.iter() {
            payloads1.lock().unwrap().payload = req.clone();
            payloads1.lock().unwrap().count = count.clone();

            let mut payloads2: PayloadRing<Payload> = PayloadRing {
                count: count.clone(),
                payload: req2.clone()
            };
            run_benchmarks(&mut payloads2, &writer).await;

            thread::sleep(time::Duration::from_secs(20));
        }
    }
    server.await;

}

fn get_filenames(path: &str) -> Vec<String> {
    let files: ReadDir = fs::read_dir(Path::new(path)).unwrap();
    let mut names: Vec<String> = files.map(|entry| {entry.unwrap().file_name().into_string().unwrap()}).collect();
    names.sort();
    names
}
