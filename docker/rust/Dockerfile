FROM rust:1.54

WORKDIR /Processing_speed_measurement_by_Rust

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y default-mysql-client

RUN cargo install diesel_cli --no-default-features --features mysql

CMD bash -c "diesel setup && Processing_speed_measurement_by_Rust"