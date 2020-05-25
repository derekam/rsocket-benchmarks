package main

import (
	"context"
	"errors"
	"log"
	_ "net/http/pprof"
	"strings"

	"github.com/rsocket/rsocket-go"
	"github.com/rsocket/rsocket-go/payload"
	"github.com/rsocket/rsocket-go/rx"
	"github.com/rsocket/rsocket-go/rx/flux"
	"github.com/rsocket/rsocket-go/rx/mono"
)

const ListenAt = "tcp://127.0.0.1:4444"

func startserver(benchmark *Benchmark) {
	err := rsocket.Receive().
		OnStart(func() {
			log.Println("server is listening:", ListenAt)
		}).
		Acceptor(func(setup payload.SetupPayload, sendingSocket rsocket.CloseableRSocket) (rsocket.RSocket, error) {
			sendingSocket.OnClose(func(err error) {
				log.Println("***** socket disconnected *****")
			})
			// For SETUP_REJECT testing.
			if strings.EqualFold(setup.DataUTF8(), "REJECT_ME") {
				return nil, errors.New("bye bye bye")
			}
			return responder(benchmark), nil
		}).
		Transport(ListenAt).
		Serve(context.Background())
	if err != nil {
		panic(err)
	}
}

func responder(benchmark *Benchmark) rsocket.RSocket {
	return rsocket.NewAbstractSocket(
		rsocket.MetadataPush(func(item payload.Payload) {
			log.Println("GOT METADATA_PUSH:", item)
		}),
		rsocket.FireAndForget(func(elem payload.Payload) {
		}),
		rsocket.RequestResponse(func(pl payload.Payload) mono.Mono {
			// just echo
			return mono.Just(pl)
		}),
		rsocket.RequestStream(func(pl payload.Payload) flux.Flux {
			return flux.Create(func(ctx context.Context, emitter flux.Sink) {
				for i := 0; i < benchmark.count; i++ {
					// You can use context for graceful coroutine shutdown, stop produce.
					select {
					case <-ctx.Done():
						log.Println("ctx done:", ctx.Err())
						return
					default:
						emitter.Next(benchmark.data)
					}
				}
				emitter.Complete()
			})
		}),
		rsocket.RequestChannel(func(payloads rx.Publisher) flux.Flux {
			return flux.Clone(payloads)

			/*payloads.(flux.Flux).
				//LimitRate(1).
				SubscribeOn(scheduler.Elastic()).
				DoOnNext(func(elem payload.Payload) {
					log.Println("receiving:", elem)
				}).
				Subscribe(context.Background())
			return flux.Create(func(i context.Context, sink flux.Sink) {
				for i := 0; i < 3; i++ {
					sink.Next(payload.NewString("world", fmt.Sprintf("%d", i)))
				}
				sink.Complete()*/
		}),
	)
}
