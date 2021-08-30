FROM rust:latest AS builder

RUN USER=root cargo new --bin template_rust_server
WORKDIR ./template_rust_server
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release \
    && rm src/*.rs target/release/deps/template_rust_server*

ADD . ./

RUN cargo build --release


FROM debian:buster-slim

ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /template_rust_server/target/release/template_rust_server ${APP}/template_rust_server

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./template_rust_server"]

