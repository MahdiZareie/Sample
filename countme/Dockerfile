FROM rust as builder

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM frolvlad/alpine-glibc

COPY --from=builder /app/target/release/countme /countme

ENTRYPOINT ["/countme"]