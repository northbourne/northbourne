FROM rust:latest

WORKDIR /usr/src/northbourne

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD ["/usr/local/cargo/bin/northbourne -c   "]