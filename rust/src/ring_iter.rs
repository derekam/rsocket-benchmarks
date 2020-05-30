use rsocket_rust::prelude::Payload;
use rsocket_rust::error::RSocketError;
use futures::Stream;
use futures::task::Context;
use tokio::macros::support::{Pin, Poll};
use std::cell::RefCell;

#[derive(Clone)]
pub struct PayloadRing<T>
    where
        T: Clone
{
    pub count: i32,
    pub payload: T
}

//unsafe impl Send for RefCell<PayloadRing<Payload>> {}
//unsafe impl Sync for RefCell<PayloadRing<Payload>> {}

/*
unsafe impl Send for PayloadRing<Payload> {}
unsafe impl Sync for PayloadRing<Payload> {}

unsafe impl<T> Send for PayloadRing<T> {}
unsafe impl<T> Sync for PayloadRing<T> {}

unsafe impl Send for Payload {}
unsafe impl Sync for Payload {}
 */

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

impl<K> Stream for RingIntoIterator<K>
    where
        K: Clone {
    type Item = K;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::from(self.next())
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

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::from(Some(Ok(self.iter.next().unwrap())))
    }
}
