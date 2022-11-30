FROM rust:latest
    MAINTAINER Benjamin Allan-Rahill <benjamin.allanrahill@gmail.com>
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

CMD ["hello-world"]
