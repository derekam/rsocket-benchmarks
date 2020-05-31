use rsocket_rust::prelude::Payload;
use rsocket_rust::error::RSocketError;
use futures::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Clone)]
pub struct PayloadRing<T>
    where
        T: Clone
{
    pub count: i32,
    pub payload: T
}

impl<T> IntoIterator for PayloadRing<T>
    where
        T: Clone {
    type Item = T;
    type IntoIter = RingIntoIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        RingIntoIterator {
            ring: self,
            index: 0,
        }
    }

}

#[derive(Clone)]
pub struct RingIntoIterator<K>
    where
        K: Clone {
    ring: PayloadRing<K>,
    index: i32,
}

impl<K> Iterator for RingIntoIterator<K>
    where
        K: Clone {
    type Item = K;
    fn next(&mut self) -> Option<K> {
        return if self.index < self.ring.count {
            self.index += 1;
            Some(self.ring.payload.clone())
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct ResultRing {
    pub ring: RingIntoIterator<Payload>
}

impl IntoIterator for ResultRing {
    type Item = Result<Payload, RSocketError>;
    type IntoIter = IteratorIntoResult;

    fn into_iter(self) -> Self::IntoIter {
        IteratorIntoResult {
            iter: self.ring
        }
    }

}


pub struct IteratorIntoResult {
    iter: RingIntoIterator<Payload>
}

impl Iterator for IteratorIntoResult {
    type Item = Result<Payload, RSocketError>;
    fn next(&mut self) -> Option<Result<Payload, RSocketError>> {
        match self.iter.next() {
            None => {None},
            Some(item) => {Some(Ok(item))},
        }
    }
}

impl Stream for IteratorIntoResult {
    type Item = Result<Payload, RSocketError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.get_mut().iter.next() {
            Some(item) => Poll::from(Some(Ok(item))),
            None => Poll::from(None)
        }
    }
}