FROM rust:bookworm AS builder

WORKDIR /build

# install necessary build dependencies
RUN cargo install dioxus-cli --locked
RUN apt update && apt install -y clang

# copy project files and compile
COPY . .
RUN dx bundle --web --release

FROM debian:bookworm-slim AS runner

# setup app
WORKDIR /app

COPY --from=builder /build/target/dx/serverspot/release/web/. .
CMD ["/app/server"]