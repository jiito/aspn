FROM rust:latest
    MAINTAINER Benjamin Allan-Rahill <benjamin.allanrahill@gmail.com>
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
EXPOSE 8080

CMD ["./target/release/hello-world"]
