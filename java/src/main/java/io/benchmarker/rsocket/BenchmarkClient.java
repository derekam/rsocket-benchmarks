package io.benchmarker.rsocket;

import io.rsocket.Payload;
import io.rsocket.RSocket;
import io.rsocket.RSocketFactory;
import io.rsocket.transport.netty.client.TcpClientTransport;
import io.rsocket.util.EmptyPayload;
import lombok.RequiredArgsConstructor;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import reactor.core.publisher.Flux;
import reactor.core.scheduler.Schedulers;

import java.io.*;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;

import static java.lang.System.*;

@RequiredArgsConstructor
public class BenchmarkClient {
    private static final Logger logger = LogManager.getLogger(BenchmarkClient.class);

    private static final String COMMA = ",";
    private final BufferedWriter statsFile;

    private RSocketFactory.ClientRSocketFactory socket;
    private long count;

    public void runBenchmarks(long count, String filename) throws Exception {
        logger.info("Starting benchmarks for payload at: {}", filename);
        socket = RSocketFactory.connect();
        this.count = count;
        runReqResBenchmark(filename);
        runFireForgetBenchmark(filename);
        runChannelBenchmark(filename);
        runStreamingBenchmark();
    }

    public void runStreamingBenchmark() throws Exception {
        logger.info("Starting Streaming benchmark.");
        long[] times = new long[(int) count + 1];
        AtomicInteger n = new AtomicInteger();
        RSocket conn = socket.transport(TcpClientTransport.create("localhost", 7879))
                .start()
                .block();
        long start = nanoTime();
        AtomicLong last = new AtomicLong(start);
        conn.requestStream(EmptyPayload.INSTANCE)
                .subscribeOn(Schedulers.single())
                .doOnEach(res ->  {
                    long now = nanoTime();
                    times[n.getAndIncrement()] = now - last.getAndSet(now);
                })
                .then()
                .toFuture()
                .get();
        doStats(start, times);
    }

    private void runReqResBenchmark(String filename) throws IOException {
        logger.info("Starting Request-Response benchmark.");
        long[] times = new long[(int) count + 1];
        AtomicInteger n = new AtomicInteger();
        RSocket conn = socket.transport(TcpClientTransport.create("localhost", 7879))
                .start()
                .block();
        long start = nanoTime();
        new RingStream(count, filename).iterator().forEachRemaining(payload -> {
            long singleStart = nanoTime();
            try {
                conn.requestResponse(payload)
                        .doFinally(res -> times[n.getAndIncrement()] = nanoTime() - singleStart)
                        .toFuture()
                        .get();
            } catch (Exception e) {
                e.printStackTrace();
            }
        });
        doStats(start, times);
    }

    private void runFireForgetBenchmark(String filename) throws IOException {
        logger.info("Starting Fire-and-Forget benchmark.");
        long[] times = new long[(int) count + 1];
        AtomicInteger n = new AtomicInteger();
        RSocket conn = socket.transport(TcpClientTransport.create("localhost", 7879))
                .start()
                .block();
        long start = nanoTime();
        new RingStream(count, filename).iterator().forEachRemaining(payload -> {
            long singleStart = nanoTime();
            try {
                conn.fireAndForget(payload)
                        .doFinally(res -> times[n.getAndIncrement()] = nanoTime() - singleStart)
                        .toFuture()
                        .get();
            } catch (Exception e) {
                e.printStackTrace();
            }
        });
        doStats(start, times);
    }

    private void runChannelBenchmark(String filename) throws Exception {
        logger.info("Starting Channel benchmark.");
        long[] times = new long[(int) count + 1];
        AtomicInteger n = new AtomicInteger();
        RSocket conn = socket.transport(TcpClientTransport.create("localhost", 7879))
                .start()
                .block();
        long start = nanoTime();
        AtomicLong last = new AtomicLong(start);
        conn.requestChannel(Flux.fromIterable(new RingStream(count, filename))
                    .subscribeOn(Schedulers.single()))
                    .doOnEach(res ->  {
                        long now = nanoTime();
                        times[n.getAndIncrement()] = now - last.getAndSet(now);
                    })
                    .doOnError(err -> {
                        logger.error("Encountered error: {}", err.getLocalizedMessage());
                    })
                    .then()
                    .toFuture()
                    .get();
        doStats(start, times);
    }

    private void doStats(long start, long[] times) throws IOException {
        long time = nanoTime() - start;
        for(int i = 0; i< times.length; i++) {
            statsFile.write(String.valueOf(times[i]));
            if(i != times.length - 1)
                statsFile.write(COMMA);
        }
        double completedSecs = time / 1_000_000_000d;
        double rps = count / (time / 1_000_000_000d);
        logger.info("Test complete in {} seconds.", completedSecs);
        logger.info("Test averaged {} requests per second.", rps);
        statsFile.newLine();
        statsFile.write(String.valueOf(completedSecs));
        statsFile.newLine();
        statsFile.write(String.valueOf(rps));
        statsFile.newLine();
        statsFile.flush();
    }

}
