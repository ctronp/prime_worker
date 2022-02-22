ARG PORT=8080

FROM docker.io/rust:latest AS build

WORKDIR /app

COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc:latest

COPY --from=build /app/target/release/api /

ARG PORT
ENV PORT $PORT
EXPOSE $PORT

CMD ["./api"]
