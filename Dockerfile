FROM rust:latest

WORKDIR /hotel-management-server
COPY . .
RUN cargo install --path .

CMD ["hotel-management-server"]

