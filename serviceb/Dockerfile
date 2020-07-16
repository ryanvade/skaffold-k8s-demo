FROM rust:1.44-alpine

WORKDIR /usr/src

COPY . .
RUN apk add --no-cache musl-dev
RUN cargo build --release

EXPOSE 8088
USER 1000
CMD ["target/release/serviceb"]
