FROM rust:1.47-buster as builder

ADD . /src
WORKDIR /src

RUN apt-get update && \
    apt-get install -y \
        gcc \
        clang \
        llvm \
        libssl-dev \
        ffmpeg \
        libavcodec-dev \
        libavformat-dev \
        libavutil-dev \
        libavdevice-dev \
        libavfilter-dev \
        libavresample-dev \
        libpostproc-dev \
        libswscale-dev \
        libclang-7-dev \
        && \
    cargo build --verbose --release && \
    cargo install --path .

FROM debian:buster
COPY --from=builder /usr/local/cargo/bin/timed_text_dump_worker /usr/bin

RUN apt update && \
    apt install -y \
        libssl1.1 \
        ca-certificates \
        ffmpeg

ENV AMQP_QUEUE job_timed_text_dump
CMD timed_text_dump_worker
