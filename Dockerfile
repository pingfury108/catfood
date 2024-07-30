FROM rust:1.80.0-bullseye as builder
WORKDIR /app
COPY . ./

RUN cargo build --release

FROM debian:bullseye-slim
RUN sed -i -e 's/deb.debian.org/mirrors.aliyun.com/' -e 's/security.debian.org/mirrors.aliyun.com/'  /etc/apt/sources.list\
    && apt update && apt install -y libpq-dev
COPY --from=builder /app/target/release/catfood /app/

CMD [ "/app/catfood" ]
