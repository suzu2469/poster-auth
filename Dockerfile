FROM rust:1.38-stretch

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run"]
