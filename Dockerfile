FROM rust:bookworm AS builder

WORKDIR /build

# install necessary build dependencies
RUN cargo install dioxus-cli --locked
RUN apt update && apt install -y clang

# copy project files and compile
COPY . .
RUN dx bundle --fullstack --release

FROM debian:bookworm-slim AS runner

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

ENV IP="0.0.0.0"
ENV PORT="8080"

WORKDIR /app

COPY --from=builder /build/target/dx/serverspot/release/web/. .

EXPOSE 8080
HEALTHCHECK --interval=30s --timeout=5s --start-period=40s --retries=5 \
    CMD curl -fsS http://127.0.0.1:8080/ >/dev/null || exit 1
CMD ["/app/server"]
