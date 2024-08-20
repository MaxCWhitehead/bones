RUST_LOG=bones_framework=DEBUG,iroh_net=DEBUG,bones_matchmaker=DEBUG,iroh_quinn_udp=DEBUG RUSTFLAGS="--cfg tokio_unstable" TOKIO_CONSOLE_BIND=localhost:6667 cargo run &> log.txt
