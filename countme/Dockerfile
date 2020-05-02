FROM rust

WORKDIR /usr/src/webserver
COPY . .

RUN cargo install --path .

CMD ["webserver"]