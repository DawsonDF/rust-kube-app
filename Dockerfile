FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:22.04
WORKDIR /app
COPY --from=builder /app/target/release/rust-kube-app /app/rust-kube-app
EXPOSE 8080
CMD ["./rust-kube-app"]
