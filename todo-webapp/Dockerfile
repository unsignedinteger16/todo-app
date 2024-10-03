FROM rust:latest

WORKDIR /usr/src/todo-webapp
COPY . .
RUN cargo install --path .
EXPOSE 8000

ENV ROCKET_ADDRESS=0.0.0.0
CMD ["todo-webapp"]
