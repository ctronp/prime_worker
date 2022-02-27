ARG PORT=8080

FROM docker.io/rust:latest AS build

WORKDIR /app

# CACHE DOWNLOADS
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch

# COMPILATION
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/static:latest

COPY --from=build /app/target/release/api /

ARG PORT
ENV PORT $PORT
EXPOSE $PORT

CMD ["./api"]
