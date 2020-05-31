package main

import (
	"bufio"
	"bytes"
	"context"
	"fmt"
	"github.com/jjeffcaii/reactor-go/scheduler"
	"github.com/rsocket/rsocket-go"
	"github.com/rsocket/rsocket-go/payload"
	"github.com/rsocket/rsocket-go/rx"
	"github.com/rsocket/rsocket-go/rx/flux"
	"github.com/rsocket/rsocket-go/rx/mono"
	"log"
	_ "net/http/pprof"
	"strconv"
	"time"
)

func runbenchmarks(benchmark Benchmark, writer *bufio.Writer) {
	log.Println("client starting")

	client, err := createClient(ListenAt)
	for err != nil {
		client, err = createClient(ListenAt)
	}
	ctx, _ := context.WithTimeout(context.Background(), time.Duration(999999999999999999))
	// request response
	times := make([]int64, int(benchmark.count))
	start := time.Now()
	for i := 0; i < benchmark.count; i++ {
		singlestart := time.Now()
		_, err = client.RequestResponse(benchmark.data).DoOnSuccess(func(input payload.Payload) {
			times[i] = time.Since(singlestart).Nanoseconds()
			if i % 1_000_000 == 0 {
				log.Println(i)
			}
		}).DoOnError(func(e error) {
			panic(e)
		}).Block(ctx)
		if err != nil {
			fmt.Println("Error from reqres, err:", err)
		}
	}
	doStats(times, start, writer)

	// fire and forget
	timess := make([]int64, int(benchmark.count))
	startt := time.Now()
	for i := 0; i < benchmark.count; i++ {
		singleStart := time.Now()
		client.FireAndForget(benchmark.data)
		times[i] = time.Since(singleStart).Nanoseconds()
	}
	doStats(timess, startt, writer)

	//req channel
	sendFlux := flux.Create(func(ctx context.Context, s flux.Sink) {
		for i := 0; i < benchmark.count; i++ {
			s.Next(benchmark.data)
		}
		s.Complete()
		ctx.Done()
	})
	// request channel
	timesss := make([]int64, int(benchmark.count))
	i := 0
	starttt := time.Now()
	last := starttt
	_, err = client.RequestChannel(sendFlux).SubscribeOn(scheduler.Single()).
		DoOnNext(func(input payload.Payload) {
			timesss[i] = time.Since(last).Nanoseconds()
			last = time.Now()
			i += 1
		}).DoFinally(func(s rx.SignalType) {
		doStats(timesss, starttt, writer)
	}).BlockLast(ctx)

	if err != nil {
		fmt.Println("Error from channel, err:", err)
	}

	// request stream
	ttimes := make([]int64, int(benchmark.count))
	i = 0
	sstart := time.Now()
	last = sstart
	_, err = client.RequestStream(benchmark.data).SubscribeOn(scheduler.Single()).DoOnNext(func(input payload.Payload) {
		ttimes[i] = time.Since(last).Nanoseconds()
		last = time.Now()
		i += 1
	}).DoFinally(func(s rx.SignalType) {
		doStats(ttimes, sstart, writer)
	}).BlockLast(ctx)
	if err != nil {
		fmt.Println("Error from stream, err:", err)
	}


}

func doStats(times []int64, start time.Time, writer *bufio.Writer) {
	timed := time.Since(start).Nanoseconds()
	for i := 0; i < len(times); i++ {
		writer.WriteString(strconv.FormatInt(times[i], 10))
		if i != len(times) - 1 {
			writer.WriteString(",")
		}
	}
	completedSecs := float64(timed) / float64(1_000_000_000)
    rps := float64(len(times)) / (float64(timed) / float64(1_000_000_000))
	log.Println("Test complete in ", completedSecs, " seconds.")
	log.Println("Test averaged ", rps, " requests per second.")
    writer.WriteString("\n")
	writer.WriteString(strconv.FormatFloat(completedSecs, 'f', 15, 64))
    writer.WriteString("\n")
	writer.WriteString(strconv.FormatFloat(rps, 'f',  15, 64))
    writer.WriteString("\n")
    writer.Flush()
}

func createClient(uri string) (rsocket.Client, error) {
	return rsocket.Connect().
		SetupPayload(payload.NewString("你好", "世界")).
		Acceptor(func(socket rsocket.RSocket) rsocket.RSocket {
			return rsocket.NewAbstractSocket(
				rsocket.RequestResponse(func(p payload.Payload) mono.Mono {
					log.Println("rcv reqresp from server:", p)
					if bytes.Equal(p.Data(), []byte("ping")) {
						return mono.Just(payload.NewString("pong", "from client"))
					}
					return mono.Just(p)
				}),
			)
		}).
		Transport(uri).
		Start(context.Background())
}
