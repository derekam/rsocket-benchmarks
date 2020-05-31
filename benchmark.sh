# TODO

cd java
gradle build
java -jar build/libs/java-all.jar

cd ../go
go build -o /tmp/___go_build_io_benchmarker_rsocket_main io.benchmarker.rsocket/main
/tmp/___go_build_io_benchmarker_rsocket_main

cd ../rust
cargo run --release