FROM rust:1.73-buster
WORKDIR /app
RUN cargo install diesel_cli --no-default-features --features postgres
COPY . .
# Note: we run migrations and start the server via docker-compose command
EXPOSE 8080
