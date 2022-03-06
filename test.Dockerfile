ARG PORT=8080
ARG MAX_VALUE_LEN=2000

FROM docker.io/rust:latest AS build

WORKDIR /app

# CACHE DOWNLOADS
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# COMPILATION
COPY src ./src
CMD ["cargo", "test"]