ARG PORT=8080
ARG MAX_VALUE_LEN=2000

FROM docker.io/rust:latest AS build

WORKDIR /app

# CACHE DOWNLOADS
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# COMPILATION
COPY . .
RUN cargo build

FROM gcr.io/distroless/cc:latest

COPY --from=build /app/target/release/api /

ARG MAX_VALUE_LEN
ENV MAX_VALUE_LEN $MAX_VALUE_LEN

ARG PORT
ENV PORT $PORT
EXPOSE $PORT

CMD ["./api"]
