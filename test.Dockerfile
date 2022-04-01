ARG PORT=80
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

# ENV-VARS
ARG PORT
ARG MAX_VALUE_LEN
ARG RUST_LOG

ENV PORT $PORT
ENV MAX_LEN $MAX_LEN
ENV RUST_LOG $RUST_LOG

# PRE-RUN
USER nonroot

# RUN
CMD ["cargo", "test"]