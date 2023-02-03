FROM rust:1-bullseye as builder

WORKDIR /usr/kplc-parser-server
COPY . .
RUN cargo build --release

FROM debian:bullseye

WORKDIR /usr/bin
RUN apt-get update
RUN apt-get install -y openssl ca-certificates poppler-utils
ENV PORT 8080
copy --from=builder /usr/kplc-parser-server/target/release/kplc-outage-server /usr/bin

CMD ["/usr/bin/kplc-outage-server"]
