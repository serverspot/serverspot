FROM rust:bookworm AS builder

# A comma-delimited list of features. By default, includes everything.
# TODO probably turn into a bunch separate bool args.
# set the value with the --build-arg flag in docker build or by using compose
ARG FEATURES="store,forum"

WORKDIR /build

# install necessary build dependencies
RUN cargo install dioxus-cli --locked
RUN apt update && apt install -y clang

# copy project files and compile
COPY . .
RUN dx bundle --web --features ${FEATURES} --release

FROM debian:bookworm-slim AS runner

ENV SURREAL_PATH="/data"
ENV SURREAL_NS="serverspot-prod"

# setup app
WORKDIR /app

COPY --from=builder /build/target/dx/serverspot/release/web/. .
CMD ["/app/server"]