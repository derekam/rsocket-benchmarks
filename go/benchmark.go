package main

import (
	"context"
	"github.com/rsocket/rsocket-go/payload"
	"github.com/rsocket/rsocket-go/rx"
	"github.com/rsocket/rsocket-go/rx/flux"
)

type Benchmark struct {
	data payload.Payload
	count int
	sink flux.Sink
}

func (bench Benchmark) Raw() flux.Flux  {
	return flux.Empty()
}

func (bench Benchmark) BlockFirst(context.Context) (payload.Payload, error) {
	return bench.data, nil
}

func (bench Benchmark) BlockLast(context.Context) (payload.Payload, error) {
	return bench.data, nil
}

func (bench Benchmark) DoFinally(rx.FnFinally) flux.Flux {
	return flux.Empty()
}

func (bench Benchmark) DoOnComplete(rx.FnOnComplete) flux.Flux {
	return flux.Empty()
}

func (bench Benchmark) DoOnError(rx.FnOnError) flux.Flux {
	return flux.Empty()
}

func (bench Benchmark) DoOnNext(rx.FnOnNext) flux.Flux {
	return flux.Empty()
}

func (bench Benchmark) DoOnRequest(rx.FnOnRequest) flux.Flux {
	return flux.Empty()
}

func (bench Benchmark) DoOnSubscribe(rx.FnOnSubscribe) flux.Flux {
	return flux.Empty()
}

func (bench Benchmark) OnNext(payload payload.Payload) {
	bench.sink.Next(payload)
}

func (bench Benchmark) OnError(err error) {
	panic("implement me")
}

func (bench Benchmark) OnComplete() {
}

func (bench Benchmark) OnSubscribe(subscription rx.Subscription) {
}

func (bench Benchmark) Filter(rx.FnPredicate) flux.Flux {
	return flux.Empty()
}

func (bench Benchmark) Map(fn func(in payload.Payload) payload.Payload) flux.Flux  {
	return flux.Empty()
}

/*
	Publisher methods
 */

func (bench Benchmark) SubscribeWith(ctx context.Context, s rx.Subscriber) {
	for i := 0; i < bench.count; i++ {
		s.OnNext(bench.data)
	}
	s.OnComplete()
}

func (bench Benchmark) Subscribe(ctx context.Context, options ...rx.SubscriberOption) {
}

