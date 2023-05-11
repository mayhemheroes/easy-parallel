# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive
## Install build dependencies.
RUN apt-get update 
RUN apt-get install -y cmake clang
RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /easy-parallel/

WORKDIR /easy-parallel/fuzz/

RUN cargo +nightly fuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly

## TODO: Change <Path in Builder Stage>

COPY --from=builder /easy-parallel/fuzz/target/x86_64-unknown-linux-gnu/release/squares /

COPY --from=builder /easy-parallel/fuzz/target/x86_64-unknown-linux-gnu/release/finish /

COPY --from=builder /easy-parallel/fuzz/target/x86_64-unknown-linux-gnu/release/smoke /

COPY --from=builder /easy-parallel/fuzz/target/x86_64-unknown-linux-gnu/release/parsum /

COPY --from=builder /easy-parallel/fuzz/target/x86_64-unknown-linux-gnu/release/collect /