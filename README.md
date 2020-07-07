# RSocket Benchmarks (Under Construction)
Benchmarking RSocket libraries in various languages, then the winner (RSocket-Rust) against a Websocket library in the same language.

## TODO (this project is not yet complete):
 - Add Rust Websocket benchmarking
 - Resolve intermittent Go issues (involves fixes for rsocket-go)
 - Use something like Redis or SQLite instead of dumping to files
 - Results analysis and visualization

Benchmarks
---

    For each benchmark, 
        for each of Payload in the resources folder,
            send 1M of them, (maybe less for large payloads?)
            making a histogram for individual payloads (latency)
            and also getting total time and average payloads per second. (throughput)
            
    FUTURE: also add concurrency -- 32 - 512 in parallel are enough.
    FUTURE: also profile memory usage and CPU usage.
    
    OUTPUT FORMAT:
    File in root of folder named $LANG_results.csv where for each trial,
        first line is CSV of individual payload times in ns
        second line is total time in seconds
        third line is average payloads/second.
    
#### Request-Response: Request single response.
- client sends to server
- server echoes without modification.
- HTTP/2 equivalent: POST
- Websocket equivalent: None

#### Fire And Forget: A single one-way message.
- client sends to server and doesn't check anything.
- HTTP/2 equivalent: None
- Websocket equivalent: None

#### Request Stream: Request a completable stream.
- client sends a request
- server sends back 1M payloads
- HTTP/2 equivalent: streaming response
- Websocket equivalent: connection with stream of events from server

#### Request Channel: Request a completable stream in both directions.
- client sends a stream of 1M payloads
- server echoes back each one
- HTTP/2 equivalent: streaming upload with streaming response? Not really.
- Websocket equivalent: socket with req/resps.

