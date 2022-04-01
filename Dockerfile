ARG PORT=8080
ARG MAX_LEN=2000
ARG RUST_LOG=info,warn,error

FROM docker.io/rust:latest AS build

WORKDIR /app

# CACHE DOWNLOADS
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# COMPILATION
COPY src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc:latest

COPY --from=build /app/target/release/api /
# Public or other importants Copy
# Copy anything else


# ENV VARS
ARG PORT
ARG MAX_LEN
ARG RUST_LOG

ENV MAX_LEN $MAX_LEN
ENV PORT $PORT
ENV RUST_LOG $RUST_LOG

# PRE-RUN
EXPOSE $PORT
USER nonroot

# RUN
CMD ["./api"]
