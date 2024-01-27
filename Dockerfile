FROM rust:1.71.1 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=builder /app/target/release/mintdb-server /

EXPOSE 3000

ENTRYPOINT [ "./mintdb-server" ]