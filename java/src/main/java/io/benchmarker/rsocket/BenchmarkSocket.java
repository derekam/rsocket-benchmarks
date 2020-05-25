package io.benchmarker.rsocket;

import io.rsocket.Payload;
import io.rsocket.RSocket;
import io.rsocket.util.DefaultPayload;
import lombok.Setter;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.reactivestreams.Publisher;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;
import reactor.core.scheduler.Schedulers;

import java.util.function.Supplier;

public class BenchmarkSocket implements RSocket {
    private static final Logger LOGGER = LogManager.getLogger(BenchmarkSocket.class);

    @Setter
    private Supplier<Flux<Payload>> fluxSupplier;


    public Mono<Void> fireAndForget(Payload payload) {
        return Mono.empty();
    }

    /**
     * Returns the sent payload unmodified to the requestor.
     * @param payload
     * @return
     */
    public Mono<Payload> requestResponse(Payload payload) {
        try {
            return Mono.just(payload);
        } catch (Exception x) {
            return Mono.error(x);
        }
    }

    public Flux<Payload> requestStream(Payload payload) {
        return fluxSupplier.get();
    }

    public Flux<Payload> requestChannel(Publisher<Payload> payloads) {
        return Flux.from(payloads)
                .map(payload -> payload)
                .subscribeOn(Schedulers.parallel())
                .doOnError(err -> {
                    LOGGER.error("Error while processing channel: {}", (Object[]) err.getStackTrace());
                });
    }

    public Mono<Void> metadataPush(Payload payload) {
        return null;
    }

    public Mono<Void> onClose() {
        return Mono.empty();
    }

    public void dispose() {

    }
}
