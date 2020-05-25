package io.benchmarker.rsocket;

import reactor.core.publisher.Flux;

import java.io.*;
import java.util.Arrays;
import java.util.concurrent.atomic.AtomicInteger;

public class Run {

    public static void main(String[] args) throws Exception {
        File fout = new File("java/java_results.csv");
        FileOutputStream fos = new FileOutputStream(fout);

        BufferedWriter statsFile = new BufferedWriter(new OutputStreamWriter(fos));
        long[] counts = { 10_000_000, 1_000_000, 100_000, 10_000, 1_000, 100 };

        final Server server = new Server();
        BenchmarkClient client = new BenchmarkClient(statsFile);

        File[] files = new File("resources").listFiles();
        AtomicInteger index = new AtomicInteger();

        Arrays.asList(files).stream()
                .map(File::getAbsolutePath)
                .sorted()
                .forEachOrdered(path -> {
                        int count = index.getAndIncrement();
                        server.setSupplier(() -> Flux.fromIterable(new RingStream(counts[count], path)));
                        try {
                            client.runBenchmarks(counts[count], path);
                        } catch (Exception e) {
                            e.printStackTrace();
                        }
                });

        statsFile.close();
    }

}
