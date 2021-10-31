FROM rust:1.54

# ローカルのtargetディレクトリにビルドするとマウントしている時に遅くなるのでビルドディレクトリを変える
ENV CARGO_TARGET_DIR=/tmp/target \
    DEBIAN_FRONTEND=noninteractive \
    LC_CTYPE=ja_JP.utf8 \
    LANG=ja_JP.utf8

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y default-mysql-client

RUN cargo install diesel_cli --no-default-features --features mysql

RUN USER=root cargo new --bin app
WORKDIR /app
COPY ./Cargo.* ./
RUN cargo build --color never 

COPY . /app
RUN cargo build

CMD ["cargo", "run"]