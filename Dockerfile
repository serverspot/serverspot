FROM rust:bookworm AS builder

WORKDIR /build

# install necessary build dependencies
RUN cargo install dioxus-cli --locked
RUN apt update && apt install -y clang

# copy project files and compile
COPY . .
RUN dx bundle --fullstack --release

FROM debian:bookworm-slim AS runner

ENV IP="0.0.0.0"
ENV PORT="8080"

# setup app
WORKDIR /app

COPY --from=builder /build/target/dx/serverspot/release/web/. .

EXPOSE 8080
CMD ["/app/server"]