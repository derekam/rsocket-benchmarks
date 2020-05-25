package io.benchmarker.rsocket;

import io.rsocket.Payload;
import io.rsocket.util.DefaultPayload;
import lombok.RequiredArgsConstructor;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.Iterator;

@RequiredArgsConstructor
public class RingStream implements Iterable<Payload> {

    private final long count;
    private final String filename;

    @Override
    public Iterator<Payload> iterator() {
        return new RingPayload(count, createPayload(filename));
    }

    private Payload createPayload(String filename) {
        File file = new File(filename);
        try {
            byte[] data = Files.readAllBytes(file.toPath());
            return DefaultPayload.create(data);
        } catch (IOException serEx) {
            throw new RuntimeException(serEx);
        }
    }

    @RequiredArgsConstructor
    private static class RingPayload implements Iterator<Payload> {

        private final long count;
        private final Payload payload;

        int counter = 0;

        @Override
        public boolean hasNext() {
            return counter < count;
        }

        @Override
        public Payload next() {
            counter++;
            return payload;
        }

    }
}
