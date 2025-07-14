FROM docker.io/library/rust:1.81-slim-bookworm as builder
WORKDIR /app
COPY . .
RUN cargo build --release && strip target/release/rinha

FROM docker.io/library/debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rinha /usr/local/bin/
EXPOSE 9999
CMD ["rinha"] 