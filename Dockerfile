FROM rust:latest as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder /app/target/release/mintdb-server /

ENTRYPOINT [ "mintdb-server" ]