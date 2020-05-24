FROM debian:bullseye-slim
WORKDIR /app
ADD target/release/hello-actix .
CMD ["/app/hello-actix"]
