FROM rustlang/rust:nightly as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/nft-passkey-dapp /app/
COPY --from=builder /app/static /app/static

EXPOSE 3030

CMD ["./nft-passkey-dapp"]