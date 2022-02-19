ARG PORT=8080

FROM docker.io/rust:latest AS build-env

WORKDIR /app

COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=build-env /app/target/release/api /

ARG PORT
ENV PORT $PORT
EXPOSE $PORT

CMD ["./api"]
