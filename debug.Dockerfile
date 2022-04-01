ARG PORT=8080
ARG MAX_LEN=2000
ARG RUST_LOG=trace

FROM docker.io/rust:latest AS build

WORKDIR /app

# CACHE DOWNLOADS
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# COMPILATION
COPY src ./src
RUN cargo build

FROM gcr.io/distroless/cc:latest

COPY --from=build /app/target/debug/api /

# ENV VARS
ARG PORT
ARG MAX_LEN
ARG RUST_LOG

ENV PORT $PORT
ENV MAX_LEN $MAX_LEN
ENV RUST_LOG $RUST_LOG

# PRE-RUN
EXPOSE $PORT
USER nonroot:nonroot

# RUN
CMD ["./api"]
