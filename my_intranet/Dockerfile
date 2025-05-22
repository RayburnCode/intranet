FROM rust:latest as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/my-intranet /usr/local/bin/

CMD ["my-intranet"]