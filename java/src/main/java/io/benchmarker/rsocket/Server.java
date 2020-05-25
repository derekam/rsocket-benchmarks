package io.benchmarker.rsocket;

import io.rsocket.Payload;
import io.rsocket.RSocketFactory;
import io.rsocket.transport.netty.server.CloseableChannel;
import io.rsocket.transport.netty.server.TcpServerTransport;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import java.util.function.Supplier;

public class Server {

    private final CloseableChannel channel;
    private final BenchmarkSocket benchmarkSocket = new BenchmarkSocket();

    public Server() {
        channel = RSocketFactory.receive()
                .acceptor((setup, socket) -> Mono.just(benchmarkSocket))
                .transport(TcpServerTransport.create(7879))
                .start()
                .doOnSubscribe(s -> System.out.println("server started"))
                .doOnError(err -> {
                    System.out.println(err);
                })
                .block();
    }

    public void setSupplier(Supplier<Flux<Payload>> supplier) {
        benchmarkSocket.setFluxSupplier(supplier);
    }

}

