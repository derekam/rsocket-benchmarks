use futures::future;
use std::future::Future;
use std::pin::Pin;
use std::result::Result;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use rsocket_rust::prelude::{RSocket, Payload, Mono, Flux, StreamExt, Spawner};
use rsocket_rust::error::RSocketError;
use crate::ring_iter::{PayloadRing, ResultRing};
use bytes::Bytes;

//    R: Send + Sync + Clone + Spawner + 'static,
#[derive(Clone)]
pub struct BenchmarkSocket {
    pub payloads: ResultRing
}

impl RSocket for BenchmarkSocket {
    fn metadata_push(&self, req: Payload) -> Mono<()> {
        Box::pin(async {})
    }

    fn fire_and_forget(&self, req: Payload) -> Mono<()> {
        Box::pin(async {})
    }

    fn request_response(&self, req: Payload) -> Mono<Result<Payload, RSocketError>> {
        Box::pin(async move { Ok(req) })
    }

    fn request_stream(&self, req: Payload) -> Flux<Result<Payload, RSocketError>> {
        Box::pin(futures::stream::iter(self.payloads.clone().into_iter()))
    }

    fn request_channel(
        &self,
        mut reqs: Flux<Result<Payload, RSocketError>>,
    ) -> Flux<Result<Payload, RSocketError>> {
        let (sender, receiver) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            while let Some(it) = reqs.next().await {
                sender.send(it).unwrap();
            }
        });
        Box::pin(receiver)
        // or returns directly
        // reqs
    }
}


pub struct BenchmarkSocket2 {
    pub payload: Mutex<PayloadRing<Payload>>
}

impl RSocket for BenchmarkSocket2 {
    fn metadata_push(&self, req: Payload) -> Mono<()> {
        Box::pin(async {})
    }

    fn fire_and_forget(&self, req: Payload) -> Mono<()> {
        Box::pin(async {})
    }

    fn request_response(&self, req: Payload) -> Mono<Result<Payload, RSocketError>> {
        Box::pin(async move { Ok(req) })
    }

    fn request_stream(&self, payload: Payload) -> Flux<Result<Payload, RSocketError>> {
        /*let count = i32::from(payload.metadata().unwrap()[..]);
        let ring = PayloadRing {
            payload,
            count
        };*/
        Box::pin(futures::stream::iter(ResultRing { ring: self.payload.into_inner().unwrap().into_iter() }))
    }

    fn request_channel(
        &self,
        mut reqs: Flux<Result<Payload, RSocketError>>,
    ) -> Flux<Result<Payload, RSocketError>> {
        let (sender, receiver) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            while let Some(it) = reqs.next().await {
                sender.send(it).unwrap();
            }
        });
        Box::pin(receiver)
        // or returns directly
        // reqs
    }
}