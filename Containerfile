FROM rust:latest AS builder
# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN set -eux && sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories && apk add --no-cache musl-dev && rm /var/cache/apk/*
# set the workdir and copy the source into it
WORKDIR /app
COPY ./src-axum /app
# do a release build
RUN cargo build --release
# RUN strip 
RUN strip target/release/src-axum


FROM alpine:latest AS release
RUN set -eux && sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories \
    && apk update \
    && apk add --no-cache libgcc \
    && rm /var/cache/apk/*
WORKDIR /app
COPY --from=builder /app/target/release/src-axum .
# set the binary as entrypoint
ENTRYPOINT ["./src-axum"]